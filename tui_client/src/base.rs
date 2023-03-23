use std::collections::HashSet;

use mahjong_core::{Game, GameId, Hand, PlayerId, TileId};
use service_contracts::{
    AdminPostDiscardTileResponse, AdminPostDrawTileResponse, GameSummary, SocketMessage,
    UserPostDiscardTileResponse,
};

use crate::service_http_client::ServiceHTTPClient;

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    User,
    Admin,
}

pub struct App {
    pub game: Option<Game>,
    pub game_summary: Option<GameSummary>,
    pub games_ids: Option<Vec<GameId>>,
    pub mode: Option<Mode>,
    pub user_id: Option<PlayerId>,
    pub waiting: bool,
    service_client: ServiceHTTPClient,
}

#[derive(Debug, Clone)]
pub struct AppEvent(pub String);

impl App {
    pub async fn new() -> Self {
        let mahjong_client = ServiceHTTPClient::new();

        let health = mahjong_client.check_health().await;

        if health.is_err() {
            println!("Error: {}", health.err().unwrap());
            std::process::exit(1);
        }

        App {
            game: None,
            game_summary: None,
            games_ids: None,
            mode: None,
            service_client: mahjong_client,
            user_id: None,
            waiting: false,
        }
    }

    pub fn is_current_player(&self) -> bool {
        if self.mode == Some(Mode::Admin) {
            return true;
        }

        let game_summary = self.game_summary.as_ref().unwrap();
        let player_id = game_summary.player_id.clone();
        let current_player = &game_summary.players[game_summary.round.player_index];

        player_id == current_player.id
    }

    pub async fn admin_start_game(&mut self) {
        self.waiting = true;
        let game = self.service_client.admin_create_game().await;
        self.waiting = false;

        if game.is_err() {
            println!("Error: {}", game.err().unwrap());
            std::process::exit(1);
        }

        let game = game.unwrap();

        self.game = Some(game.clone());

        let websocket = self
            .service_client
            .connect_to_websocket(&game.id.clone(), None)
            .await;

        if websocket.is_err() {
            println!("Error: {:?}", websocket);
            std::process::exit(1);
        }
    }

    pub async fn user_load_game(
        &mut self,
        game_id: &str,
        player_id: &PlayerId,
    ) -> Result<(), String> {
        self.waiting = true;

        let game_summary = self.service_client.user_load_game(game_id, player_id).await;
        self.waiting = false;
        if game_summary.is_err() {
            return Err("Failed to load game".to_string());
        }

        self.game_summary = Some(game_summary.unwrap());

        let websocket = self
            .service_client
            .connect_to_websocket(game_id, Some(player_id.clone()))
            .await;
        if websocket.is_err() {
            println!("Error: {}", websocket.err().unwrap());
            std::process::exit(1);
        }

        Ok(())
    }

    pub async fn admin_load_game(&mut self, game_id: &str) -> Result<(), String> {
        self.waiting = true;

        let game = self.service_client.admin_load_game(game_id).await;
        self.waiting = false;
        if game.is_err() {
            return Err("Failed to load game".to_string());
        }

        self.game = Some(game.unwrap());

        let websocket = self
            .service_client
            .connect_to_websocket(game_id, None)
            .await;
        if websocket.is_err() {
            println!("Error: {}", websocket.err().unwrap());
            std::process::exit(1);
        }

        Ok(())
    }

    pub async fn get_games(&mut self) {
        self.waiting = true;
        let games = self
            .service_client
            .get_games(if self.mode == Some(Mode::User) {
                Some(self.user_id.as_ref().unwrap())
            } else {
                None
            })
            .await;
        self.waiting = false;
        if games.is_err() {
            println!("Error: {}", games.err().unwrap());
            std::process::exit(1);
        }
        self.games_ids = Some(games.unwrap());
    }

    pub async fn admin_sort_hands(&mut self) -> bool {
        let game = self.game.as_mut().unwrap();
        self.waiting = true;
        let sorted_hands = self.service_client.admin_sort_hands(&game.id).await;
        self.waiting = false;

        if sorted_hands.is_err() {
            return false;
        }

        let sorted_hands = sorted_hands.unwrap();

        game.table.hands = sorted_hands;

        true
    }

