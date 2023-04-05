use std::{collections::HashSet, env};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use mahjong_core::{GameId, Hands, PlayerId, TileId};
use reqwest::Error;
use service_contracts::{
    AdminGetGamesResponse, AdminPostAIContinueRequest, AdminPostAIContinueResponse,
    AdminPostClaimTileRequest, AdminPostClaimTileResponse, AdminPostCreateMeldRequest,
    AdminPostCreateMeldResponse, AdminPostDiscardTileRequest, AdminPostDiscardTileResponse,
    AdminPostDrawTileResponse, AdminPostMovePlayerResponse, AdminPostSayMahjongRequest,
    AdminPostSayMahjongResponse, AdminPostSortHandsResponse, AdminPostSwapDrawTilesRequest,
    AdminPostSwapDrawTilesResponse, ServiceGame, SocketMessage, UserGetGamesQuery,
    UserGetLoadGameResponse, UserLoadGameQuery, UserPostDiscardTileRequest,
    UserPostDiscardTileResponse, WebSocketQuery,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub struct ServiceHTTPClient {
    domain: String,
    url: String,
    client: reqwest::Client,
    write_stream: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    read_stream: Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
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
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        let domain = env::var("MAHJONG_SERVICE_DOMAIN").unwrap_or("localhost:3000".to_string());
        Self {
            client,
            domain: domain.clone(),
            read_stream: None,
            url: format!("http://{}", domain),
            write_stream: None,
        }
    }

    pub async fn connect_to_websocket(
        &mut self,
        game_id: &str,
        player_id: Option<PlayerId>,
    ) -> Result<(), String> {
        let query = WebSocketQuery {
            game_id: game_id.to_string(),
            player_id,
        };
        let url = format!(
            "ws://{}/v1/ws?{}",
            self.domain,
            serde_qs::to_string(&query).unwrap()
        );
        let connection = connect_async(url).await;
        if connection.is_err() {
            return Err("Failed to connect to websocket".to_string());
        }

        let (ws_stream, _) = connection.unwrap();
        let (write, read) = ws_stream.split();

        self.write_stream = Some(write);
        self.read_stream = Some(read);

        Ok(())
    }

    pub async fn read_message(&mut self) -> Result<SocketMessage, String> {
        let read = self.read_stream.as_mut().unwrap();

        loop {
            let response = read.next().await;

            if response.is_none() {
                return Err("Failed to read message".to_string());
            }

            let msg = response.unwrap().unwrap();
            let msg = msg.to_string();

            // Heartbeat
            if msg.is_empty() {
                continue;
            }

            let socket_message = serde_json::from_str::<SocketMessage>(&msg);

            return match socket_message {
                Ok(message) => Ok(message),
                Err(err) => {
                    println!("Error: {}", err);
                    Err("Failed to parse message".to_string())
                }
            };
        }
    }

    async fn send_message(&mut self, message: &SocketMessage) -> Result<(), String> {
        let write = self.write_stream.as_mut().unwrap();
        let content = serde_json::to_string(&message).unwrap();
        let result = write.send(Message::text(content)).await;
        if result.is_err() {
            return Err("Failed to send message".to_string());
        }

        Ok(())
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

    pub async fn admin_create_game(&self) -> Result<ServiceGame, String> {
        let url = format!("{}/v1/admin/game", self.url);
        let result = self.client.post(url).send().await;
        let validation = validate_response(&result);

        if validation.is_ok() {
            let game = result.unwrap().json::<ServiceGame>().await;
            if game.is_err() {
                return Err("Game not found".to_string());
            }
            Ok(game.unwrap())
        } else {
            Err(validation.err().unwrap())
        }
    }

    pub async fn admin_swap_wall_tiles(
        &self,
        game_id: &GameId,
        tile_id_a: TileId,
        tile_id_b: TileId,
    ) -> Result<AdminPostSwapDrawTilesResponse, String> {
        let url = format!(
            "{}/v1/admin/game/{}/draw-wall-swap-tiles",
            self.url, game_id
        );
        let request = AdminPostSwapDrawTilesRequest {
            tile_id_a,
            tile_id_b,
        };
        let result = self.client.post(url).json(&request).send().await;
        let validation = validate_response(&result);

        if validation.is_ok() {
            let game = result.unwrap().json::<ServiceGame>().await;
            if game.is_err() {
                return Err("Game not found".to_string());
            }
            Ok(game.unwrap())
        } else {
            Err(validation.err().unwrap())
        }
    }

    pub async fn admin_load_game(&self, game_id: &str) -> Result<ServiceGame, String> {
        let url = format!("{}/v1/admin/game/{game_id}", self.url);
        let result = self.client.get(url).send().await;
        let validation = validate_response(&result);

        if validation.is_ok() {
            let game = result.unwrap().json::<ServiceGame>().await;
            if game.is_err() {
                return Err("Game not found".to_string());
            }
            Ok(game.unwrap())
        } else {
            Err(validation.err().unwrap())
        }
    }

    pub async fn admin_say_mahjong(
        &self,
        game_id: &GameId,
        player_id: &PlayerId,
    ) -> Result<AdminPostSayMahjongResponse, String> {
        let url = format!("{}/v1/admin/game/{game_id}/say-mahjong", self.url);
        let request_body = AdminPostSayMahjongRequest {
            player_id: player_id.clone(),
        };
        let result = self.client.post(url).json(&request_body).send().await;
        let validation = validate_response(&result);

        if validation.is_ok() {
            let game = result.unwrap().json::<AdminPostSayMahjongResponse>().await;
            if game.is_err() {
                return Err("Game not found".to_string());
            }
            Ok(game.unwrap())
        } else {
            Err(validation.err().unwrap())
        }
    }

    pub async fn user_load_game(
        &self,
        game_id: &str,
        player_id: &PlayerId,
    ) -> Result<UserGetLoadGameResponse, String> {
        let query = UserLoadGameQuery {
            player_id: player_id.to_string(),
        };
        let url = format!(
            "{}/v1/user/game/{game_id}?{}",
            self.url,
            serde_qs::to_string(&query).unwrap()
        );
        let result = self.client.get(url).send().await;
        let validation = validate_response(&result);

        if validation.is_ok() {
            let game = result.unwrap().json::<UserGetLoadGameResponse>().await;
            if game.is_err() {
                return Err("Game not found".to_string());
            }
            Ok(game.unwrap())
        } else {
            Err(validation.err().unwrap())
        }
    }

    pub async fn admin_sort_hands(
        &self,
        game_id: &str,
    ) -> Result<AdminPostSortHandsResponse, String> {
        let url = format!("{}/v1/admin/game/{game_id}/sort-hands", self.url);
        let result = self.client.post(url).send().await;
        let validation = validate_response(&result);
        if validation.is_ok() {
            let hands = result.unwrap().json::<Hands>().await;
            if hands.is_err() {
                return Err("Hands not found".to_string());
            }
            Ok(hands.unwrap())
        } else {
            Err(validation.err().unwrap())
        }
    }

    pub async fn get_games(&self, user: Option<&PlayerId>) -> Result<Vec<GameId>, String> {
        let url = if user.is_some() {
            let query = UserGetGamesQuery {
                player_id: user.unwrap().to_string(),
            };
            format!(
                "{}/v1/user/game?{}",
                self.url,
                serde_qs::to_string(&query).unwrap()
            )
        } else {
            format!("{}/v1/admin/game", self.url)
        };
        let result = self.client.get(url).send().await;
        let validation = validate_response(&result);
        if validation.is_ok() {
            let games = result.unwrap().json::<AdminGetGamesResponse>().await;
            if games.is_err() {
                return Err("Error getting games".to_string());
            }
            Ok(games.unwrap())
        } else {
            Err(validation.err().unwrap())
        }
    }

    pub async fn admin_draw_tile(
        &self,
        game_id: &GameId,
    ) -> Result<AdminPostDrawTileResponse, String> {
        let url = format!("{}/v1/admin/game/{game_id}/draw-tile", self.url);
        let result = self.client.post(url).send().await;
        let validation = validate_response(&result);
        if validation.is_ok() {
            let response = result.unwrap().json::<AdminPostDrawTileResponse>().await;
            if response.is_err() {
                return Err("Tile could not be drawn".to_string());
            }
            Ok(response.unwrap())
        } else {
            Err(validation.err().unwrap())
        }
    }

    pub async fn admin_create_meld(
        &self,
        game_id: &GameId,
        player_id: &PlayerId,
        tiles: &HashSet<TileId>,
    ) -> Result<AdminPostCreateMeldResponse, String> {
        let url = format!("{}/v1/admin/game/{game_id}/create-meld", self.url);
        let request_body = AdminPostCreateMeldRequest {
            player_id: player_id.clone(),
            tiles: tiles.clone(),
        };
        let result = self.client.post(url).json(&request_body).send().await;
        let validation = validate_response(&result);
        if validation.is_ok() {
            let hand = result.unwrap().json::<AdminPostCreateMeldResponse>().await;
            if hand.is_err() {
                return Err("Meld could not be created".to_string());
            }
            Ok(hand.unwrap())
        } else {
            Err(validation.err().unwrap())
        }
    }

    pub async fn admin_ai_continue(
        &self,
        game_id: &GameId,
    ) -> Result<AdminPostAIContinueResponse, String> {
        let url = format!("{}/v1/admin/game/{game_id}/ai-continue", self.url);
        let body = AdminPostAIContinueRequest { draw: None };
        let result = self.client.post(url).json(&body).send().await;
        let validation = validate_response(&result);
        if validation.is_ok() {
            let hand = result.unwrap().json::<AdminPostAIContinueResponse>().await;
            if hand.is_err() {
                return Err("Tile could not be discarded".to_string());
            }
            Ok(hand.unwrap())
        } else {
            Err(validation.err().unwrap())
        }
    }

    pub async fn admin_discard_tile(
        &self,
        game_id: &GameId,
        tile_id: &TileId,
    ) -> Result<AdminPostDiscardTileResponse, String> {
        let url = format!("{}/v1/admin/game/{game_id}/discard-tile", self.url);
        let request_body = AdminPostDiscardTileRequest { tile_id: *tile_id };
        let result = self.client.post(url).json(&request_body).send().await;
        let validation = validate_response(&result);
        if validation.is_ok() {
            let hand = result.unwrap().json::<AdminPostDiscardTileResponse>().await;
            if hand.is_err() {
                return Err("Tile could not be discarded".to_string());
            }
            Ok(hand.unwrap())
        } else {
            Err(validation.err().unwrap())
        }
    }

    pub async fn user_discard_tile(
        &self,
        game_id: &GameId,
        tile_id: &TileId,
    ) -> Result<UserPostDiscardTileResponse, String> {
        let url = format!("{}/v1/user/game/{game_id}/discard-tile", self.url);
        let request_body = UserPostDiscardTileRequest { tile_id: *tile_id };
        let result = self.client.post(url).json(&request_body).send().await;
        let validation = validate_response(&result);
        if validation.is_ok() {
            let hand = result.unwrap().json::<UserPostDiscardTileResponse>().await;
            if hand.is_err() {
                return Err("Tile could not be discarded".to_string());
            }
            Ok(hand.unwrap())
        } else {
            Err(validation.err().unwrap())
        }
    }

    pub async fn admin_move_player(
        &self,
        game_id: &GameId,
    ) -> Result<AdminPostMovePlayerResponse, String> {
        let url = format!("{}/v1/admin/game/{game_id}/move-player", self.url);
        let result = self.client.post(url).send().await;
        let validation = validate_response(&result);
        if validation.is_ok() {
            let hand = result.unwrap().json::<AdminPostDiscardTileResponse>().await;
            if hand.is_err() {
                return Err("Could not move player".to_string());
            }
            Ok(hand.unwrap())
        } else {
            Err(validation.err().unwrap())
        }
    }

    pub async fn admin_claim_tile(
        &self,
        game_id: &GameId,
        player_id: &PlayerId,
    ) -> Result<AdminPostClaimTileResponse, String> {
        let url = format!("{}/v1/admin/game/{game_id}/claim-tile", self.url);
        let request_body = AdminPostClaimTileRequest {
            player_id: player_id.clone(),
        };
        let result = self.client.post(url).json(&request_body).send().await;
        let validation = validate_response(&result);
        if validation.is_ok() {
            let response = result.unwrap().json::<AdminPostClaimTileResponse>().await;
            if response.is_err() {
                return Err("Could not claim tile".to_string());
            }
            Ok(response.unwrap())
        } else {
            Err(validation.err().unwrap())
        }
    }

    // TODO: Remove
    pub async fn admin_send_foo(&mut self) -> Result<(), String> {
        let message = SocketMessage::ListRooms;
        self.send_message(&message).await
    }
}
