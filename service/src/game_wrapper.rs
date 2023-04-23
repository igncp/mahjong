use crate::{
    ai_wrapper::AIWrapper,
    http_server::{SocketServer, StorageData},
    socket_server::ClientMessage,
    socket_session::MahjongWebsocketSession,
};
use actix_web::{web, HttpResponse};
use mahjong_core::{game::GameVersion, Game, PlayerId, TileId};
use service_contracts::{
    AdminPostAIContinueRequest, AdminPostAIContinueResponse, AdminPostBreakMeldRequest,
    AdminPostBreakMeldResponse, AdminPostClaimTileResponse, AdminPostCreateMeldRequest,
    AdminPostCreateMeldResponse, AdminPostDiscardTileResponse, AdminPostDrawTileResponse,
    AdminPostMovePlayerResponse, AdminPostSayMahjongResponse, AdminPostSwapDrawTilesResponse,
    GameSettings, ServiceGame, ServiceGameSummary, ServicePlayer, SocketMessage,
    UserPostAIContinueRequest, UserPostAIContinueResponse, UserPostBreakMeldRequest,
    UserPostBreakMeldResponse, UserPostCreateGameResponse, UserPostCreateMeldRequest,
    UserPostCreateMeldResponse, UserPostDiscardTileResponse, UserPostDrawTileResponse,
    UserPostMovePlayerResponse, UserPostSayMahjongResponse, UserPostSetGameSettingsResponse,
    UserPostSortHandResponse,
};
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

pub struct GameWrapper<'a> {
    service_game: ServiceGame,
    socket_server: SocketServer,
    storage: &'a StorageData,
}

impl<'a> GameWrapper<'a> {
    pub async fn from_storage(
        storage: &'a StorageData,
        game_id: &web::Path<String>,
        socket_server: SocketServer,
        game_version: Option<&'a GameVersion>,
    ) -> Result<GameWrapper<'a>, HttpResponse> {
        let game = storage.get_game(&game_id.to_string()).await;

        if game.is_err() {
            return Err(HttpResponse::InternalServerError().body("Error loading game"));
        }

        let game_content = game.unwrap();

        if game_content.is_none() {
            return Err(HttpResponse::BadRequest().body("No game found"));
        }

        let service_game = game_content.unwrap();

        if game_version.is_some() && service_game.game.version != *game_version.unwrap() {
            return Err(HttpResponse::BadRequest().body("Game version mismatch"));
        }

