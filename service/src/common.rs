use crate::auth::{AuthInfo, GetAuthInfo};
use async_trait::async_trait;
use mahjong_core::{GameId, PlayerId};
use service_contracts::{ServiceGame, ServicePlayer, ServicePlayerGame};

#[async_trait]
pub trait Storage: Send + Sync {
    async fn get_auth_info(&self, get_auth_info: GetAuthInfo) -> Result<Option<AuthInfo>, String>;
    async fn get_game(&self, id: &GameId, use_cache: bool) -> Result<Option<ServiceGame>, String>;
    async fn get_player_games(
        &self,
        player_id: &Option<PlayerId>,
    ) -> Result<Vec<ServicePlayerGame>, String>;
    async fn get_player(&self, id: &PlayerId) -> Result<Option<ServicePlayer>, String>;
    async fn get_player_total_score(&self, id: &PlayerId) -> Result<i32, String>;
    async fn save_auth_info(&self, auth_info: &AuthInfo) -> Result<(), String>;
    async fn save_game(&self, game: &ServiceGame) -> Result<(), String>;
    async fn save_player(&self, player: &ServicePlayer) -> Result<(), String>;
    async fn delete_games(&self, ids: &[GameId]) -> Result<(), String>;
}
