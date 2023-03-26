use async_trait::async_trait;
use mahjong_core::{GameId, PlayerId};
use service_contracts::ServiceGame;

#[async_trait]
pub trait Storage: Send + Sync {
    async fn save_game(&self, game: &ServiceGame) -> Result<(), String>;
    async fn get_game(&self, id: &str) -> Result<Option<ServiceGame>, String>;
    async fn get_games_ids(&self, player_id: &Option<PlayerId>) -> Result<Vec<GameId>, String>;
}
