use super::models::{
    DieselAuthInfo, DieselAuthInfoAnonymous, DieselAuthInfoEmail, DieselAuthInfoGithub, DieselGame,
    DieselGameBoard, DieselGameDrawWall, DieselGameHand, DieselGamePlayer, DieselGameScore,
    DieselGameSettings, DieselPlayer,
};
use super::schema;
use crate::auth::{AuthInfo, AuthInfoAnonymous, AuthInfoData, AuthInfoEmail, AuthInfoGithub};
use diesel::prelude::*;
use diesel::PgConnection;
use mahjong_core::deck::DEFAULT_DECK;
use mahjong_core::{
    game::GameStyle,
    round::{Round, RoundTileClaimed},
};
use mahjong_core::{
    Board, BonusTiles, DrawWall, DrawWallPlace, Game, GameId, Hand, HandTile, Hands, PlayerId,
    Score, ScoreMap, TileId,
};
use rustc_hash::FxHashMap;
use schema::player::dsl as player_dsl;
use service_contracts::{
    AuthProvider, GameSettings, ServiceGame, ServicePlayer, ServicePlayerGame,
};
use std::str::FromStr;
use tracing::debug;

pub fn wait_common() {
    std::thread::sleep(std::time::Duration::from_millis(1));
}

fn db_request<A, B, C>(mut func: A)
where
    A: FnMut() -> Result<B, C>,
    B: std::fmt::Debug,
    C: std::fmt::Debug,
{
    loop {
        let result = func();

        if result.is_ok() {
            break;
        }
        debug!("DB request failed: {:?}", result);
        wait_common();
    }
}

impl DieselAuthInfo {
    pub fn into_raw(self, data: &AuthInfoData) -> AuthInfo {
        AuthInfo {
            data: data.clone(),
            role: serde_json::from_str(&self.role).unwrap(),
            user_id: self.user_id,
        }
    }

    pub fn into_raw_get_data(self, connection: &mut PgConnection) -> AuthInfo {
        let data = match self.provider.as_str() {
            val if val == AuthProvider::Email.to_string() => {
                let data = DieselAuthInfoEmail::get_by_id(connection, &self.user_id)
                    .expect("User not found");

                AuthInfoData::Email(data.into_raw())
            }
            val if val == AuthProvider::Github.to_string() => {
                let data = DieselAuthInfoGithub::get_by_id(connection, &self.user_id)
                    .expect("Github not found");

                AuthInfoData::Github(data.into_raw())
            }
            val if val == AuthProvider::Anonymous.to_string() => {
                let data = DieselAuthInfoAnonymous::get_by_id(connection, &self.user_id)
                    .expect("Anonymous not found");

                AuthInfoData::Anonymous(data.into_raw())
            }
            _ => panic!("Unknown provider"),
        };

        AuthInfo {
            data,
            role: serde_json::from_str(&self.role).unwrap(),
            user_id: self.user_id,
        }
    }

    pub fn from_raw(raw: &AuthInfo) -> Self {
        Self {
            provider: match raw.data {
                AuthInfoData::Email(_) => AuthProvider::Email.to_string(),
                AuthInfoData::Github(_) => AuthProvider::Github.to_string(),
                AuthInfoData::Anonymous(_) => AuthProvider::Anonymous.to_string(),
            },
            role: serde_json::to_string(&raw.role).unwrap(),
            user_id: raw.user_id.clone(),
        }
    }

    pub fn get_info_by_id(
        connection: &mut PgConnection,
        id: &String,
    ) -> Result<Option<AuthInfo>, String> {
        use schema::auth_info::dsl;

        let auth_info = dsl::auth_info
            .filter(dsl::user_id.eq(&id))
            .limit(1)
            .load::<Self>(connection)
            .unwrap()
            .first()
            .map(|auth_info| auth_info.clone().into_raw_get_data(connection));

        Ok(auth_info)
    }

