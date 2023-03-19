use async_trait::async_trait;
use mahjong_core::{Game, GameId};

#[async_trait]
pub trait Storage: Send + Sync {
    async fn save_game(&self, game: &Game) -> Result<(), String>;
    async fn get_game(&self, id: &str) -> Result<Option<Game>, String>;
    async fn get_games_ids(&self) -> Result<Vec<GameId>, String>;
}
