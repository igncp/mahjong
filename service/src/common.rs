use crate::auth::{AuthInfo, Username};
use async_trait::async_trait;
use mahjong_core::{GameId, PlayerId};
use service_contracts::{ServiceGame, ServicePlayer};

#[async_trait]
pub trait Storage: Send + Sync {
    async fn get_auth_info(&self, username: &Username) -> Result<Option<AuthInfo>, String>;
    async fn get_game(&self, id: &GameId) -> Result<Option<ServiceGame>, String>;
    async fn get_games_ids(&self, player_id: &Option<PlayerId>) -> Result<Vec<GameId>, String>;
    async fn get_player(&self, id: &PlayerId) -> Result<Option<ServicePlayer>, String>;
    async fn save_auth_info(&self, auth_info: &AuthInfo) -> Result<(), String>;
    async fn save_game(&self, game: &ServiceGame) -> Result<(), String>;
    async fn save_player(&self, player: &ServicePlayer) -> Result<(), String>;
}