    pub async fn wait_for_message(&mut self) -> Result<AppEvent, String> {
        let message = self.service_client.read_message().await;

        if message.is_err() {
            return Err("Failed to read messages".to_string());
        }

        let message = message.unwrap();
        let message = match message {
            SocketMessage::ListRooms => "list".to_string(),
            SocketMessage::GameUpdate(new_game) => {
                self.game = Some(new_game);
                "update".to_string()
            }
            SocketMessage::GameSummaryUpdate(new_game) => {
                self.game_summary = Some(new_game);
                "update".to_string()
            }
            _ => "other".to_string(),
        };

        Ok(AppEvent(message))
    }

    pub async fn admin_draw_tile(&mut self) -> bool {
        self.waiting = true;
        let game = self.game.as_mut().unwrap();
        let result = self.service_client.admin_draw_tile(&game.id).await;
        self.waiting = false;

        if result.is_err() {
            return false;
        }

        let hand: AdminPostDrawTileResponse = result.unwrap();
        let current_player = game.get_current_player();

        game.table.hands.insert(current_player.id.clone(), hand);

        true
    }

    pub async fn admin_create_meld(
        &mut self,
        player_id: &PlayerId,
        tiles: &HashSet<TileId>,
    ) -> Hand {
        self.waiting = true;
        let game_id = self.game.as_ref().unwrap().id.clone();

        let hand = self
            .service_client
            .admin_create_meld(&game_id, player_id, tiles)
            .await;

        self.waiting = false;

        if hand.is_err() {
            println!("Error: {}", hand.err().unwrap());
            std::process::exit(1);
        }

        hand.unwrap()
    }

    pub async fn admin_discard_tile(&mut self, tile_id: &TileId) {
        self.waiting = true;
        let game = self.game.as_mut().unwrap();
        let result = self
            .service_client
            .admin_discard_tile(&game.id, tile_id)
            .await;
        self.waiting = false;

        if result.is_err() {
            println!("Error: {}", result.err().unwrap());
            std::process::exit(1);
        }

        let game: AdminPostDiscardTileResponse = result.unwrap();
        self.game = Some(game);
    }

    pub async fn user_discard_tile(&mut self, tile_id: &TileId) {
        self.waiting = true;
        let game_summary = self.game_summary.as_mut().unwrap();
        let result = self
            .service_client
            .user_discard_tile(&game_summary.id, tile_id)
            .await;
        self.waiting = false;

        if result.is_err() {
            println!("Error: {}", result.err().unwrap());
            std::process::exit(1);
        }

        let game_summary: UserPostDiscardTileResponse = result.unwrap();
        self.game_summary = Some(game_summary);
    }

    pub async fn admin_move_player(&mut self) -> bool {
        self.waiting = true;
        let game = self.game.as_mut().unwrap();
        let result = self.service_client.admin_move_player(&game.id).await;
        self.waiting = false;

        // Ignore the error case
        if let Ok(game) = result {
            self.game = Some(game);
            return true;
        }

        false
    }

    // This can potentially be moved to the backend
    pub async fn admin_move_player_combined(&mut self) {
        let moved = self.admin_move_player().await;

        if !moved {
            return;
        }

        let drawn = self.admin_draw_tile().await;
        if !drawn {
            return;
        };

        self.admin_sort_hands().await;
    }

    pub async fn admin_claim_tile(&mut self, player_id: &PlayerId) -> bool {
        self.waiting = true;
        let game = self.game.as_mut().unwrap();
        let result = self
            .service_client
            .admin_claim_tile(&game.id, player_id)
            .await;
        self.waiting = false;

        if result.is_err() {
            return false;
        }

        self.game = Some(result.unwrap());

        true
    }

    // To be removed
    pub async fn admin_send_foo(&mut self) {
        self.waiting = true;
        let result = self.service_client.admin_send_foo().await;
        self.waiting = false;

        if result.is_err() {
            println!("Error: {}", result.err().unwrap());
            std::process::exit(1);
        }
    }
}
