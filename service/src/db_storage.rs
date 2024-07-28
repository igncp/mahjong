use crate::{
    auth::{AuthInfo, AuthInfoData, GetAuthInfo, Username},
    common::Storage,
    db_storage::models::{
        DieselAuthInfo, DieselAuthInfoEmail, DieselAuthInfoGithub, DieselGame, DieselGamePlayer,
        DieselGameScore, DieselPlayer,
    },
    env::{ENV_PG_URL, ENV_REDIS_URL},
};
use async_trait::async_trait;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use mahjong_core::{Game, GameId, PlayerId, Players};
use redis::Commands;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use service_contracts::{ServiceGame, ServicePlayer, ServicePlayerGame};
use tracing::debug;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

use self::{
    models::{
        DieselAuthInfoAnonymous, DieselGameBoard, DieselGameDrawWall, DieselGameHand,
        DieselGameSettings,
    },
    models_translation::DieselGameExtra,
};

mod models;
mod models_translation;
mod schema;

pub struct DBStorage {
    db_path: String,
    redis_path: String,
}

#[derive(Serialize, Deserialize)]
struct FileContent {
    auth: Option<FxHashMap<Username, AuthInfo>>,
    games: Option<FxHashMap<GameId, Game>>,
    players: Option<FxHashMap<PlayerId, ServicePlayer>>,
}

#[async_trait]
impl Storage for DBStorage {
    async fn get_auth_info(&self, get_auth_info: GetAuthInfo) -> Result<Option<AuthInfo>, String> {
        let mut connection = PgConnection::establish(&self.db_path).unwrap();

        match get_auth_info {
            GetAuthInfo::EmailUsername(username) => {
                DieselAuthInfoEmail::get_info_by_username(&mut connection, &username)
            }
            GetAuthInfo::GithubUsername(username) => {
                DieselAuthInfoGithub::get_info_by_username(&mut connection, &username)
            }
            GetAuthInfo::PlayerId(player_id) => {
                DieselAuthInfo::get_info_by_id(&mut connection, &player_id)
            }
            GetAuthInfo::AnonymousToken(id_token) => {
                DieselAuthInfoAnonymous::get_info_by_hashed_token(&mut connection, &id_token)
            }
        }
    }

    async fn get_player_total_score(&self, player_id: &PlayerId) -> Result<i32, String> {
        let mut connection = PgConnection::establish(&self.db_path).unwrap();

        let total_score = DieselGameScore::read_total_from_player(&mut connection, player_id);

        Ok(total_score)
    }

    async fn save_auth_info(&self, auth_info: &AuthInfo) -> Result<(), String> {
        use schema::auth_info::table;
        use schema::auth_info_anonymous::table as anonymous_table;
        use schema::auth_info_email::table as email_table;
        use schema::auth_info_github::table as github_table;

        let mut connection = PgConnection::establish(&self.db_path).unwrap();
        let diesel_auth_info = DieselAuthInfo::from_raw(auth_info);

        diesel::insert_into(table)
            .values(&diesel_auth_info)
            .execute(&mut connection)
            .unwrap();

        match auth_info.data {
            AuthInfoData::Email(ref email) => {
                let diesel_auth_info_email = DieselAuthInfoEmail::from_raw(email);

                diesel::insert_into(email_table)
                    .values(&diesel_auth_info_email)
                    .execute(&mut connection)
                    .unwrap();
            }
            AuthInfoData::Github(ref github) => {
                let diesel_auth_info_github = DieselAuthInfoGithub::from_raw(github);

                diesel::insert_into(github_table)
                    .values(&diesel_auth_info_github)
                    .execute(&mut connection)
                    .unwrap();
            }
            AuthInfoData::Anonymous(ref anonymous) => {
                let diesel_auth_info_anonymous = DieselAuthInfoAnonymous::from_raw(anonymous);

                diesel::insert_into(anonymous_table)
                    .values(&diesel_auth_info_anonymous)
                    .execute(&mut connection)
                    .unwrap();
            }
        }

        Ok(())
    }

    async fn save_game(&self, service_game: &ServiceGame) -> Result<(), String> {
        let mut connection = PgConnection::establish(&self.db_path).unwrap();
        let redis_client = redis::Client::open(self.redis_path.clone()).unwrap();
        let mut redis_connection = redis_client.get_connection().unwrap();

        let game_str = serde_json::to_string(&service_game).unwrap();
        let redis_key = format!("game:{}", service_game.game.id);

        let _: () = redis_connection.set(redis_key.clone(), game_str).unwrap();
        let _: () = redis_connection.expire(redis_key, 60 * 60).unwrap();

        DieselPlayer::update_from_game(&mut connection, service_game);

        let diesel_game_extra = DieselGameExtra {
            created_at: chrono::DateTime::from_timestamp_millis(service_game.created_at)
                .unwrap()
                .naive_utc(),
            game: service_game.game.clone(),
            updated_at: chrono::DateTime::from_timestamp_millis(service_game.updated_at)
                .unwrap()
                .naive_utc(),
        };

        let diesel_game = DieselGame::from_raw(&diesel_game_extra);

        diesel_game.update(&mut connection);

        let diesel_game_players = DieselGamePlayer::from_game(&service_game.game);
        DieselGamePlayer::update(&mut connection, &diesel_game_players, &service_game.game);

        DieselGameScore::update_from_game(&mut connection, service_game);
        DieselGameBoard::update_from_game(&mut connection, service_game);
        DieselGameDrawWall::update_from_game(&mut connection, service_game);
        DieselGameHand::update_from_game(&mut connection, service_game);
        DieselGameSettings::update_from_game(&mut connection, service_game);

        Ok(())
    }

