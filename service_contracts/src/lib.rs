use mahjong_core::{Game, GameId, Hand};
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
