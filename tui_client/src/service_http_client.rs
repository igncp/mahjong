use mahjong_core::Game;
use reqwest::Error;

pub struct ServiceHTTPClient {
    url: String,
    client: reqwest::Client,
}

fn validate_response(response: &Result<reqwest::Response, Error>) -> Result<(), String> {
    match response {
        Ok(response) => {
            let status = response.status();
            if status == 200 {
                Ok(())
            } else {
                Err(format!("Service is not healthy: {status}"))
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

impl ServiceHTTPClient {
    pub fn new() -> ServiceHTTPClient {
        let client = reqwest::Client::new();
        ServiceHTTPClient {
            url: "http://localhost:3000".to_string(),
            client,
        }
    }

    pub async fn check_health(&self) -> Result<(), String> {
        let url = format!("{}/health", self.url);
        let result = reqwest::get(url).await;
        let validation = validate_response(&result);

        if validation.is_ok() {
            Ok(())
        } else {
            Err(validation.err().unwrap())
        }
    }

    pub async fn create_game(&self) -> Result<Game, String> {
        let url = format!("{}/v1/admin/game", self.url);
        let result = self.client.post(url).send().await;
        let validation = validate_response(&result);

        if validation.is_ok() {
            let game = result.unwrap().json::<Game>().await;
            if game.is_err() {
                return Err("Game not found".to_string());
            }
            Ok(game.unwrap())
        } else {
            Err(validation.err().unwrap())
        }
    }

    pub async fn load_game(&self, game_id: &str) -> Result<Game, String> {
        let url = format!("{}/v1/admin/game?id={game_id}", self.url);
        let result = self.client.get(url).send().await;
        let validation = validate_response(&result);

        if validation.is_ok() {
            let game = result.unwrap().json::<Game>().await;
            if game.is_err() {
                return Err("Game not found".to_string());
            }
            Ok(game.unwrap())
        } else {
            Err(validation.err().unwrap())
        }
    }
}
