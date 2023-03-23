pub use crate::game_summary::GameSummary;
use mahjong_core::{Game, GameId, Hand, PlayerId, TileId};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

mod game_summary;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocketMessage {
    GameUpdate(Game),
    GameSummaryUpdate(GameSummary),
    ListRooms,
    Name(String),
    PlayerLeft,
    PlayerJoined,
}

#[derive(Serialize, Deserialize)]
pub struct WebSocketQuery {
    pub game_id: GameId,
    pub player_id: Option<PlayerId>,
}

pub type AdminGetGamesResponse = Vec<GameId>;

// Initially separating the get-games endpoints by mode to allow changing the response in future
#[derive(Deserialize, Serialize)]
pub struct UserGetGamesQuery {
    pub player_id: String,
}
pub type UserGetGamesResponse = Vec<GameId>;

pub type AdminPostDrawTileResponse = Hand;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminPostCreateMeldRequest {
    pub player_id: String,
    pub tiles: HashSet<TileId>,
}
pub type AdminPostCreateMeldResponse = Hand;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminPostDiscardTileRequest {
    pub tile_id: TileId,
}
pub type AdminPostDiscardTileResponse = Game;

pub type UserPostDiscardTileRequest = AdminPostDiscardTileRequest;
pub type UserPostDiscardTileResponse = GameSummary;

pub type AdminPostMovePlayerResponse = Game;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminPostClaimTileRequest {
    pub player_id: PlayerId,
}
pub type AdminPostClaimTileResponse = Game;

#[derive(Deserialize, Serialize)]
pub struct UserLoadGameQuery {
    pub player_id: String,
}
pub type UserGetLoadGameResponse = GameSummary;
