use mahjong_core::PlayerId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, juniper::GraphQLObject)]
pub struct ServicePlayer {
    pub id: PlayerId,
    pub is_ai: bool,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePlayerSummary {
    pub id: PlayerId,
    pub name: String,
}