    pub fn get_by_id_with_data(
        connection: &mut PgConnection,
        id: &String,
        data: &AuthInfoData,
    ) -> Option<AuthInfo> {
        use super::schema::auth_info::dsl;

        let auth_info = loop {
            if let Ok(db_data) = dsl::auth_info
                .filter(dsl::user_id.eq(id))
                .limit(1)
                .load::<Self>(connection)
            {
                break db_data;
            }
            wait_common();
        }
        .first()
        .cloned();

        auth_info.map(|auth_info_content| auth_info_content.into_raw(data))
    }
}

impl DieselAuthInfoEmail {
    pub fn into_raw(self) -> AuthInfoEmail {
        AuthInfoEmail {
            hashed_pass: self.hashed_pass,
            id: self.user_id,
            username: self.username,
        }
    }

    pub fn from_raw(raw: &AuthInfoEmail) -> Self {
        Self {
            hashed_pass: raw.hashed_pass.clone(),
            user_id: raw.id.clone(),
            username: raw.username.clone(),
        }
    }

    pub fn get_by_id(connection: &mut PgConnection, id: &String) -> Option<Self> {
        use super::schema::auth_info_email::dsl;

        loop {
            if let Ok(data) = dsl::auth_info_email
                .filter(dsl::user_id.eq(id))
                .limit(1)
                .load::<Self>(connection)
            {
                break data;
            }
            wait_common();
        }
        .first()
        .cloned()
    }

    pub fn get_info_by_username(
        connection: &mut PgConnection,
        username: &String,
    ) -> Result<Option<AuthInfo>, String> {
        use schema::auth_info_email::dsl as email_dsl;

        let auth_info_email = email_dsl::auth_info_email
            .filter(email_dsl::username.eq(&username))
            .limit(1)
            .load::<Self>(connection)
            .unwrap()
            .first()
            .map(|auth_info| auth_info.clone().into_raw());

        if auth_info_email.is_none() {
            return Ok(None);
        }

        let auth_info_email = auth_info_email.unwrap();

        let auth_info = DieselAuthInfo::get_by_id_with_data(
            connection,
            &auth_info_email.id,
            &AuthInfoData::Email(auth_info_email.clone()),
        );

        Ok(auth_info)
    }
}

impl DieselAuthInfoAnonymous {
    pub fn into_raw(self) -> AuthInfoAnonymous {
        AuthInfoAnonymous {
            hashed_token: self.hashed_token,
            id: self.user_id,
        }
    }

    pub fn from_raw(raw: &AuthInfoAnonymous) -> Self {
        Self {
            hashed_token: raw.hashed_token.clone(),
            user_id: raw.id.clone(),
        }
    }

    pub fn get_by_id(connection: &mut PgConnection, id: &String) -> Option<Self> {
        use super::schema::auth_info_anonymous::dsl;

        loop {
            if let Ok(data) = dsl::auth_info_anonymous
                .filter(dsl::user_id.eq(id))
                .limit(1)
                .load::<Self>(connection)
            {
                break data;
            }
            wait_common();
        }
        .first()
        .cloned()
    }

    pub fn get_info_by_hashed_token(
        connection: &mut PgConnection,
        hashed_token: &String,
    ) -> Result<Option<AuthInfo>, String> {
        use schema::auth_info_anonymous::dsl as anonymous_dsl;

        let auth_info_email = anonymous_dsl::auth_info_anonymous
            .filter(anonymous_dsl::hashed_token.eq(&hashed_token))
            .limit(1)
            .load::<Self>(connection)
            .unwrap()
            .first()
            .map(|auth_info| auth_info.clone().into_raw());

        if auth_info_email.is_none() {
            return Ok(None);
        }

        let auth_info_anonymous = auth_info_email.unwrap();

        let auth_info = DieselAuthInfo::get_by_id_with_data(
            connection,
            &auth_info_anonymous.id,
            &AuthInfoData::Anonymous(auth_info_anonymous.clone()),
        );

        Ok(auth_info)
    }
}

