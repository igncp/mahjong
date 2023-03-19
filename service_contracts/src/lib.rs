use std::collections::HashSet;

use mahjong_core::{Game, GameId, Hand, TileId};
use serde::{Deserialize, Serialize};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocketMessage {
    GameUpdate(Game),
    ListRooms,
    Name(String),
    PlayerLeft,
    PlayerJoined,
}

pub type AdminGetGamesResponse = Vec<GameId>;
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

pub type AdminPostMovePlayerResponse = Game;
