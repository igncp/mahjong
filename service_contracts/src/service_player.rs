use mahjong_core::PlayerId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePlayer {
    pub ai_enabled: bool,
    pub id: PlayerId,
    pub is_ai: bool,
    pub name: String,
}

impl Default for ServicePlayer {
    fn default() -> Self {
        Self {
            ai_enabled: true,
            id: PlayerId::default(),
            is_ai: false,
            name: String::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePlayerSummary {
    pub id: PlayerId,
    pub name: String,
}
