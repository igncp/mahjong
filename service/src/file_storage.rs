use async_trait::async_trait;
use mahjong_core::Game;
use serde::{Deserialize, Serialize};

use crate::common::Storage;

pub struct FileStorage;

#[derive(Serialize, Deserialize)]
struct FileContent {
    games: Option<Vec<Game>>,
}

#[async_trait]
impl Storage for FileStorage {
    async fn save_game(&self, game: &Game) -> Result<(), String> {
        let mut file_content = self.get_file();
        let game = (*game).clone();
        if file_content.games.is_none() {
            file_content.games = Some(vec![]);
        }
        let games = file_content.games.as_mut().unwrap();
        games.push(game);

        self.save_file(&file_content);

        Ok(())
    }

    async fn get_game(&self, id: &str) -> Result<Option<Game>, String> {
        let file_content = self.get_file();
        let game = file_content
            .games
            .unwrap()
            .iter()
            .cloned()
            .find(|game| game.id == id);

        Ok(game)
    }
}

const FILE_PATH: &str = "./mahjong.json";

impl FileStorage {
    pub fn new_dyn() -> Box<dyn Storage> {
        let file_storage = FileStorage {};

        Box::new(file_storage)
    }

    fn save_file(&self, file_content: &FileContent) {
        let file_content_str = serde_json::to_string(&file_content).unwrap();
        std::fs::write(FILE_PATH, file_content_str).unwrap();
    }

    fn get_file(&self) -> FileContent {
        let file_content_str = std::fs::read_to_string(FILE_PATH).unwrap_or("{}".to_string());

        serde_json::from_str::<FileContent>(&file_content_str).unwrap()
    }
}
