use mahjong_core::{game_summary::GameSummary, GameId, HandTile};
use service_contracts::{GameSettingsSummary, ServiceGameSummary, ServicePlayerSummary};

#[derive(Debug, Clone, juniper::GraphQLObject)]
pub struct GraphQLGameSummary {
    pub draw_wall_count: i32,
    pub hand: Vec<HandTile>,
    pub id: GameId,
}
impl GraphQLGameSummary {
    pub fn from_game_summary(game_summary: &GameSummary) -> Self {
        Self {
            draw_wall_count: game_summary.draw_wall_count as i32,
            hand: game_summary.hand.0.clone(),
            id: game_summary.id.clone(),
        }
    }
}

#[derive(Debug, Clone, juniper::GraphQLObject)]
pub struct GraphQLServiceGameSummary {
    pub game_summary: GraphQLGameSummary,
    pub players: Vec<ServicePlayerSummary>,
    pub settings: GameSettingsSummary,
}

impl GraphQLServiceGameSummary {
    pub fn from_service_game_summary(service_game_summary: &ServiceGameSummary) -> Self {
        let players = service_game_summary.players.values().cloned().collect();
        let game_summary =
            GraphQLGameSummary::from_game_summary(&service_game_summary.game_summary);

        Self {
            game_summary,
            players,
            settings: service_game_summary.settings.clone(),
        }
    }
}
