use mahjong_core::Game;

use crate::service_http_client::ServiceHTTPClient;

#[derive(Debug, PartialEq)]
pub enum AppDisplay {
    Game,
    Init,
}

pub struct App {
    service_client: ServiceHTTPClient,
    pub waiting: bool,
    pub game: Option<Game>,
    pub display: AppDisplay,
}

impl App {
    pub async fn new() -> Self {
        let mahjong_client = ServiceHTTPClient::new();

        let health = mahjong_client.check_health().await;

        if health.is_err() {
            println!("Error: {}", health.err().unwrap());
            std::process::exit(1);
        }

        App {
            display: AppDisplay::Init,
            game: None,
            service_client: mahjong_client,
            waiting: false,
        }
    }

    pub async fn start_game(&mut self) {
        let game = self.service_client.create_game().await;

        if game.is_err() {
            println!("Error: {}", game.err().unwrap());
            std::process::exit(1);
        }

        let game = game.unwrap();

        self.game = Some(game);
    }

    pub async fn load_game(&mut self, game_id: &str) -> Result<(), String> {
        let game = self.service_client.load_game(game_id).await;
        if game.is_err() {
            return Err("Failed to load game".to_string());
        }

        self.game = Some(game.unwrap());
        self.display = AppDisplay::Game;

        Ok(())
    }
}
