#![deny(clippy::use_self, clippy::shadow_unrelated)]
use mahjong_core::{
    game::GameVersion, game_summary::GameSummary, hand::SetIdContent, Game, GameId, Hand, Hands,
    PlayerId, TileId,
};
use serde::{Deserialize, Serialize};
pub use service_player::{ServicePlayer, ServicePlayerSummary};
use std::collections::{HashMap, HashSet};

mod service_player;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceGame {
    pub game: Game,
    pub players: HashMap<PlayerId, ServicePlayer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceGameSummary {
    pub ai_enabled: bool,
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

        let players: HashMap<PlayerId, ServicePlayerSummary> = game
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
            .collect();

        let player = game.players.get(player_id);
        let ai_enabled = player.map(|p| p.ai_enabled).unwrap_or(false);

        Some(Self {
            ai_enabled,
            game_summary: GameSummary::from_game(&game.game, player_id).unwrap(),
            players,
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
    pub token: String,
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
    pub set_id: SetIdContent,
}
pub type AdminPostBreakMeldResponse = Hand;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminPostDiscardTileRequest {
    pub tile_id: TileId,
}
pub type AdminPostDiscardTileResponse = ServiceGame;

pub type UserPostDiscardTileRequest = AdminPostDiscardTileRequest;
pub type UserPostDiscardTileResponse = ServiceGameSummary;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPostDrawTileRequest {
    pub game_version: GameVersion,
    pub player_id: PlayerId,
}
pub type UserPostDrawTileResponse = ServiceGameSummary;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPostCreateGameRequest {
    pub player_id: PlayerId,
}
pub type UserPostCreateGameResponse = ServiceGameSummary;

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
    pub player_id: PlayerId,
}
pub type UserGetLoadGameResponse = ServiceGameSummary;

#[derive(Deserialize, Serialize)]
pub struct UserPostMovePlayerRequest {
    pub player_id: PlayerId,
}
pub type UserPostMovePlayerResponse = ServiceGameSummary;

#[derive(Deserialize, Serialize)]
pub struct UserPostSortHandRequest {
    pub game_version: GameVersion,
    pub player_id: PlayerId,
}
pub type UserPostSortHandResponse = ServiceGameSummary;

#[derive(Deserialize, Serialize)]
pub struct UserPostCreateMeldRequest {
    pub player_id: PlayerId,
    pub tiles: HashSet<TileId>,
}
pub type UserPostCreateMeldResponse = ServiceGameSummary;

#[derive(Deserialize, Serialize)]
pub struct UserPostBreakMeldRequest {
    pub player_id: PlayerId,
    pub set_id: SetIdContent,
}
pub type UserPostBreakMeldResponse = ServiceGameSummary;

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
pub struct AdminPostAIContinueRequest {
    pub draw: Option<bool>,
}
#[derive(Deserialize, Serialize)]
pub struct AdminPostAIContinueResponse {
    pub service_game: ServiceGame,
    pub changed: bool,
}

#[derive(Deserialize, Serialize)]
pub struct UserPostAIContinueRequest {
    pub player_id: PlayerId,
}
#[derive(Deserialize, Serialize)]
pub struct UserPostAIContinueResponse {
    pub service_game_summary: ServiceGameSummary,
    pub changed: bool,
}

#[derive(Deserialize, Serialize)]
pub struct UserPostClaimTileRequest {
    pub player_id: PlayerId,
}
pub type UserPostClaimTileResponse = ServiceGameSummary;

#[derive(Deserialize, Serialize)]
pub struct UserPostSettingsRequest {
    pub ai_enabled: Option<bool>,
    pub player_id: PlayerId,
}
pub type UserPostSettingsResponse = ();

#[derive(Deserialize, Serialize)]
pub struct UserPostSayMahjongRequest {
    pub player_id: PlayerId,
}
pub type UserPostSayMahjongResponse = ServiceGameSummary;

#[derive(Deserialize, Serialize, Debug)]
pub struct UserPostSetAuthRequest {
    pub username: String,
    pub password: String,
}
#[derive(Deserialize, Serialize)]
pub struct UserPostSetAuthResponse {
    pub token: String,
}
