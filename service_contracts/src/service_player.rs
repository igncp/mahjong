use mahjong_core::{GameId, PlayerId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, juniper::GraphQLObject)]
pub struct ServicePlayer {
    pub created_at: String,
    pub id: PlayerId,
    pub is_ai: bool,
    pub name: String,
}

// These timestamps are converted into string to be able to use them in GraphQL.
#[derive(Debug, Clone, Serialize, Deserialize, Default, juniper::GraphQLObject)]
pub struct ServicePlayerGame {
    pub created_at: String,
    pub id: GameId,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct ServicePlayerSummary {
    pub id: PlayerId,
    pub name: String,
}
