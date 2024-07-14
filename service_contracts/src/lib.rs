#![deny(clippy::use_self, clippy::shadow_unrelated)]
use std::fmt::{self, Display};
use ts_rs::TS;

use mahjong_core::{
    game::GameVersion, game_summary::GameSummary, hand::SetIdContent, Game, GameId, Hand, Hands,
    PlayerId, TileId,
};
use rustc_hash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};
pub use service_player::{ServicePlayer, ServicePlayerGame, ServicePlayerSummary};

mod service_player;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GameSettings {
    pub ai_enabled: bool,
    pub auto_sort_players: FxHashSet<PlayerId>,
    pub auto_stop_claim_meld: FxHashSet<PlayerId>,
    pub discard_wait_ms: Option<i32>,
    pub fixed_settings: bool,
    pub last_discard_time: i128,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            ai_enabled: true,
            auto_sort_players: FxHashSet::default(),
            auto_stop_claim_meld: FxHashSet::default(),
            discard_wait_ms: Some(1000),
            fixed_settings: false,
            last_discard_time: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceGame {
    pub created_at: i64,
    pub game: Game,
    pub players: FxHashMap<PlayerId, ServicePlayer>,
    pub settings: GameSettings,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GameSettingsSummary {
    pub ai_enabled: bool,
    pub auto_sort: bool,
    pub auto_stop_claim_meld: bool,
    pub discard_wait_ms: Option<i32>,
    pub fixed_settings: bool,
    pub last_discard_time: String,
}

impl GameSettingsSummary {
    pub fn from_game_settings(settings: &GameSettings, player_id: &PlayerId) -> Self {
        Self {
            ai_enabled: settings.ai_enabled,
            auto_sort: settings.auto_sort_players.iter().any(|p| p == player_id),
            auto_stop_claim_meld: settings.auto_stop_claim_meld.iter().any(|p| p == player_id),
            discard_wait_ms: settings.discard_wait_ms,
            fixed_settings: settings.fixed_settings,
            last_discard_time: settings.last_discard_time.to_string(),
        }
    }

    pub fn to_game_settings(&self, player_id: &PlayerId, settings: &GameSettings) -> GameSettings {
        let mut new_settings = settings.clone();

        if self.auto_sort {
            new_settings.auto_sort_players.insert(player_id.clone());
        } else {
            new_settings.auto_sort_players.remove(player_id);
        }

        if self.auto_stop_claim_meld {
            new_settings.auto_stop_claim_meld.insert(player_id.clone());
        } else {
            new_settings.auto_stop_claim_meld.remove(player_id);
        }

        new_settings.ai_enabled = self.ai_enabled;
        new_settings.discard_wait_ms = self.discard_wait_ms;
        new_settings.fixed_settings = self.fixed_settings;
        new_settings.last_discard_time = self.last_discard_time.parse().unwrap_or(0);

        new_settings
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ServiceGameSummary {
    pub game_summary: GameSummary,
    pub players: FxHashMap<PlayerId, ServicePlayerSummary>,
    pub settings: GameSettingsSummary,
}

impl ServiceGame {
    pub fn get_ai_players(&self) -> FxHashSet<PlayerId> {
        self.players
            .iter()
            .filter(|(_, player)| player.is_ai)
            .map(|(id, _)| id.clone())
            .collect::<FxHashSet<PlayerId>>()
    }
}

impl ServiceGameSummary {
    pub fn from_service_game(game: &ServiceGame, player_id: &PlayerId) -> Option<Self> {
        let game_summary = GameSummary::from_game(&game.game, player_id);

        game_summary.as_ref()?;

        let game_summary = game_summary.unwrap();

        let players: FxHashMap<PlayerId, ServicePlayerSummary> = game
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

        Some(Self {
            game_summary,
            players,
            settings: GameSettingsSummary::from_game_settings(&game.settings, player_id),
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

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct WebSocketQuery {
    pub game_id: GameId,
    pub player_id: Option<PlayerId>,
    pub token: String,
}

pub type AdminGetGamesResponse = Vec<ServicePlayerGame>;

// Initially separating the get-games endpoints by mode to allow changing the response in future
#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct UserGetGamesQuery {
    pub player_id: String,
}
pub type UserGetGamesResponse = Vec<ServicePlayerGame>;

pub type AdminPostDrawTileResponse = Hand;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminPostCreateMeldRequest {
    pub player_id: String,
    pub tiles: FxHashSet<TileId>,
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
    pub ai_player_names: Option<Vec<String>>,
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

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct UserPostMovePlayerRequest {
    pub player_id: PlayerId,
}
pub type UserPostMovePlayerResponse = ServiceGameSummary;

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct UserPostSortHandRequest {
    pub game_version: GameVersion,
    pub player_id: PlayerId,
    pub tiles: Option<Vec<TileId>>,
}
pub type UserPostSortHandResponse = ServiceGameSummary;

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct UserPostCreateMeldRequest {
    pub player_id: PlayerId,
    pub tiles: FxHashSet<TileId>,
}
pub type UserPostCreateMeldResponse = ServiceGameSummary;

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct UserPostBreakMeldRequest {
    pub player_id: PlayerId,
    pub set_id: SetIdContent,
}
pub type UserPostBreakMeldResponse = ServiceGameSummary;

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct AdminPostSwapDrawTilesRequest {
    pub tile_id_a: TileId,
    pub tile_id_b: TileId,
}
pub type AdminPostSwapDrawTilesResponse = ServiceGame;

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct AdminPostSayMahjongRequest {
    pub player_id: PlayerId,
}
pub type AdminPostSayMahjongResponse = ServiceGame;

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
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
pub struct UserPostSayMahjongRequest {
    pub player_id: PlayerId,
}
pub type UserPostSayMahjongResponse = ServiceGameSummary;

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct UserPostSetGameSettingsRequest {
    pub player_id: PlayerId,
    pub settings: GameSettingsSummary,
}
pub type UserPostSetGameSettingsResponse = ServiceGameSummary;

#[derive(Deserialize, Serialize, Debug, TS)]
#[ts(export)]
pub struct UserPostSetAuthRequest {
    pub username: String,
    pub password: String,
}
#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct UserPostSetAuthResponse {
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserPostSetAuthAnonRequest {
    pub id_token: String,
}
#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct UserPostSetAuthAnonResponse {
    pub token: String,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct UserPostPassRoundRequest {
    pub player_id: PlayerId,
}
pub type UserPostPassRoundResponse = ServiceGameSummary;

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct UserGetInfoResponse {
    pub name: String,
    pub total_score: i32,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct DashboardPlayer {
    pub id: String,
    pub name: String,
    pub created_at: String,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct DashboardGame {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TS)]
#[ts(export)]
pub enum Provider {
    Anonymous,
    Email,
    Github,
}

#[derive(Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct AuthInfoSummary {
    pub provider: Provider,
    pub username: Option<String>,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct UserGetDashboardResponse {
    pub auth_info: AuthInfoSummary,
    pub player: DashboardPlayer,
    pub player_games: Vec<DashboardGame>,
    pub player_total_score: i32,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct UserPatchInfoRequest {
    pub name: String,
}
pub type UserPatchInfoResponse = UserGetInfoResponse;

impl Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = match self {
            Self::Anonymous => "anonymous".to_string(),
            Self::Email => "email".to_string(),
            Self::Github => "github".to_string(),
        };

        write!(f, "{}", result)
    }
}