impl DieselAuthInfoGithub {
    pub fn into_raw(self) -> AuthInfoGithub {
        AuthInfoGithub {
            id: self.user_id.clone(),
            token: self.token.clone(),
            username: self.username,
        }
    }

    pub fn from_raw(raw: &AuthInfoGithub) -> Self {
        Self {
            token: raw.token.clone(),
            user_id: raw.id.clone(),
            username: raw.username.clone(),
        }
    }

    pub fn get_by_id(connection: &mut PgConnection, id: &String) -> Option<Self> {
        use super::schema::auth_info_github::dsl;

        loop {
            if let Ok(data) = dsl::auth_info_github
                .filter(dsl::user_id.eq(id))
                .limit(1)
                .load::<Self>(connection)
            {
                break data;
            }
            wait_common();
        }
        .first()
        .cloned()
    }

    pub fn get_info_by_username(
        connection: &mut PgConnection,
        username: &String,
    ) -> Result<Option<AuthInfo>, String> {
        use schema::auth_info_github::dsl as github_dsl;

        let auth_info_github = github_dsl::auth_info_github
            .filter(github_dsl::username.eq(&username))
            .limit(1)
            .load::<Self>(connection)
            .unwrap()
            .first()
            .map(|auth_info| auth_info.clone().into_raw());

        if auth_info_github.is_none() {
            return Ok(None);
        }

        let auth_info_github = auth_info_github.unwrap();

        let auth_info = DieselAuthInfo::get_by_id_with_data(
            connection,
            &auth_info_github.id,
            &AuthInfoData::Github(auth_info_github.clone()),
        );

        Ok(auth_info)
    }
}

impl DieselPlayer {
    pub fn into_raw(self) -> ServicePlayer {
        ServicePlayer {
            created_at: self.created_at.timestamp_millis().to_string(),
            id: self.id,
            is_ai: self.is_ai == 1,
            name: self.name,
        }
    }

    pub fn from_raw(raw: &ServicePlayer) -> Self {
        Self {
            created_at: chrono::NaiveDateTime::from_timestamp_millis(
                raw.created_at.parse::<i64>().unwrap(),
            )
            .unwrap(),
            id: raw.id.clone(),
            is_ai: if raw.is_ai { 1 } else { 0 },
            name: raw.name.clone(),
        }
    }

    pub fn read_from_ids(
        connection: &mut PgConnection,
        ids: &Vec<PlayerId>,
    ) -> FxHashMap<PlayerId, ServicePlayer> {
        loop {
            if let Ok(data) = player_dsl::player
                .filter(player_dsl::id.eq_any(ids))
                .load::<Self>(connection)
            {
                break data;
            }
            wait_common();
        }
        .into_iter()
        .map(|player| (player.id.clone(), player.into_raw()))
        .collect::<FxHashMap<PlayerId, ServicePlayer>>()
    }

    pub fn update_from_game(connection: &mut PgConnection, service_game: &ServiceGame) {
        use super::schema::player::table as player_table;

        let players = service_game
            .players
            .values()
            .map(Self::from_raw)
            .collect::<Vec<Self>>();

        for player in players {
            loop {
                let result = diesel::insert_into(player_table)
                    .values(&player)
                    .on_conflict(player_dsl::id)
                    .do_update()
                    .set(&player)
                    .execute(connection);

                if result.is_ok() {
                    break;
                }
                debug!("Error saving player: {:?}", result.err());
                wait_common();
            }
        }
    }

    pub fn save(connection: &mut PgConnection, player: &ServicePlayer) {
        use super::schema::player::table as player_table;

        let player = Self::from_raw(player);

        loop {
            if diesel::insert_into(player_table)
                .values(&player)
                .on_conflict(player_dsl::id)
                .do_update()
                .set(&player)
                .execute(connection)
                .is_ok()
            {
                break;
            }
            wait_common();
        }
    }

