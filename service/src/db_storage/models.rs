use super::schema::auth_info;
use super::schema::auth_info_anonymous;
use super::schema::auth_info_email;
use super::schema::auth_info_github;
use super::schema::game;
use super::schema::game_board;
use super::schema::game_draw_wall;
use super::schema::game_hand;
use super::schema::game_player;
use super::schema::game_score;
use super::schema::game_settings;
use super::schema::player;
use diesel::prelude::*;
use mahjong_core::game::GameVersion;
use mahjong_core::GameId;
use mahjong_core::PlayerId;

#[derive(Insertable, Queryable, Clone)]
#[diesel(table_name = auth_info)]
pub struct DieselAuthInfo {
    pub provider: String,
    pub role: String,
    pub user_id: PlayerId,
}

#[derive(Insertable, Queryable, Clone)]
#[diesel(table_name = auth_info_email)]
pub struct DieselAuthInfoEmail {
    pub hashed_pass: String,
    pub user_id: PlayerId,
    pub username: String,
}

#[derive(Insertable, Queryable, Clone)]
#[diesel(table_name = auth_info_anonymous)]
pub struct DieselAuthInfoAnonymous {
    pub hashed_token: String,
    pub user_id: PlayerId,
}

#[derive(Insertable, Queryable, Clone)]
#[diesel(table_name = auth_info_github)]
pub struct DieselAuthInfoGithub {
    pub token: Option<String>,
    pub user_id: PlayerId,
    pub username: String,
}

#[derive(Insertable, AsChangeset, Queryable, Selectable, Identifiable, PartialEq, Clone, Debug)]
#[diesel(treat_none_as_null = true)]
#[diesel(table_name = player)]
pub struct DieselPlayer {
    pub created_at: chrono::NaiveDateTime,
    pub id: PlayerId,
    pub is_ai: i32,
    pub name: String,
}

#[derive(Insertable, AsChangeset, Queryable, Selectable, Identifiable, PartialEq, Clone, Debug)]
#[diesel(treat_none_as_null = true)]
#[diesel(table_name = game)]
pub struct DieselGame {
    pub created_at: chrono::NaiveDateTime,
    pub id: GameId,
    pub name: String,
    pub phase: String,
    pub round_claimed_by: Option<PlayerId>,
    pub round_claimed_from: Option<PlayerId>,
    pub round_claimed_id: Option<i32>,
    pub round_dealer_index: i32,
    pub round_index: i32,
    pub round_player_index: i32,
    pub round_wall_tile_drawn: Option<i32>,
    pub round_wind: String,
    pub updated_at: chrono::NaiveDateTime,
    pub version: GameVersion,
    pub style: String,
}

#[derive(Identifiable, Insertable, Selectable, Queryable, Associations, Debug)]
#[diesel(table_name = game_player)]
#[diesel(belongs_to(DieselGame, foreign_key = game_id))]
#[diesel(belongs_to(DieselPlayer, foreign_key = player_id))]
#[diesel(primary_key(game_id, player_id))]
pub struct DieselGamePlayer {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub player_index: i32,
}

#[derive(Insertable, Queryable, Clone)]
#[diesel(table_name = game_score)]
pub struct DieselGameScore {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub score: i32,
}

#[derive(Insertable, Queryable, Clone)]
#[diesel(table_name = game_board)]
pub struct DieselGameBoard {
    pub game_id: GameId,
    pub tile_id: i32,
    pub tile_index: i32,
}

#[derive(Insertable, Queryable, Clone)]
#[diesel(table_name = game_draw_wall)]
pub struct DieselGameDrawWall {
    pub game_id: GameId,
    pub tile_id: i32,
    pub tile_index: i32,
}

#[derive(Insertable, Queryable, Clone)]
#[diesel(table_name = game_hand)]
pub struct DieselGameHand {
    pub concealed: i32,
    pub game_id: GameId,
    pub player_id: String,
    pub set_id: Option<String>,
    pub tile_id: i32,
    pub tile_index: i32,
}

#[derive(Insertable, AsChangeset, Queryable, Clone)]
#[diesel(treat_none_as_null = true)]
#[diesel(table_name = game_settings)]
pub struct DieselGameSettings {
    pub ai_enabled: i32,
    pub auto_sort_players: String,
    pub auto_stop_claim_meld: String,
    pub discard_wait_ms: Option<i32>,
    pub fixed_settings: i32,
    pub game_id: GameId,
    pub last_discard_time: i64,
}
