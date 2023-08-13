use crate::{
    auth::{AuthInfo, GetAuthInfo, Username},
    common::Storage,
    env::ENV_FILE_STORAGE_KEY,
};
use async_trait::async_trait;
use mahjong_core::{Game, GameId, PlayerId};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use service_contracts::{GameSettings, ServiceGame, ServicePlayer, ServicePlayerGame};
use tracing::debug;

pub struct FileStorage {
    file_path: String,
}

#[derive(Serialize, Deserialize)]
struct FileContent {
    auth: Option<FxHashMap<Username, AuthInfo>>,
    games: Option<FxHashMap<GameId, Game>>,
    players: Option<FxHashMap<PlayerId, ServicePlayer>>,
    settings: Option<FxHashMap<GameId, GameSettings>>,
}

#[async_trait]
impl Storage for FileStorage {
    async fn get_auth_info(&self, get_auth_info: GetAuthInfo) -> Result<Option<AuthInfo>, String> {
        let file_content = self.get_file();
        let username = match get_auth_info {
            GetAuthInfo::Username(username) => username,
            GetAuthInfo::PlayerId(_) => panic!(),
        };
        let auth = file_content.auth.unwrap_or_default();

        let auth_info = auth.get(&username);

        if let Some(auth_info) = auth_info {
            Ok(Some(auth_info.clone()))
        } else {
            Ok(None)
        }
    }

    async fn get_player_total_score(&self, _id: &PlayerId) -> Result<i32, String> {
        unimplemented!()
    }

    async fn save_auth_info(&self, auth_info: &AuthInfo) -> Result<(), String> {
        let mut file_content = self.get_file();
        let mut auth = file_content.auth.unwrap_or_default();

        auth.insert(auth_info.username.clone(), auth_info.clone());

        file_content.auth = Some(auth);

        self.save_file(&file_content);

        Ok(())
    }

    async fn save_game(&self, service_game: &ServiceGame) -> Result<(), String> {
        let mut file_content = self.get_file();
        if file_content.games.is_none() {
            file_content.games = Some(FxHashMap::default());
        }
        if file_content.players.is_none() {
            file_content.players = Some(FxHashMap::default());
        }
        if file_content.settings.is_none() {
            file_content.settings = Some(FxHashMap::default());
        }

        let games = file_content.games.as_mut().unwrap();
        let players = file_content.players.as_mut().unwrap();
        let settings = file_content.settings.as_mut().unwrap();

        games.insert(service_game.game.id.clone(), service_game.game.clone());
        for (player_id, player) in &service_game.players {
            players.insert(player_id.clone(), player.clone());
        }
        settings.insert(service_game.game.id.clone(), service_game.settings.clone());

        self.save_file(&file_content);

        Ok(())
    }

    async fn get_game(&self, id: &GameId) -> Result<Option<ServiceGame>, String> {
        let mut file_content = self.get_file();
        if file_content.games.is_none() {
            file_content.games = Some(FxHashMap::default());
        }
        if file_content.players.is_none() {
            file_content.players = Some(FxHashMap::default());
        }
        if file_content.settings.is_none() {
            file_content.settings = Some(FxHashMap::default());
        }

        let games = file_content.games.as_mut().unwrap();
        let players = file_content.players.as_mut().unwrap();
        let settings = file_content.settings.as_mut().unwrap();

        let game = games.get(id);

        if game.is_none() {
            return Ok(None);
        }

        let players: FxHashMap<PlayerId, ServicePlayer> = game
            .unwrap()
            .players
            .iter()
            .map(|player_id| {
                let player = players.get(player_id).unwrap();

                (player_id.clone(), player.clone())
            })
            .collect();

        let service_game = Some(ServiceGame {
            created_at: 0,
            game: game.unwrap().clone(),
            players,
            settings: settings.get(id).unwrap().clone(),
            updated_at: 0,
        });

        Ok(service_game)
    }

    async fn get_player_games(
        &self,
        _player_id: &Option<PlayerId>,
    ) -> Result<Vec<ServicePlayerGame>, String> {
        panic!("Not implemented")
    }

    async fn get_player(&self, id: &PlayerId) -> Result<Option<ServicePlayer>, String> {
        let file_content = self.get_file();
        let players = file_content.players.unwrap_or_default();

        let player = players.get(id);

        if let Some(player) = player {
            Ok(Some(player.clone()))
        } else {
            Ok(None)
        }
    }

    async fn save_player(&self, player: &ServicePlayer) -> Result<(), String> {
        let mut file_content = self.get_file();
        let mut players = file_content.players.unwrap_or_default();

        players.insert(player.id.clone(), player.clone());

        file_content.players = Some(players);

        self.save_file(&file_content);

        Ok(())
    }

    async fn delete_games(&self, _ids: &[GameId]) -> Result<(), String> {
        panic!()
    }
}

impl FileStorage {
    #[allow(dead_code)]
    pub fn new_dyn() -> Box<dyn Storage> {
        let file_path = std::env::var(ENV_FILE_STORAGE_KEY).unwrap_or("./mahjong.json".to_string());

        debug!("FileStorage: {}", file_path);

        let file_storage = Self { file_path };

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
