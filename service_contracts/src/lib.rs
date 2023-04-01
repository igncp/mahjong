pub use crate::game_summary::GameSummary;
use mahjong_core::{Game, GameId, Hand, Hands, PlayerId, TileId};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

mod game_summary;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePlayer {
    pub id: PlayerId,
    pub name: String,
    pub is_ai: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePlayerSummary {
    pub id: PlayerId,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceGame {
    pub game: Game,
    pub players: HashMap<PlayerId, ServicePlayer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceGameSummary {
    pub game_summary: GameSummary,
    pub players: HashMap<PlayerId, ServicePlayerSummary>,
}

impl ServiceGame {
    pub fn get_ai_players(&self) -> HashSet<PlayerId> {
        self.players
            .iter()
            .filter(|(_, player)| player.is_ai)
            .map(|(id, _)| id.clone())
            .collect::<HashSet<PlayerId>>()
    }
}

impl ServiceGameSummary {
    pub fn from_service_game(game: &ServiceGame, player_id: &PlayerId) -> Option<Self> {
        let game_summary = GameSummary::from_game(&game.game, player_id);

        game_summary?;

        Some(ServiceGameSummary {
            game_summary: GameSummary::from_game(&game.game, player_id).unwrap(),
            players: game
                .players
                .clone()
                .into_iter()
                .map(|(id, player)| {
                    (
                        id,
                        ServicePlayerSummary {
                            id: player.id,
                            name: player.name,
                        },
                    )
                })
                .collect(),
        })
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocketMessage {
    GameUpdate(ServiceGame),
    GameSummaryUpdate(ServiceGameSummary),
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
pub struct AdminPostBreakMeldRequest {
    pub player_id: String,
    pub set_id: String,
}
pub type AdminPostBreakMeldResponse = Hand;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminPostDiscardTileRequest {
    pub tile_id: TileId,
}
pub type AdminPostDiscardTileResponse = ServiceGame;

pub type UserPostDiscardTileRequest = AdminPostDiscardTileRequest;
pub type UserPostDiscardTileResponse = ServiceGameSummary;

pub type AdminPostMovePlayerRequest = ();
pub type AdminPostMovePlayerResponse = ServiceGame;

pub type AdminPostSortHandsRequest = ();
pub type AdminPostSortHandsResponse = Hands;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminPostClaimTileRequest {
    pub player_id: PlayerId,
}
pub type AdminPostClaimTileResponse = ServiceGame;

#[derive(Deserialize, Serialize)]
pub struct UserLoadGameQuery {
    pub player_id: String,
}
pub type UserGetLoadGameResponse = ServiceGameSummary;

#[derive(Deserialize, Serialize)]
pub struct AdminPostSwapDrawTilesRequest {
    pub tile_id_a: TileId,
    pub tile_id_b: TileId,
}
pub type AdminPostSwapDrawTilesResponse = ServiceGame;

#[derive(Deserialize, Serialize)]
pub struct AdminPostSayMahjongRequest {
    pub player_id: PlayerId,
}
pub type AdminPostSayMahjongResponse = ServiceGame;

#[derive(Deserialize, Serialize)]
pub struct AdminPostAIContinueResponse {
    pub service_game: ServiceGame,
    pub changed: bool,
}
