use async_trait::async_trait;
use mahjong_core::{GameId, PlayerId};
use serde::{Deserialize, Serialize};
use service_contracts::ServiceGame;

use crate::common::Storage;

pub struct FileStorage {
    file_path: String,
}

#[derive(Serialize, Deserialize)]
struct FileContent {
    games: Option<Vec<ServiceGame>>,
}

#[async_trait]
impl Storage for FileStorage {
    async fn save_game(&self, game: &ServiceGame) -> Result<(), String> {
        let mut file_content = self.get_file();
        if file_content.games.is_none() {
            file_content.games = Some(vec![]);
        }
        let games = file_content.games.as_mut().unwrap();
        let mut games: Vec<ServiceGame> = games
            .iter()
            .filter(|g| g.game.id != game.game.id)
            .cloned()
            .collect();

        games.insert(0, game.clone());

        file_content.games = Some(games);

        self.save_file(&file_content);

        Ok(())
    }

    async fn get_game(&self, id: &str) -> Result<Option<ServiceGame>, String> {
        let file_content = self.get_file();
        let game = file_content
            .games
            .unwrap()
            .iter()
            .cloned()
            .find(|game| game.game.id == id);

        Ok(game)
    }

    async fn get_games_ids(&self, player_id: &Option<PlayerId>) -> Result<Vec<GameId>, String> {
        let file_content = self.get_file();
        let games = file_content.games;

        if games.is_none() {
            return Ok(vec![]);
        }

        let mut games = games.unwrap();

        if player_id.is_some() {
            let player_id = player_id.as_ref().unwrap();
            games = games
                .iter()
                .cloned()
                .filter(|game| game.game.table.hands.get(player_id).is_some())
                .collect();
        }

        let games_ids = games.iter().map(|game| game.game.id.clone()).collect();

        Ok(games_ids)
    }
}

impl FileStorage {
    pub fn new_dyn() -> Box<dyn Storage> {
        let file_path =
            std::env::var("MAHJONG_STORAGE_FILE").unwrap_or("./mahjong.json".to_string());

        let file_storage = FileStorage { file_path };

        Box::new(file_storage)
    }

    fn save_file(&self, file_content: &FileContent) {
        let file_content_str = serde_json::to_string(&file_content).unwrap();
        std::fs::write(&self.file_path, file_content_str).unwrap();
    }

    fn get_file(&self) -> FileContent {
        let file_content_str = std::fs::read_to_string(&self.file_path).unwrap_or("{}".to_string());

        serde_json::from_str::<FileContent>(&file_content_str).unwrap()
    }
}
