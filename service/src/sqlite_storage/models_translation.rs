use super::models::{
    DieselAuthInfo, DieselGame, DieselGameBoard, DieselGameDrawWall, DieselGameHand,
    DieselGamePlayer, DieselGameScore, DieselGameSettings, DieselPlayer,
};
use super::schema;
use crate::auth::AuthInfo;
use diesel::prelude::*;
use diesel::SqliteConnection;
use mahjong_core::{
    Board, DrawWall, Game, GameId, Hand, HandTile, Hands, PlayerId, Round, RoundTileClaimed, Score,
    TileId,
};
use schema::player::dsl as player_dsl;
use service_contracts::{GameSettings, ServiceGame, ServicePlayer};
use std::collections::HashMap;

pub fn wait_common() {
    std::thread::sleep(std::time::Duration::from_millis(1));
}

impl DieselAuthInfo {
    pub fn into_raw(self) -> AuthInfo {
        AuthInfo {
            hashed_pass: self.hashed_pass,
            role: serde_json::from_str(&self.role).unwrap(),
            user_id: self.user_id,
            username: self.username,
        }
    }

    pub fn from_raw(raw: &AuthInfo) -> Self {
        Self {
            hashed_pass: raw.hashed_pass.clone(),
            role: serde_json::to_string(&raw.role).unwrap(),
            user_id: raw.user_id.clone(),
            username: raw.username.clone(),
        }
    }
}

impl DieselPlayer {
    pub fn into_raw(self) -> ServicePlayer {
        ServicePlayer {
            id: self.id,
            is_ai: self.is_ai == 1,
            name: self.name,
        }
    }

    pub fn from_raw(raw: &ServicePlayer) -> Self {
        Self {
            id: raw.id.clone(),
            is_ai: if raw.is_ai { 1 } else { 0 },
            name: raw.name.clone(),
        }
    }

    pub fn read_from_ids(
        connection: &mut SqliteConnection,
        ids: &Vec<PlayerId>,
    ) -> HashMap<PlayerId, ServicePlayer> {
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
        .collect::<HashMap<PlayerId, ServicePlayer>>()
    }

    pub fn update_from_game(connection: &mut SqliteConnection, service_game: &ServiceGame) {
        use super::schema::player::table as player_table;

        let players = service_game
            .players
            .values()
            .map(Self::from_raw)
            .collect::<Vec<Self>>();

        loop {
            if diesel::delete(player_table)
                .filter(
                    schema::player::dsl::id.eq_any(
                        players
                            .iter()
                            .map(|player| player.id.clone())
                            .collect::<Vec<PlayerId>>(),
                    ),
                )
                .execute(connection)
                .is_ok()
            {
                break;
            }
            wait_common();
        }

        loop {
            if diesel::insert_into(player_table)
                .values(&players)
                .execute(connection)
                .is_ok()
            {
                break;
            }
            wait_common();
        }
    }

    pub fn save(connection: &mut SqliteConnection, player: &ServicePlayer) {
        use super::schema::player::table as player_table;

        let player = Self::from_raw(player);

        loop {
            if diesel::delete(player_table)
                .filter(schema::player::dsl::id.eq(player.id.clone()))
                .execute(connection)
                .is_ok()
            {
                break;
            }
            wait_common();
        }

        loop {
            if diesel::insert_into(player_table)
                .values(&player)
                .execute(connection)
                .is_ok()
            {
                break;
            }
            wait_common();
        }
    }

    pub fn read_from_id(
        connection: &mut SqliteConnection,
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
        .get(0)
        .map(|player| player.clone().into_raw())
    }
}

impl DieselGame {
    pub fn into_raw(self) -> Game {
        let round = Round {
            dealer_player_index: self.round_dealer_index as usize,
            player_index: self.round_player_index as usize,
            tile_claimed: self.round_claimed_id.map(|id| RoundTileClaimed {
                by: self.round_claimed_by,
                from: self.round_claimed_from.unwrap(),
                id: id as TileId,
            }),
            wall_tile_drawn: self.round_wall_tile_drawn.map(|tile_id| tile_id as TileId),
            wind: serde_json::from_str(&self.round_wind).unwrap(),
        };
        Game {
            name: self.name,
            version: self.version,
            id: self.id,
            phase: serde_json::from_str(&self.phase).unwrap(),
            round,
            // For now the deck is not persisted
            ..Game::default()
        }
    }

    pub fn read_from_id(connection: &mut SqliteConnection, game_id: &GameId) -> Option<Game> {
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
        .get(0)
        .map(|game| game.clone().into_raw())
    }

    pub fn read_ids(connection: &mut SqliteConnection) -> Vec<GameId> {
        use schema::game::dsl as game_dsl;

        loop {
            if let Ok(game) = game_dsl::game.load::<Self>(connection) {
                break game;
            }
            wait_common();
        }
        .into_iter()
        .map(|game| game.id)
        .collect()
    }

    pub fn update(&self, connection: &mut SqliteConnection) {
        use super::schema::game::table as game_table;

        loop {
            if diesel::delete(game_table)
                .filter(schema::game::dsl::id.eq(&self.id))
                .execute(connection)
                .is_ok()
            {
                break;
            }
            wait_common();
        }

        loop {
            if diesel::insert_into(game_table)
                .values(self)
                .execute(connection)
                .is_ok()
            {
                break;
            }
            wait_common();
        }
    }