    async fn get_game(&self, id: &GameId, use_cache: bool) -> Result<Option<ServiceGame>, String> {
        let redis_client = redis::Client::open(self.redis_path.clone()).unwrap();
        let mut redis_connection = redis_client.get_connection().unwrap();

        let redis_key = format!("game:{}", id);

        if use_cache {
            let game_str: Option<String> = redis_connection.get(redis_key).unwrap();

            if game_str.is_some() {
                let game: ServiceGame = serde_json::from_str(&game_str.unwrap()).unwrap();

                return Ok(Some(game));
            }
        } else {
            redis_connection.del::<String, bool>(redis_key).unwrap();
        }

        let mut connection = PgConnection::establish(&self.db_path).unwrap();

        let result = DieselGame::read_from_id(&mut connection, id);

        if result.is_none() {
            return Ok(None);
        }

        let game_players = DieselGamePlayer::read_from_game(&mut connection, id);
        let players = DieselPlayer::read_from_ids(&mut connection, &game_players);

        let score = DieselGameScore::read_from_game(&mut connection, id);
        let board = DieselGameBoard::read_from_game(&mut connection, id);
        let draw_wall = DieselGameDrawWall::read_from_game(&mut connection, id);
        let (hands, bonus_tiles) = DieselGameHand::read_from_game(&mut connection, id);
        let settings = DieselGameSettings::read_from_game(&mut connection, id);

        if settings.is_none() {
            return Ok(None);
        }

        let game_extra = result.unwrap();
        let mut game = game_extra.game;
        game.players = Players(game_players);
        game.score = score;
        game.table.hands = hands;
        game.table.bonus_tiles = bonus_tiles;
        game.table.board = board;
        game.table.draw_wall = draw_wall;

        let service_game = ServiceGame {
            created_at: game_extra.created_at.and_utc().timestamp_millis(),
            game,
            players,
            settings: settings.unwrap(),
            updated_at: game_extra.updated_at.and_utc().timestamp_millis(),
        };

        Ok(Some(service_game))
    }

    async fn get_player_games(
        &self,
        player_id: &Option<PlayerId>,
    ) -> Result<Vec<ServicePlayerGame>, String> {
        let mut connection = PgConnection::establish(&self.db_path).unwrap();

        if player_id.is_some() {
            let result =
                DieselGamePlayer::read_from_player(&mut connection, &player_id.clone().unwrap());

            return Ok(result);
        }

        let all = DieselGame::read_player_games(&mut connection);

        Ok(all)
    }

    async fn get_player(&self, player_id: &PlayerId) -> Result<Option<ServicePlayer>, String> {
        let mut connection = PgConnection::establish(&self.db_path).unwrap();

        let player = DieselPlayer::read_from_id(&mut connection, player_id);

        Ok(player)
    }

    async fn save_player(&self, player: &ServicePlayer) -> Result<(), String> {
        let mut connection = PgConnection::establish(&self.db_path).unwrap();

        DieselPlayer::save(&mut connection, player);

        Ok(())
    }

    async fn delete_games(&self, ids: &[GameId]) -> Result<(), String> {
        let mut connection = PgConnection::establish(&self.db_path).unwrap();

        DieselGamePlayer::delete_games(&mut connection, ids);
        DieselGameScore::delete_games(&mut connection, ids);
        DieselGameBoard::delete_games(&mut connection, ids);
        DieselGameDrawWall::delete_games(&mut connection, ids);
        DieselGameHand::delete_games(&mut connection, ids);
        DieselGameSettings::delete_games(&mut connection, ids);
        DieselGame::delete_games(&mut connection, ids);

        Ok(())
    }
}

impl DBStorage {
    #[allow(dead_code)]
    pub fn new_dyn() -> Box<dyn Storage> {
        let db_path = std::env::var(ENV_PG_URL)
            .unwrap_or("postgres://postgres:postgres@localhost/mahjong".to_string());
        let redis_path = std::env::var(ENV_REDIS_URL).unwrap();

        debug!("DBStorage: {} {}", db_path, redis_path);

        let file_storage = Self {
            db_path: db_path.clone(),
            redis_path,
        };

        let mut connection = PgConnection::establish(&db_path).unwrap();

        connection.run_pending_migrations(MIGRATIONS).unwrap();

        Box::new(file_storage)
    }
}