        Ok(Self {
            storage,
            service_game,
            socket_server,
        })
    }

    pub async fn from_new_game(
        storage: &'a StorageData,
        socket_server: SocketServer,
        player_id: Option<PlayerId>,
    ) -> Result<GameWrapper<'a>, String> {
        let service_player = if player_id.is_some() {
            let player = storage.get_player(&player_id.unwrap()).await;

            if player.is_err() {
                return Err("Player not found".to_string());
            }

            let player_content = player.unwrap();

            if player_content.is_none() {
                return Err("Player not found".to_string());
            }

            Some(player_content.unwrap())
        } else {
            None
        };
        let service_game = create_game(&service_player);

        Ok(Self {
            storage,
            service_game,
            socket_server,
        })
    }

    pub async fn handle_admin_say_mahjong(&mut self, player_id: &PlayerId) -> HttpResponse {
        let success = self.service_game.game.say_mahjong(player_id);

        if !success {
            return HttpResponse::BadRequest().body("Error saying mahjong");
        }

        let response: &AdminPostSayMahjongResponse = &self.service_game;

        self.save_and_return(response, "Error saying mahjong").await
    }

    pub async fn handle_user_say_mahjong(&mut self, player_id: &PlayerId) -> HttpResponse {
        let success = self.service_game.game.say_mahjong(player_id);

        if !success {
            return HttpResponse::BadRequest().body("Error saying mahjong");
        }

        let game_summary: UserPostSayMahjongResponse =
            ServiceGameSummary::from_service_game(&self.service_game, player_id).unwrap();

        self.save_and_return(&game_summary, "Error saying mahjong")
            .await
    }

    fn sync_game(&self) {
        let socket_server = loop {
            if let Ok(srv) = self.socket_server.lock() {
                break srv.clone();
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        };
        socket_server.do_send(ClientMessage {
            id: rand::random(),
            msg: SocketMessage::GameUpdate(self.service_game.clone()),
            room: MahjongWebsocketSession::get_room_id(&self.service_game.game.id, None),
        });

        for player in self.service_game.game.players.iter() {
            let game_summary =
                ServiceGameSummary::from_service_game(&self.service_game, player).unwrap();

            socket_server.do_send(ClientMessage {
                id: rand::random(),
                msg: SocketMessage::GameSummaryUpdate(game_summary),
                room: MahjongWebsocketSession::get_room_id(
                    &self.service_game.game.id,
                    Some(player),
                ),
            });
        }

        std::mem::drop(socket_server);
    }

    async fn save_and_return<A>(&self, data: A, err_msg: &'static str) -> HttpResponse
    where
        A: serde::Serialize,
    {
        let save_result = self.storage.save_game(&self.service_game).await;
        self.sync_game();

        match save_result {
            Ok(_) => HttpResponse::Ok().json(data),
            Err(_) => HttpResponse::InternalServerError().body(err_msg),
        }
    }

    pub async fn handle_admin_new_game(&self) -> HttpResponse {
        self.save_and_return(&self.service_game, "Error creating game")
            .await
    }

    pub async fn handle_user_new_game(&self, player_id: &PlayerId) -> HttpResponse {
        let response: UserPostCreateGameResponse =
            ServiceGameSummary::from_service_game(&self.service_game, player_id).unwrap();

        self.save_and_return(response, "Error creating game").await
    }

    pub fn user_load_game(&self, player_id: &PlayerId) -> HttpResponse {
        match ServiceGameSummary::from_service_game(&self.service_game, player_id) {
            Some(summary) => HttpResponse::Ok().json(summary),
            None => HttpResponse::InternalServerError().body("Error loading game"),
        }
    }

    pub async fn handle_sort_hands(&mut self) -> HttpResponse {
        for player in self.service_game.game.players.iter() {
            let hand = self.service_game.game.table.hands.get_mut(player).unwrap();
            hand.sort_default(&self.service_game.game.deck);
        }

        self.save_and_return(&self.service_game.game.table.hands, "Error sorting hands")
            .await
    }

    pub async fn handle_user_set_game_settings(
        &mut self,
        player_id: &PlayerId,
        settings: &GameSettings,
    ) -> HttpResponse {
        if settings.fixed_settings {
            return HttpResponse::BadRequest().body("Cannot change fixed settings");
        }

        self.service_game.settings = settings.clone();
        let game_summary: UserPostSetGameSettingsResponse =
            ServiceGameSummary::from_service_game(&self.service_game, player_id).unwrap();

        self.save_and_return(game_summary, "Error setting game settings")
            .await
    }

    pub async fn handle_admin_draw_tile(&mut self) -> HttpResponse {
        self.service_game.game.draw_tile_from_wall();

        let current_player_id = self.service_game.game.get_current_player().clone();
        let hand = self
            .service_game
            .game
            .table
            .hands
            .get(&current_player_id)
            .unwrap();

        let response: AdminPostDrawTileResponse = hand.clone();

        self.save_and_return(&response, "Error when drawing tile")
            .await
    }

    pub async fn handle_user_draw_tile(&mut self, player_id: &PlayerId) -> HttpResponse {
        let current_player = self.service_game.game.get_current_player();
        if &current_player != player_id {
            return HttpResponse::BadRequest().body("Not your turn");
        }

        self.service_game.game.draw_tile_from_wall();
        self.service_game.game.update_version();

        let response: UserPostDrawTileResponse =
            ServiceGameSummary::from_service_game(&self.service_game, player_id).unwrap();

        self.save_and_return(&response, "Error when drawing tile")
            .await
    }

    pub async fn handle_draw_wall_swap_tiles(
        &mut self,
        tile_id_a: &TileId,
        tile_id_b: &TileId,
    ) -> HttpResponse {
        let swapped = self
            .service_game
            .game
            .draw_wall_swap_tiles(tile_id_a, tile_id_b);

        if !swapped {
            return HttpResponse::BadRequest().body("Error when swapping tiles");
        }

        let response: AdminPostSwapDrawTilesResponse = self.service_game.clone();

        self.save_and_return(&response, "Error when swapping tiles")
            .await
    }

    pub async fn handle_admin_ai_continue(
        &mut self,
        body: &AdminPostAIContinueRequest,
    ) -> HttpResponse {
        let mut standard_ai = AIWrapper::new(&mut self.service_game, body.draw);

        let mut global_changed = false;

        loop {
            let changed = standard_ai.play_action().changed;
            if !global_changed {
                global_changed = changed;
            }
            if !changed {
                break;
            }
        }

        let response: AdminPostAIContinueResponse = AdminPostAIContinueResponse {
            service_game: self.service_game.to_owned(),
            changed: global_changed,
        };

        self.save_and_return(response, "Error with AI action").await
    }

    pub async fn handle_user_ai_continue(
        &mut self,
        body: &UserPostAIContinueRequest,
    ) -> HttpResponse {
        let mut standard_ai = AIWrapper::new(&mut self.service_game, None);

        let mut global_changed = false;

        loop {
            let changed = standard_ai.play_action().changed;

            if !global_changed {
                global_changed = changed;
            }

            let is_after_discard = standard_ai.get_is_after_discard();

            if is_after_discard {
                break;
            }

            if !changed {
                break;
            }
        }

        self.service_game.game.update_version();

        let response = UserPostAIContinueResponse {
            service_game_summary: ServiceGameSummary::from_service_game(
                &self.service_game,
                &body.player_id,
            )
            .unwrap(),
            changed: global_changed,
        };

        self.save_and_return(response, "Error with AI action").await
    }

    pub async fn handle_server_ai_continue(&mut self) -> HttpResponse {
        if !self.service_game.settings.ai_enabled {
            // This response is not used
            return HttpResponse::BadRequest().body("AI disabled");
        }

        let mut standard_ai = AIWrapper::new(&mut self.service_game, None);

        standard_ai.play_action();

        self.save_and_return(&self.service_game.game, "Error with AI action")
            .await
    }

    pub async fn handle_user_move_player(&mut self, player_id: &PlayerId) -> HttpResponse {
        let current_player = self.service_game.game.get_current_player();

        if &current_player != player_id {
            return HttpResponse::BadRequest().body("Not your turn");
        }

        let success = self
            .service_game
            .game
            .round
            .next(&self.service_game.game.table.hands);

        self.service_game.game.update_version();

        if success {
            let response: UserPostMovePlayerResponse =
                ServiceGameSummary::from_service_game(&self.service_game, player_id).unwrap();

            self.save_and_return(response, "Error moving player").await
        } else {
            HttpResponse::BadRequest().body("Error when moving player")
        }
    }

    pub async fn handle_discard_tile(&mut self, is_admin: bool, tile_id: &TileId) -> HttpResponse {
        let now_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        self.service_game.game.discard_tile_to_board(tile_id);
        let mut game = self.service_game.clone();

        game.game.update_version();
        game.settings.last_discard_time = now_time;

        if is_admin {
            let response: AdminPostDiscardTileResponse = game;
            self.save_and_return(&response, "Error when discarding the tile")
                .await
        } else {
            let player_id = self.service_game.game.get_current_player().clone();
            let response: UserPostDiscardTileResponse =
                ServiceGameSummary::from_service_game(&game, &player_id).unwrap();

            self.save_and_return(&response, "Error when discarding the tile")
                .await
        }
    }

    pub async fn handle_admin_break_meld(
        &mut self,
        body: &AdminPostBreakMeldRequest,
    ) -> HttpResponse {
        let result = self
            .service_game
            .game
            .break_meld(&body.player_id, &body.set_id);

        if !result {
            return HttpResponse::BadRequest().body("Error when breaking meld");
        }

        let hand = self
            .service_game
            .game
            .table
            .hands
            .get(&body.player_id)
            .unwrap();

        let response: AdminPostBreakMeldResponse = hand.clone();

        self.save_and_return(&response, "Error when breaking meld")
            .await
    }

    pub async fn handle_user_break_meld(
        &mut self,
        body: &UserPostBreakMeldRequest,
    ) -> HttpResponse {
        let result = self
            .service_game
            .game
            .break_meld(&body.player_id, &body.set_id);

        if !result {
            return HttpResponse::BadRequest().body("Error when breaking meld");
        }

        self.service_game.game.update_version();

        let response: UserPostBreakMeldResponse =
            ServiceGameSummary::from_service_game(&self.service_game, &body.player_id).unwrap();

        self.save_and_return(&response, "Error when breaking meld")
            .await
    }

    pub async fn handle_admin_create_meld(
        &mut self,
        body: &AdminPostCreateMeldRequest,
    ) -> HttpResponse {
        let result = self
            .service_game
            .game
            .create_meld(&body.player_id, &body.tiles);

        if !result {
            return HttpResponse::BadRequest().body("Error when creating meld");
        }

        let hand = self
            .service_game
            .game
            .table
            .hands
            .get(&body.player_id)
            .unwrap();
        let response: AdminPostCreateMeldResponse = hand.clone();

        self.save_and_return(&response, "Error when creating meld")
            .await
    }

    pub async fn handle_user_create_meld(
        &mut self,
        body: &UserPostCreateMeldRequest,
    ) -> HttpResponse {
        let result = self
            .service_game
            .game
            .create_meld(&body.player_id, &body.tiles);

        if !result {
            return HttpResponse::BadRequest().body("Error when creating meld");
        }

        self.service_game.game.update_version();

        let response: UserPostCreateMeldResponse =
            ServiceGameSummary::from_service_game(&self.service_game, &body.player_id).unwrap();

        self.save_and_return(&response, "Error when creating meld")
            .await
    }

    pub async fn handle_admin_move_player(&mut self) -> HttpResponse {
        let success = self
            .service_game
            .game
            .round
            .next(&self.service_game.game.table.hands);

        if success {
            self.service_game.game.update_version();

            let response: &AdminPostMovePlayerResponse = &self.service_game;

            self.save_and_return(response, "Error moving player").await
        } else {
            HttpResponse::BadRequest().body("Error when moving player")
        }
    }

    pub async fn handle_user_sort_hand(&mut self, player_id: &PlayerId) -> HttpResponse {
        let hand = self
            .service_game
            .game
            .table
            .hands
            .get_mut(player_id)
            .unwrap();

        hand.sort_default(&self.service_game.game.deck);
        self.service_game.game.update_version();

        let response: UserPostSortHandResponse =
            ServiceGameSummary::from_service_game(&self.service_game, player_id).unwrap();

        self.save_and_return(&response, "Error sorting hand").await
    }

    pub async fn handle_admin_claim_tile(&mut self, player_id: &PlayerId) -> HttpResponse {
        let success = self.service_game.game.claim_tile(player_id);

        if success {
            self.service_game.game.update_version();
            let response: &AdminPostClaimTileResponse = &self.service_game;

            self.save_and_return(response, "Error claiming tile").await
        } else {
            HttpResponse::BadRequest().body("Error claiming tile")
        }
    }

    pub async fn handle_user_claim_tile(&mut self, player_id: &PlayerId) -> HttpResponse {
        let success = self.service_game.game.claim_tile(player_id);

        if success {
            self.service_game.game.update_version();

            let response =
                ServiceGameSummary::from_service_game(&self.service_game, player_id).unwrap();

            self.save_and_return(response, "Error claiming tile").await
        } else {
            HttpResponse::BadRequest().body("Error claiming tile")
        }
    }

    pub fn get_current_player_id(&self) -> PlayerId {
        self.service_game.game.get_current_player()
    }
}

fn create_game(player: &Option<ServicePlayer>) -> ServiceGame {
    let mut players: Vec<PlayerId> = vec![];
    for player_index in 0..4 {
        if player_index == 0 && player.is_some() {
            players.push(player.as_ref().unwrap().id.clone());
        } else {
            let id = Uuid::new_v4().to_string();

            players.push(id);
        }
    }

    let mut game = Game {
        id: Uuid::new_v4().to_string(),
        name: "Custom Game".to_string(),
        ..Default::default()
    };

    game.set_players(&players);
    let mut players_set = HashMap::<String, ServicePlayer>::new();

    for (index, game_player) in game.players.iter().enumerate() {
        if index == 0 && player.is_some() {
            let player_clone = player.as_ref().unwrap().clone();
            players_set.insert(player_clone.id.clone(), player_clone);
        } else {
            let service_player = ServicePlayer {
                id: game_player.clone(),
                is_ai: true,
                name: format!("Player {}", index),
            };
            players_set.insert(game_player.clone(), service_player);
        }
    }

    ServiceGame {
        game,
        players: players_set,
        settings: GameSettings::default(),
    }
}
