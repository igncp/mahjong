use crate::{
    auth::{AuthInfo, Username},
    common::Storage,
    env::ENV_SQLITE_DB_KEY,
    sqlite_storage::{
        models::{DieselAuthInfo, DieselGame, DieselGamePlayer, DieselGameScore, DieselPlayer},
        models_translation::wait_common,
    },
};
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use mahjong_core::{Game, GameId, PlayerId};
use serde::{Deserialize, Serialize};
use service_contracts::{ServiceGame, ServicePlayer};
use std::collections::HashMap;
use tracing::debug;

use self::models::{DieselGameBoard, DieselGameDrawWall, DieselGameHand, DieselGameSettings};

mod models;
mod models_translation;
mod schema;

pub struct SQLiteStorage {
    db_path: String,
}

#[derive(Serialize, Deserialize)]
struct FileContent {
    auth: Option<HashMap<Username, AuthInfo>>,
    games: Option<HashMap<GameId, Game>>,
    players: Option<HashMap<PlayerId, ServicePlayer>>,
}

#[async_trait]
impl Storage for SQLiteStorage {
    async fn get_auth_info(&self, username: &Username) -> Result<Option<AuthInfo>, String> {
        use schema::auth_info::dsl;
        let mut connection = SqliteConnection::establish(&self.db_path).unwrap();

        let auth_info: Option<AuthInfo> = loop {
            if let Ok(data) = dsl::auth_info
                .filter(dsl::username.eq(username))
                .limit(1)
                .load::<DieselAuthInfo>(&mut connection)
            {
                break data;
            }
            wait_common();
        }
        .get(0)
        .map(|auth_info| auth_info.clone().into_raw());

        Ok(auth_info)
    }

    async fn save_auth_info(&self, auth_info: &AuthInfo) -> Result<(), String> {
        use schema::auth_info::table;
        let mut connection = SqliteConnection::establish(&self.db_path).unwrap();
        let diesel_auth_info = DieselAuthInfo::from_raw(auth_info);

        diesel::insert_into(table)
            .values(&diesel_auth_info)
            .execute(&mut connection)
            .unwrap();

        Ok(())
    }

    async fn save_game(&self, service_game: &ServiceGame) -> Result<(), String> {
        let mut connection = SqliteConnection::establish(&self.db_path).unwrap();

        DieselPlayer::update_from_game(&mut connection, service_game);

        // This could be a transaction

        let diesel_game_players = DieselGamePlayer::from_game(&service_game.game);
        DieselGamePlayer::update(&mut connection, &diesel_game_players, &service_game.game);
        DieselGameScore::update_from_game(&mut connection, service_game);
        DieselGameBoard::update_from_game(&mut connection, service_game);
        DieselGameDrawWall::update_from_game(&mut connection, service_game);
        DieselGameHand::update_from_game(&mut connection, service_game);
        DieselGameSettings::update_from_game(&mut connection, service_game);

        let diesel_game = DieselGame::from_raw(&service_game.game);
        diesel_game.update(&mut connection);

        Ok(())
    }

    async fn get_game(&self, id: &GameId) -> Result<Option<ServiceGame>, String> {
        let mut connection = SqliteConnection::establish(&self.db_path).unwrap();

        let result = DieselGame::read_from_id(&mut connection, id);

        if result.is_none() {
            return Ok(None);
        }

        let game_players = DieselGamePlayer::read_from_game(&mut connection, id);
        let players = DieselPlayer::read_from_ids(&mut connection, &game_players);

        let score = DieselGameScore::read_from_game(&mut connection, id);
        let board = DieselGameBoard::read_from_game(&mut connection, id);
        let draw_wall = DieselGameDrawWall::read_from_game(&mut connection, id);
        let hands = DieselGameHand::read_from_game(&mut connection, id);
        let settings = DieselGameSettings::read_from_game(&mut connection, id);

        if settings.is_none() {
            return Ok(None);
        }

        let mut game = result.unwrap();
        game.set_players(&game_players);
        game.score = score;
        game.table.hands = hands;
        game.table.board = board;
        game.table.draw_wall = draw_wall;

        let service_game = ServiceGame {
            game,
            players,
            settings: settings.unwrap(),
        };

        Ok(Some(service_game))
    }

    async fn get_games_ids(&self, player_id: &Option<PlayerId>) -> Result<Vec<GameId>, String> {
        let mut connection = SqliteConnection::establish(&self.db_path).unwrap();

        if player_id.is_some() {
            let result =
                DieselGamePlayer::read_from_player(&mut connection, &player_id.clone().unwrap());

            return Ok(result);
        }

        let all = DieselGame::read_ids(&mut connection);

        Ok(all)
    }

    async fn get_player(&self, player_id: &PlayerId) -> Result<Option<ServicePlayer>, String> {
        let mut connection = SqliteConnection::establish(&self.db_path).unwrap();

        let player = DieselPlayer::read_from_id(&mut connection, player_id);

        Ok(player)
    }

    async fn save_player(&self, player: &ServicePlayer) -> Result<(), String> {
        let mut connection = SqliteConnection::establish(&self.db_path).unwrap();

        DieselPlayer::save(&mut connection, player);

        Ok(())
    }
}

impl SQLiteStorage {
    #[allow(dead_code)]
    pub fn new_dyn() -> Box<dyn Storage> {
        let db_path = std::env::var(ENV_SQLITE_DB_KEY).unwrap_or("sqlite://mahjong.db".to_string());

        debug!("SQLiteStorage: {}", db_path);

        let file_storage = Self { db_path };

        Box::new(file_storage)
    }
}