    pub fn from_raw(raw: &Game) -> Self {
        Self {
            id: raw.id.clone(),
            name: raw.name.clone(),
            phase: serde_json::to_string(&raw.phase).unwrap(),
            round_claimed_by: raw.round.tile_claimed.clone().and_then(|t| t.by),
            round_claimed_from: raw.round.tile_claimed.clone().map(|t| t.from),
            round_claimed_id: raw.round.tile_claimed.clone().map(|t| t.id as i32),
            round_dealer_index: raw.round.dealer_player_index as i32,
            round_player_index: raw.round.player_index as i32,
            round_wall_tile_drawn: raw.round.wall_tile_drawn.map(|tile_id| tile_id as i32),
            round_wind: serde_json::to_string(&raw.round.wind).unwrap(),
            version: raw.version.clone(),
        }
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

    pub fn read_from_game(connection: &mut SqliteConnection, game_id: &GameId) -> Vec<PlayerId> {
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
        connection: &mut SqliteConnection,
        player_id: &PlayerId,
    ) -> Vec<GameId> {
        use schema::game_player::dsl as game_player_dsl;

        loop {
            if let Ok(data) = game_player_dsl::game_player
                .filter(game_player_dsl::player_id.eq(player_id))
                .order(game_player_dsl::player_index.asc())
                .load::<Self>(connection)
            {
                break data;
            }
            wait_common();
        }
        .into_iter()
        .map(|game| game.game_id)
        .collect::<Vec<GameId>>()
    }

    pub fn update(connection: &mut SqliteConnection, diesel_game_players: &Vec<Self>, game: &Game) {
        use schema::game_player::table as game_player_table;

        loop {
            if diesel::delete(game_player_table)
                .filter(schema::game_player::dsl::game_id.eq(&game.id))
                .execute(connection)
                .is_ok()
            {
                break;
            }

            wait_common();
        }

        loop {
            if diesel::insert_into(game_player_table)
                .values(diesel_game_players)
                .execute(connection)
                .is_ok()
            {
                break;
            }
            wait_common();
        }
    }
}

impl DieselGameScore {
    pub fn update_from_game(connection: &mut SqliteConnection, service_game: &ServiceGame) {
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

    pub fn read_from_game(connection: &mut SqliteConnection, game_id: &GameId) -> Score {
        use schema::game_score::dsl as game_score_dsl;

        loop {
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
        .collect::<Score>()
    }
}

impl DieselGameBoard {
    pub fn update_from_game(connection: &mut SqliteConnection, service_game: &ServiceGame) {
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
            .iter()
            .enumerate()
            .map(|(tile_index, tile_id)| Self {
                game_id: service_game.game.id.clone(),
                tile_id: *tile_id as i32,
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

    pub fn read_from_game(connection: &mut SqliteConnection, game_id: &GameId) -> Board {
        use schema::game_board::dsl as game_board_dsl;

        loop {
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
        .collect::<Board>()
    }
}

impl DieselGameDrawWall {
    pub fn update_from_game(connection: &mut SqliteConnection, service_game: &ServiceGame) {
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

        let draw_wall = service_game
            .game
            .table
            .draw_wall
            .iter()
            .enumerate()
            .map(|(tile_index, tile_id)| Self {
                game_id: service_game.game.id.clone(),
                tile_id: *tile_id as i32,
                tile_index: tile_index as i32,
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

    pub fn read_from_game(connection: &mut SqliteConnection, game_id: &GameId) -> DrawWall {
        use schema::game_draw_wall::dsl as game_draw_wall_dsl;

        loop {
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
        .map(|game_draw_wall| game_draw_wall.tile_id as TileId)
        .collect::<DrawWall>()
    }
}

impl DieselGameHand {
    pub fn update_from_game(connection: &mut SqliteConnection, service_game: &ServiceGame) {
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
            .iter()
            .for_each(|(player_id, hand)| {
                hand.0.iter().enumerate().for_each(|(tile_index, tile)| {
                    let tile_id = tile.id as i32;
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

    pub fn read_from_game(connection: &mut SqliteConnection, game_id: &GameId) -> Hands {
        use schema::game_hand::dsl as game_hand_dsl;
        let mut hands = Hands::new();

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
            let player_id = game_hand.player_id;
            let concealed = game_hand.concealed == 1;
            let set_id = game_hand.set_id;
            let mut current_hand = hands.get(&player_id).unwrap_or(&Hand(Vec::new())).clone();
            current_hand.0.push(HandTile {
                id: tile_id,
                concealed,
                set_id,
            });

            hands.insert(player_id, current_hand);
        });

        hands
    }
}

impl DieselGameSettings {
    pub fn update_from_game(connection: &mut SqliteConnection, service_game: &ServiceGame) {
        use schema::game_settings::table as game_settings_table;

        loop {
            if diesel::delete(game_settings_table)
                .filter(schema::game_settings::dsl::game_id.eq(&service_game.game.id))
                .execute(connection)
                .is_ok()
            {
                break;
            }
            wait_common();
        }

        let settings = Self {
            last_discard_time: service_game.settings.last_discard_time as i64,
            ai_enabled: if service_game.settings.ai_enabled {
                1
            } else {
                0
            },
            discard_wait_ms: service_game.settings.discard_wait_ms.map(|x| x as i32),
            game_id: service_game.game.id.clone(),
            fixed_settings: if service_game.settings.fixed_settings {
                1
            } else {
                0
            },
        };

        loop {
            if diesel::insert_into(game_settings_table)
                .values(&settings)
                .execute(connection)
                .is_ok()
            {
                break;
            }
            wait_common();
        }
    }

    pub fn read_from_game(
        connection: &mut SqliteConnection,
        game_id: &GameId,
    ) -> Option<GameSettings> {
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
        .get(0)
        .map(|game_settings| GameSettings {
            ai_enabled: game_settings.ai_enabled == 1,
            discard_wait_ms: game_settings.discard_wait_ms.map(|x| x as u32),
            fixed_settings: game_settings.fixed_settings == 1,
            last_discard_time: game_settings.last_discard_time as u128,
        })
    }
}