    pub fn read_from_id_raw(connection: &mut PgConnection, player_id: &PlayerId) -> Option<Self> {
        loop {
            if let Ok(player) = player_dsl::player
                .filter(player_dsl::id.eq(player_id))
                .limit(1)
                .load::<Self>(connection)
            {
                break player;
            }
            wait_common();
        }
        .first()
        .cloned()
    }

    pub fn read_from_id(
        connection: &mut PgConnection,
        player_id: &PlayerId,
    ) -> Option<ServicePlayer> {
        loop {
            if let Ok(player) = player_dsl::player
                .filter(player_dsl::id.eq(player_id))
                .limit(1)
                .load::<Self>(connection)
            {
                break player;
            }
            wait_common();
        }
        .first()
        .map(|player| player.clone().into_raw())
    }
}

pub struct DieselGameExtra {
    pub game: Game,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl DieselGame {
    pub fn into_raw(self) -> DieselGameExtra {
        let default_game = Game::new(None);
        let game_style = GameStyle::from_str(&self.style);

        let round = Round {
            dealer_player_index: self.round_dealer_index as usize,
            player_index: self.round_player_index as usize,
            round_index: self.round_index as u32,
            tile_claimed: self.round_claimed_id.map(|id| RoundTileClaimed {
                by: self.round_claimed_by,
                from: self.round_claimed_from.unwrap(),
                id: id as TileId,
            }),
            wall_tile_drawn: self.round_wall_tile_drawn.map(|tile_id| tile_id as TileId),
            wind: serde_json::from_str(&self.round_wind).unwrap(),
            style: game_style.clone().unwrap(),
            consecutive_same_seats: self.round_consecutive_same_seats as usize,
            east_player_index: self.round_east_player_index as usize,
            initial_winds: self.round_initial_winds.map(|w| w as u8),
        };
        let game = Game {
            name: self.name,
            version: self.version,
            id: self.id,
            phase: serde_json::from_str(&self.phase).unwrap(),
            round,
            style: game_style.unwrap(),
            // For now the deck is not persisted
            ..default_game
        };

        DieselGameExtra {
            game,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    pub fn read_from_id(
        connection: &mut PgConnection,
        game_id: &GameId,
    ) -> Option<DieselGameExtra> {
        use schema::game::dsl as game_dsl;

        loop {
            if let Ok(game) = game_dsl::game
                .filter(game_dsl::id.eq(game_id))
                .limit(1)
                .load::<Self>(connection)
            {
                break game;
            }
            wait_common();
        }
        .first()
        .map(|game| game.clone().into_raw())
    }

    pub fn read_player_games(connection: &mut PgConnection) -> Vec<ServicePlayerGame> {
        use schema::game::dsl as game_dsl;

        loop {
            if let Ok(game) = game_dsl::game.load::<Self>(connection) {
                break game;
            }
            wait_common();
        }
        .into_iter()
        .map(|game| ServicePlayerGame {
            created_at: game.created_at.timestamp_millis().to_string(),
            id: game.id,
            updated_at: game.updated_at.timestamp_millis().to_string(),
        })
        .collect()
    }

    pub fn update(&self, connection: &mut PgConnection) {
        use super::schema::game::table as game_table;

        loop {
            if diesel::insert_into(game_table)
                .values(self)
                .on_conflict(super::schema::game::dsl::id)
                .do_update()
                .set(self)
                .execute(connection)
                .is_ok()
            {
                break;
            }
            wait_common();
        }
    }

    pub fn from_raw(extra: &DieselGameExtra) -> Self {
        let raw = &extra.game;

        Self {
            created_at: extra.created_at,
            id: raw.id.clone(),
            name: raw.name.clone(),
            phase: serde_json::to_string(&raw.phase).unwrap(),
            round_claimed_by: raw.round.tile_claimed.clone().and_then(|t| t.by),
            round_claimed_from: raw.round.tile_claimed.clone().map(|t| t.from),
            round_claimed_id: raw.round.tile_claimed.clone().map(|t| t.id),
            round_dealer_index: raw.round.dealer_player_index as i32,
            round_index: raw.round.round_index as i32,
            round_player_index: raw.round.player_index as i32,
            round_wall_tile_drawn: raw.round.wall_tile_drawn,
            round_wind: serde_json::to_string(&raw.round.wind).unwrap(),
            updated_at: extra.updated_at,
            version: raw.version.clone(),
            style: raw.style.to_string(),
            round_consecutive_same_seats: raw.round.consecutive_same_seats as i32,
            round_east_player_index: raw.round.east_player_index as i32,
            round_initial_winds: raw.round.initial_winds.map(|w| w as i32),
        }
    }

    pub fn delete_games(connection: &mut PgConnection, game_ids: &[GameId]) {
        db_request(|| {
            diesel::delete(schema::game::table)
                .filter(schema::game::dsl::id.eq_any(game_ids))
                .execute(connection)
        });
    }
}

impl DieselGamePlayer {
    pub fn from_game(game: &Game) -> Vec<Self> {
        game.players
            .iter()
            .enumerate()
            .map(|(index, player_id)| Self {
                game_id: game.id.clone(),
                player_id: player_id.clone(),
                player_index: index as i32,
            })
            .collect::<Vec<Self>>()
    }

    pub fn read_from_game(connection: &mut PgConnection, game_id: &GameId) -> Vec<PlayerId> {
        use schema::game_player::dsl as game_player_dsl;
        loop {
            if let Ok(data) = game_player_dsl::game_player
                .filter(game_player_dsl::game_id.eq(game_id))
                .order(game_player_dsl::player_index.asc())
                .load::<Self>(connection)
            {
                break data;
            }
            wait_common();
        }
        .into_iter()
        .map(|game| game.player_id)
        .collect::<Vec<PlayerId>>()
    }

    pub fn read_from_player(
        connection: &mut PgConnection,
        player_id: &PlayerId,
    ) -> Vec<ServicePlayerGame> {
        let player = DieselPlayer::read_from_id_raw(connection, player_id);

        if player.is_none() {
            return vec![];
        }

        let player = player.unwrap();

        loop {
            if let Ok(data) = Self::belonging_to(&player)
                .inner_join(super::schema::game::table)
                .select(DieselGame::as_select())
                .order(super::schema::game::dsl::updated_at.desc())
                .load(connection)
            {
                break data;
            }
            wait_common();
        }
        .into_iter()
        .map(|game| ServicePlayerGame {
            created_at: game.created_at.timestamp_millis().to_string(),
            id: game.id,
            updated_at: game.updated_at.timestamp_millis().to_string(),
        })
        .collect::<Vec<_>>()
    }

    pub fn update(connection: &mut PgConnection, diesel_game_players: &Vec<Self>, game: &Game) {
        use schema::game_player::table as game_player_table;

        db_request(|| {
            diesel::delete(game_player_table)
                .filter(schema::game_player::dsl::game_id.eq(&game.id))
                .execute(connection)
        });

        db_request(|| {
            diesel::insert_into(game_player_table)
                .values(diesel_game_players)
                .execute(connection)
        });
    }

    pub fn delete_games(connection: &mut PgConnection, game_ids: &[GameId]) {
        db_request(|| {
            diesel::delete(schema::game_player::table)
                .filter(schema::game_player::dsl::game_id.eq_any(game_ids))
                .execute(connection)
        });
    }
}

impl DieselGameScore {
    pub fn update_from_game(connection: &mut PgConnection, service_game: &ServiceGame) {
        use schema::game_score::table as game_score_table;

        loop {
            if diesel::delete(game_score_table)
                .filter(schema::game_score::dsl::game_id.eq(&service_game.game.id))
                .execute(connection)
                .is_ok()
            {
                break;
            }
            wait_common();
        }

        let scores = service_game
            .game
            .score
            .iter()
            .map(|(player_id, score)| Self {
                game_id: service_game.game.id.clone(),
                player_id: player_id.clone(),
                score: *score as i32,
            })
            .collect::<Vec<Self>>();

        loop {
            if diesel::insert_into(game_score_table)
                .values(&scores)
                .execute(connection)
                .is_ok()
            {
                break;
            }
            wait_common();
        }
    }

    pub fn read_from_game(connection: &mut PgConnection, game_id: &GameId) -> Score {
        use schema::game_score::dsl as game_score_dsl;

        let score_map = loop {
            if let Ok(data) = game_score_dsl::game_score
                .filter(game_score_dsl::game_id.eq(game_id))
                .load::<Self>(connection)
            {
                break data;
            }
            wait_common();
        }
        .into_iter()
        .map(|game_score| {
            let score = game_score.score as u32;

            (game_score.player_id, score)
        })
        .collect::<ScoreMap>();

        Score(score_map)
    }

    pub fn read_total_from_player(connection: &mut PgConnection, player_id: &PlayerId) -> i32 {
        use schema::game_score::dsl as game_score_dsl;

        loop {
            if let Ok(data) = game_score_dsl::game_score
                .filter(game_score_dsl::player_id.eq(player_id))
                .load::<Self>(connection)
            {
                break data;
            }
            wait_common();
        }
        .into_iter()
        .map(|game_score| game_score.score)
        .sum()
    }

    pub fn delete_games(connection: &mut PgConnection, game_ids: &[GameId]) {
        db_request(|| {
            diesel::delete(schema::game_score::table)
                .filter(schema::game_score::dsl::game_id.eq_any(game_ids))
                .execute(connection)
        });
    }
}

impl DieselGameBoard {
    pub fn update_from_game(connection: &mut PgConnection, service_game: &ServiceGame) {
        use schema::game_board::table as game_board_table;

        loop {
            if diesel::delete(game_board_table)
                .filter(schema::game_board::dsl::game_id.eq(&service_game.game.id))
                .execute(connection)
                .is_ok()
            {
                break;
            }
            wait_common();
        }

        let board = service_game
            .game
            .table
            .board
            .0
            .iter()
            .enumerate()
            .map(|(tile_index, tile_id)| Self {
                game_id: service_game.game.id.clone(),
                tile_id: *tile_id,
                tile_index: tile_index as i32,
            })
            .collect::<Vec<Self>>();

        loop {
            if diesel::insert_into(game_board_table)
                .values(&board)
                .execute(connection)
                .is_ok()
            {
                break;
            }
            wait_common();
        }
    }

    pub fn read_from_game(connection: &mut PgConnection, game_id: &GameId) -> Board {
        use schema::game_board::dsl as game_board_dsl;

        let board_content = loop {
            if let Ok(data) = game_board_dsl::game_board
                .filter(game_board_dsl::game_id.eq(game_id))
                .order(game_board_dsl::tile_index.asc())
                .load::<Self>(connection)
            {
                break data;
            }
            wait_common()
        }
        .into_iter()
        .map(|game_board| game_board.tile_id as TileId)
        .collect::<Vec<TileId>>();

        Board(board_content)
    }

    pub fn delete_games(connection: &mut PgConnection, game_ids: &[GameId]) {
        db_request(|| {
            diesel::delete(schema::game_board::table)
                .filter(schema::game_board::dsl::game_id.eq_any(game_ids))
                .execute(connection)
        });
    }
}

impl DieselGameDrawWall {
    pub fn update_from_game(connection: &mut PgConnection, service_game: &ServiceGame) {
        use schema::game_draw_wall::table as game_draw_wall_table;

        loop {
            if diesel::delete(game_draw_wall_table)
                .filter(schema::game_draw_wall::dsl::game_id.eq(&service_game.game.id))
                .execute(connection)
                .is_ok()
            {
                break;
            }
            wait_common();
        }

        let mut draw_wall_vec = vec![];

        let draw_wall = service_game
            .game
            .table
            .draw_wall
            .iter_all(&mut draw_wall_vec)
            .enumerate()
            .map(|(tile_index, draw_wall_tile)| Self {
                game_id: service_game.game.id.clone(),
                tile_id: draw_wall_tile.0,
                tile_index: tile_index as i32,
                place: draw_wall_tile.1.to_string(),
            })
            .collect::<Vec<Self>>();

        loop {
            if diesel::insert_into(game_draw_wall_table)
                .values(&draw_wall)
                .execute(connection)
                .is_ok()
            {
                break;
            }
            wait_common();
        }
    }

    pub fn read_from_game(connection: &mut PgConnection, game_id: &GameId) -> DrawWall {
        use schema::game_draw_wall::dsl as game_draw_wall_dsl;

        let draw_wall_content = loop {
            if let Ok(data) = game_draw_wall_dsl::game_draw_wall
                .filter(game_draw_wall_dsl::game_id.eq(game_id))
                .order(game_draw_wall_dsl::tile_index.asc())
                .load::<Self>(connection)
            {
                break data;
            }
            wait_common();
        }
        .into_iter()
        .map(|game_draw_wall| {
            (
                game_draw_wall.tile_id,
                DrawWallPlace::from_str(&game_draw_wall.place).unwrap_or_else(|_| {
                    panic!("Unknown draw wall place: {}", game_draw_wall.place)
                }),
            )
        })
        .collect::<Vec<(TileId, DrawWallPlace)>>();

        DrawWall::new_full(draw_wall_content)
    }

    pub fn delete_games(connection: &mut PgConnection, game_ids: &[GameId]) {
        db_request(|| {
            diesel::delete(schema::game_draw_wall::table)
                .filter(schema::game_draw_wall::dsl::game_id.eq_any(game_ids))
                .execute(connection)
        });
    }
}

impl DieselGameHand {
    pub fn update_from_game(connection: &mut PgConnection, service_game: &ServiceGame) {
        use schema::game_hand::table as game_hand_table;

        loop {
            if diesel::delete(game_hand_table)
                .filter(schema::game_hand::dsl::game_id.eq(&service_game.game.id))
                .execute(connection)
                .is_ok()
            {
                break;
            }
            wait_common();
        }

        let mut hands: Vec<Self> = vec![];

        service_game
            .game
            .table
            .hands
            .0
            .iter()
            .for_each(|(player_id, hand)| {
                hand.list.iter().enumerate().for_each(|(tile_index, tile)| {
                    let tile_id = tile.id;
                    let concealed = if tile.concealed { 1 } else { 0 };
                    let set_id = tile.set_id.clone();

                    let game_hand = Self {
                        concealed,
                        game_id: service_game.game.id.clone(),
                        player_id: player_id.clone(),
                        set_id,
                        tile_id,
                        tile_index: tile_index as i32,
                    };

                    hands.push(game_hand);
                });
            });

        service_game
            .game
            .table
            .bonus_tiles
            .0
            .iter()
            .for_each(|(player_id, player_bonus_tiles)| {
                player_bonus_tiles
                    .iter()
                    .enumerate()
                    .for_each(|(tile_index, tile_id)| {
                        let game_hand = Self {
                            concealed: 0,
                            game_id: service_game.game.id.clone(),
                            player_id: player_id.clone(),
                            set_id: None,
                            tile_id: *tile_id,
                            tile_index: tile_index as i32,
                        };

                        hands.push(game_hand);
                    });
            });

        loop {
            if diesel::insert_into(game_hand_table)
                .values(&hands)
                .execute(connection)
                .is_ok()
            {
                break;
            }
            wait_common();
        }
    }

    pub fn read_from_game(connection: &mut PgConnection, game_id: &GameId) -> (Hands, BonusTiles) {
        use schema::game_hand::dsl as game_hand_dsl;
        let mut hands = Hands::default();
        let mut bonus_tiles = BonusTiles::default();

        loop {
            if let Ok(data) = game_hand_dsl::game_hand
                .filter(game_hand_dsl::game_id.eq(game_id))
                .order(game_hand_dsl::tile_index.asc())
                .load::<Self>(connection)
            {
                break data;
            }
            wait_common();
        }
        .into_iter()
        .for_each(|game_hand| {
            let tile_id = game_hand.tile_id as TileId;
            if DEFAULT_DECK.get_sure(tile_id).is_bonus() {
                let player_bonus_tiles = bonus_tiles.get_or_create(&game_hand.player_id);
                player_bonus_tiles.push(tile_id);
            } else {
                let player_id = game_hand.player_id;
                let concealed = game_hand.concealed == 1;
                let set_id = game_hand.set_id;
                let mut current_hand = hands
                    .0
                    .get(&player_id)
                    .unwrap_or(&Hand::new(Vec::new()))
                    .clone();
                current_hand.push(HandTile {
                    id: tile_id,
                    concealed,
                    set_id,
                });

                hands.0.insert(player_id, current_hand);
            }
        });

        (hands, bonus_tiles)
    }

    pub fn delete_games(connection: &mut PgConnection, game_ids: &[GameId]) {
        db_request(|| {
            diesel::delete(schema::game_hand::table)
                .filter(schema::game_hand::dsl::game_id.eq_any(game_ids))
                .execute(connection)
        });
    }
}

impl DieselGameSettings {
    pub fn update_from_game(connection: &mut PgConnection, service_game: &ServiceGame) {
        use schema::game_settings::table as game_settings_table;

        let auto_sort_players = service_game
            .settings
            .auto_sort_players
            .clone()
            .into_iter()
            .collect::<Vec<_>>()
            .join(&','.to_string());

        let auto_stop_claim_meld = service_game
            .settings
            .auto_stop_claim_meld
            .clone()
            .into_iter()
            .collect::<Vec<_>>()
            .join(&','.to_string());

        let settings = Self {
            last_discard_time: service_game.settings.last_discard_time as i64,
            ai_enabled: if service_game.settings.ai_enabled {
                1
            } else {
                0
            },
            discard_wait_ms: service_game.settings.discard_wait_ms,
            game_id: service_game.game.id.clone(),
            fixed_settings: if service_game.settings.fixed_settings {
                1
            } else {
                0
            },
            auto_sort_players,
            auto_stop_claim_meld,
        };

        loop {
            if diesel::insert_into(game_settings_table)
                .values(&settings)
                .on_conflict(schema::game_settings::dsl::game_id)
                .do_update()
                .set(&settings)
                .execute(connection)
                .is_ok()
            {
                break;
            }
            wait_common();
        }
    }

    pub fn read_from_game(connection: &mut PgConnection, game_id: &GameId) -> Option<GameSettings> {
        use schema::game_settings::dsl as game_settings_dsl;

        loop {
            if let Ok(data) = game_settings_dsl::game_settings
                .filter(game_settings_dsl::game_id.eq(game_id))
                .limit(1)
                .load::<Self>(connection)
            {
                break data;
            }
            wait_common();
        }
        .first()
        .map(|game_settings| GameSettings {
            ai_enabled: game_settings.ai_enabled == 1,
            discard_wait_ms: game_settings.discard_wait_ms,
            fixed_settings: game_settings.fixed_settings == 1,
            last_discard_time: game_settings.last_discard_time as i128,
            auto_stop_claim_meld: game_settings
                .auto_stop_claim_meld
                .split(',')
                .map(|s| s.to_string())
                .collect(),
            auto_sort_players: game_settings
                .auto_sort_players
                .split(',')
                .map(|s| s.to_string())
                .collect(),
        })
    }

    pub fn delete_games(connection: &mut PgConnection, game_ids: &[GameId]) {
        db_request(|| {
            diesel::delete(schema::game_settings::table)
                .filter(schema::game_settings::dsl::game_id.eq_any(game_ids))
                .execute(connection)
        });
    }
}
