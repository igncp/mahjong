use mahjong_core::{GameId, PlayerId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServicePlayer {
    pub created_at: String,
    pub id: PlayerId,
    pub is_ai: bool,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServicePlayerGame {
    pub created_at: String,
    pub id: GameId,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePlayerSummary {
    pub id: PlayerId,
    pub name: String,
}
