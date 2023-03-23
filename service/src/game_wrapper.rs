use actix_web::{web, HttpResponse};
use mahjong_core::{Game, Player, PlayerId, TileId};
use service_contracts::{
    AdminPostClaimTileResponse, AdminPostCreateMeldRequest, AdminPostCreateMeldResponse,
    AdminPostDiscardTileResponse, AdminPostDrawTileResponse, AdminPostMovePlayerResponse,
    GameSummary, SocketMessage, UserPostDiscardTileResponse,
};
use uuid::Uuid;

use crate::{
    http_server::{SocketServer, StorageData},
    socket_server::ClientMessage,
    socket_session::MahjongWebsocketSession,
};

pub struct GameWrapper {
    game: Game,
    socket_server: SocketServer,
    storage: StorageData,
}

impl GameWrapper {
    pub async fn from_storage(
        storage: StorageData,
        game_id: &web::Path<String>,
        socket_server: SocketServer,
    ) -> Result<Self, HttpResponse> {
        let game = storage.get_game(&game_id.to_string()).await;

        if game.is_err() {
            return Err(HttpResponse::InternalServerError().body("Error loading game"));
        }

        let game_content = game.unwrap();

        if game_content.is_none() {
            return Err(HttpResponse::BadRequest().body("No game found"));
        }

        let game = game_content.unwrap();

        Ok(Self {
            storage,
            game,
            socket_server,
        })
    }

    pub async fn from_new_game(storage: StorageData, socket_server: SocketServer) -> Self {
        let game = create_game();

        Self {
            storage,
            game,
            socket_server,
        }
    }

    fn sync_game(&self) {
        self.socket_server.do_send(ClientMessage {
            id: rand::random(),
            msg: SocketMessage::GameUpdate(self.game.clone()),
            room: MahjongWebsocketSession::get_room_id(&self.game.id, None),
        });

        for player in self.game.players.iter() {
            let game_summary = GameSummary::from_game(&self.game, &player.id).unwrap();

            self.socket_server.do_send(ClientMessage {
                id: rand::random(),
                msg: SocketMessage::GameSummaryUpdate(game_summary),
                room: MahjongWebsocketSession::get_room_id(&self.game.id, Some(&player.id)),
            });
        }
    }

    async fn save_and_return<A>(&self, data: A, err_msg: &'static str) -> HttpResponse
    where
        A: serde::Serialize,
    {
        let save_result = self.storage.save_game(&self.game).await;
        self.sync_game();

        match save_result {
            Ok(_) => HttpResponse::Ok().json(data),
            Err(_) => HttpResponse::InternalServerError().body(err_msg),
        }
    }

    pub async fn handle_new_game(&self) -> HttpResponse {
        self.save_and_return(&self.game.table.hands, "Error creating game")
            .await
    }

    pub fn user_load_game(&self, player_id: &PlayerId) -> HttpResponse {
        match GameSummary::from_game(&self.game, player_id) {
            Some(summary) => HttpResponse::Ok().json(summary),
            None => HttpResponse::InternalServerError().body("Error loading game"),
        }
    }

    pub async fn handle_sort_hands(&mut self) -> HttpResponse {
        for player in self.game.players.iter() {
            let hand = self.game.table.hands.get_mut(&player.id).unwrap();
            hand.sort_default(&self.game.deck);
        }

        self.save_and_return(&self.game.table.hands, "Error sorting hands")
            .await
    }

    pub async fn handle_draw_tile(&mut self) -> HttpResponse {
        self.game.draw_tile_from_wall();

        let current_player_id = self.game.get_current_player().id.clone();
        let hand = self.game.table.hands.get(&current_player_id).unwrap();

        let response: AdminPostDrawTileResponse = hand.clone();

        self.save_and_return(&response, "Error when drawing tile")
            .await
    }

    pub async fn handle_discard_tile(&mut self, is_admin: bool, tile_id: &TileId) -> HttpResponse {
        self.game.discard_tile_to_board(tile_id);
        let game = self.game.clone();

        if is_admin {
            let response: AdminPostDiscardTileResponse = game;
            self.save_and_return(&response, "Error when discarding the tile")
                .await
        } else {
            let player_id = self.game.get_current_player().id.clone();
            let response: UserPostDiscardTileResponse =
                GameSummary::from_game(&game, &player_id).unwrap();

            self.save_and_return(&response, "Error when discarding the tile")
                .await
        }
    }

    pub async fn handle_create_meld(&mut self, body: &AdminPostCreateMeldRequest) -> HttpResponse {
        self.game.create_meld(&body.player_id, &body.tiles);

        let current_player_id = self.game.get_current_player().id.clone();
        let hand = self.game.table.hands.get(&current_player_id).unwrap();
        let response: AdminPostCreateMeldResponse = hand.clone();

        self.save_and_return(&response, "Error when creating meld")
            .await
    }

    pub async fn handle_move_player(&mut self) -> HttpResponse {
        let success = self.game.round.next(&self.game.table.hands);

        if success {
            let response: AdminPostMovePlayerResponse = self.game.clone();

            self.save_and_return(&response, "Error moving player").await
        } else {
            HttpResponse::BadRequest().body("Error when moving player")
        }
    }

    pub async fn handle_claim_tile(&mut self, player_id: &PlayerId) -> HttpResponse {
        let success = self.game.claim_tile(player_id);

        if success {
            let response: AdminPostClaimTileResponse = self.game.clone();

            self.save_and_return(&response, "Error claiming tile").await
        } else {
            HttpResponse::BadRequest().body("Error claiming tile")
        }
    }
}

fn create_game() -> Game {
    let mut players: Vec<Player> = vec![];
    for index in 0..4 {
        let player = Player {
            id: Uuid::new_v4().to_string(),
            name: format!("Custom Player {index}"),
        };

        players.push(player);
    }

    let mut game = Game {
        id: Uuid::new_v4().to_string(),
        name: "Custom Game".to_string(),
        ..Default::default()
    };

    game.set_players(&players);

    game
}
