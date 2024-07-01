use crate::{
    ai_wrapper::AIWrapper,
    http_server::{DataSocketServer, DataStorage},
    socket::{MahjongWebsocketSession, SocketClientMessage},
    time::get_timestamp,
};
use actix_web::{web, HttpResponse};
use mahjong_core::{
    game::{DrawTileResult, GameVersion},
    Game, PlayerId, Players, TileId,
};
use rustc_hash::{FxHashMap, FxHashSet};
use service_contracts::{
    AdminPostAIContinueRequest, AdminPostAIContinueResponse, AdminPostBreakMeldRequest,
    AdminPostBreakMeldResponse, AdminPostClaimTileResponse, AdminPostCreateMeldRequest,
    AdminPostCreateMeldResponse, AdminPostDiscardTileResponse, AdminPostDrawTileResponse,
    AdminPostMovePlayerResponse, AdminPostSayMahjongResponse, AdminPostSwapDrawTilesResponse,
    GameSettings, GameSettingsSummary, ServiceGame, ServiceGameSummary, ServicePlayer,
    SocketMessage, UserPostAIContinueRequest, UserPostAIContinueResponse, UserPostBreakMeldRequest,
    UserPostBreakMeldResponse, UserPostCreateGameResponse, UserPostCreateMeldRequest,
    UserPostCreateMeldResponse, UserPostDiscardTileResponse, UserPostDrawTileResponse,
    UserPostMovePlayerResponse, UserPostPassRoundResponse, UserPostSayMahjongResponse,
    UserPostSetGameSettingsResponse, UserPostSortHandResponse,
};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;
use uuid::Uuid;

pub struct GameWrapper<'a> {
    service_game: ServiceGame,
    socket_server: DataSocketServer,
    storage: &'a DataStorage,
}

impl<'a> GameWrapper<'a> {
    async fn from_storage_base(
        storage: &'a DataStorage,
        game_id: &web::Path<String>,
        socket_server: DataSocketServer,
        game_version: Option<&'a GameVersion>,
        use_cache: bool,
    ) -> Result<GameWrapper<'a>, HttpResponse> {
        let game = storage.get_game(&game_id.to_string(), use_cache).await;

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

    pub async fn from_storage(
        storage: &'a DataStorage,
        game_id: &web::Path<String>,
        socket_server: DataSocketServer,
        game_version: Option<&'a GameVersion>,
    ) -> Result<GameWrapper<'a>, HttpResponse> {
        Self::from_storage_base(storage, game_id, socket_server, game_version, true).await
    }

    pub async fn from_storage_no_cache(
        storage: &'a DataStorage,
        game_id: &web::Path<String>,
        socket_server: DataSocketServer,
        game_version: Option<&'a GameVersion>,
    ) -> Result<GameWrapper<'a>, HttpResponse> {
        Self::from_storage_base(storage, game_id, socket_server, game_version, false).await
    }

    pub async fn from_new_game(
        storage: &'a DataStorage,
        socket_server: DataSocketServer,
        player_id: Option<PlayerId>,
        ai_player_names: &Option<Vec<String>>,
    ) -> Result<GameWrapper<'a>, String> {
        let service_player = if let Some(player_id) = player_id {
            let player = storage.get_player(&player_id).await;

            if player.is_err() {
                debug!("Player not found with error");
                return Err("Player not found".to_string());
            }

            let player_content = player.unwrap();

            if player_content.is_none() {
                debug!("Player not found with none");
                return Err("Player not found".to_string());
            }

            Some(player_content.unwrap())
        } else {
            None
        };
        let service_game = create_game(&service_player, ai_player_names);

        Ok(Self {
            storage,
            service_game,
            socket_server,
        })
    }

    pub async fn handle_admin_say_mahjong(&mut self, player_id: &PlayerId) -> HttpResponse {
        let success = self.service_game.game.say_mahjong(player_id);

        if success.is_err() {
            return HttpResponse::BadRequest().body("Error saying mahjong");
        }

        self.sync_game_updated();

        let response: &AdminPostSayMahjongResponse = &self.service_game;

        self.save_and_return(response, "Error saying mahjong").await
    }

    pub async fn handle_user_say_mahjong(&mut self, player_id: &PlayerId) -> HttpResponse {
        let success = self.service_game.game.say_mahjong(player_id);

        if success.is_err() {
            return HttpResponse::BadRequest().body("Error saying mahjong");
        }

        self.sync_game_updated();

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
        socket_server.do_send(SocketClientMessage {
            id: rand::random(),
            msg: SocketMessage::GameUpdate(self.service_game.clone()),
            room: MahjongWebsocketSession::get_room_id(&self.service_game.game.id, None),
        });

        for player in self.service_game.game.players.iter() {
            let game_summary =
                ServiceGameSummary::from_service_game(&self.service_game, player).unwrap();

            socket_server.do_send(SocketClientMessage {
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

        if save_result.is_err() {
            debug!("Error saving game");
            return HttpResponse::InternalServerError().body(err_msg);
        }

        self.sync_game();

        HttpResponse::Ok().json(data)
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
            let hand = self
                .service_game
                .game
                .table
                .hands
                .0
                .get_mut(player)
                .unwrap();
            hand.sort_default();
        }

        self.sync_game_updated();

        self.save_and_return(&self.service_game.game.table.hands, "Error sorting hands")
            .await
    }

    pub async fn handle_user_set_game_settings(
        &mut self,
        player_id: &PlayerId,
        settings: &GameSettingsSummary,
    ) -> HttpResponse {
        if settings.fixed_settings {
            return HttpResponse::BadRequest().body("Cannot change fixed settings");
        }

        let existing_settings = self.service_game.settings.clone();
        self.service_game.settings = settings.to_game_settings(player_id, &existing_settings);

        let game_summary: UserPostSetGameSettingsResponse =
            ServiceGameSummary::from_service_game(&self.service_game, player_id).unwrap();

        self.sync_game_updated();

        self.save_and_return(game_summary, "Error setting game settings")
            .await
    }

    pub async fn handle_admin_draw_tile(&mut self) -> HttpResponse {
        self.service_game.game.draw_tile_from_wall();

        self.sync_game_updated();

        let current_player_id = self.service_game.game.get_current_player().clone();
        let hand = self
            .service_game
            .game
            .table
            .hands
            .0
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

        let tile_drawn = self.service_game.game.draw_tile_from_wall();

        match tile_drawn {
            DrawTileResult::WallExhausted | DrawTileResult::AlreadyDrawn => {
                return HttpResponse::BadRequest().body("Error when drawing tile");
            }
            DrawTileResult::Normal(_) | DrawTileResult::Bonus(_) => {}
        }

        self.sync_game_updated();

        let response: UserPostDrawTileResponse =
            ServiceGameSummary::from_service_game(&self.service_game, player_id).unwrap();

        self.save_and_return(&response, "Error when drawing tile")
            .await
    }

    pub async fn handle_user_pass_round(&mut self, player_id: &PlayerId) -> HttpResponse {
        let success = self.service_game.game.pass_null_round();

        if !success {
            return HttpResponse::BadRequest().body("Error when passing round");
        }

        self.sync_game_updated();

        let game_summary: UserPostPassRoundResponse =
            ServiceGameSummary::from_service_game(&self.service_game, player_id).unwrap();

        self.save_and_return(&game_summary, "Error passing round")
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

        self.sync_game_updated();

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

        self.sync_game_updated();

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

        self.sync_game_updated();

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

        self.sync_game_updated();

        self.save_and_return(&self.service_game.game, "Error with AI action")
            .await
    }

    pub async fn handle_user_move_player(&mut self, player_id: &PlayerId) -> HttpResponse {
        let current_player = self.service_game.game.get_current_player();
        let are_more_real = self.service_game.game.players.iter().any(|p_id| {
            let id = p_id.to_owned();
            let player = self.service_game.players.get(&id).unwrap();
            !player.is_ai && p_id != player_id
        });

        if are_more_real && current_player != player_id.clone() {
            return HttpResponse::BadRequest().body("Not your turn");
        }

        let success = self
            .service_game
            .game
            .round
            .next_turn(&self.service_game.game.table.hands);

        match success {
            Ok(()) => {
                self.sync_game_updated();

                let response: UserPostMovePlayerResponse =
                    ServiceGameSummary::from_service_game(&self.service_game, player_id).unwrap();

                self.save_and_return(response, "Error moving player").await
            }
            Err(_) => HttpResponse::BadRequest().body("Error when moving player"),
        }
    }

    pub async fn handle_discard_tile(&mut self, is_admin: bool, tile_id: &TileId) -> HttpResponse {
        let now_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        self.service_game
            .game
            .discard_tile_to_board(tile_id)
            .unwrap_or_default();

        let mut game = self.service_game.clone();

        game.settings.last_discard_time = now_time as i128;

        self.sync_game_updated();

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

        self.sync_game_updated();

        let hand = self
            .service_game
            .game
            .table
            .hands
            .0
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

        self.sync_game_updated();

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

        self.sync_game_updated();

        let hand = self
            .service_game
            .game
            .table
            .hands
            .0
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

        self.sync_game_updated();

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
            .next_turn(&self.service_game.game.table.hands);

        match success {
            Ok(_) => {
                self.sync_game_updated();

                let response: &AdminPostMovePlayerResponse = &self.service_game;

                self.save_and_return(response, "Error moving player").await
            }
            Err(_) => HttpResponse::BadRequest().body("Error when moving player"),
        }
    }

    pub async fn handle_user_sort_hand(
        &mut self,
        player_id: &PlayerId,
        tiles: &Option<Vec<TileId>>,
    ) -> HttpResponse {
        let hand = self
            .service_game
            .game
            .table
            .hands
            .0
            .get_mut(player_id)
            .unwrap();

        if tiles.is_none() {
            hand.sort_default();
        } else {
            let sorting_result = hand.sort_by_tiles(tiles.as_ref().unwrap());

            match sorting_result {
                Ok(_) => {}
                Err(_) => {
                    debug!("Failed to sort hand");
                }
            }
        }

        self.sync_game_updated();

        let response: UserPostSortHandResponse =
            ServiceGameSummary::from_service_game(&self.service_game, player_id).unwrap();

        debug!("Sorted hand for player: {:?}", player_id);

        self.save_and_return(&response, "Error sorting hand").await
    }

    pub async fn handle_admin_claim_tile(&mut self, player_id: &PlayerId) -> HttpResponse {
        let success = self.service_game.game.claim_tile(player_id);

        if success {
            self.sync_game_updated();
            let response: &AdminPostClaimTileResponse = &self.service_game;

            self.save_and_return(response, "Error claiming tile").await
        } else {
            HttpResponse::BadRequest().body("Error claiming tile")
        }
    }

    pub async fn handle_user_claim_tile(&mut self, player_id: &PlayerId) -> HttpResponse {
        let success = self.service_game.game.claim_tile(player_id);

        if success {
            self.sync_game_updated();

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

    fn sync_game_updated(&mut self) {
        self.service_game.updated_at = get_timestamp();
        self.service_game.game.update_version();
    }
}

fn create_game(
    player: &Option<ServicePlayer>,
    ai_player_names: &Option<Vec<String>>,
) -> ServiceGame {
    let mut players = Players::default();

    debug!("Going to create new game players");

    let mut game = Game {
        id: Uuid::new_v4().to_string(),
        name: "Custom Game".to_string(),
        ..Game::new(None)
    };

    for player_index in 0..Game::get_players_num(&game.style) {
        if player_index == 0 && player.is_some() {
            players.push(player.as_ref().unwrap().id.clone());
        } else {
            let id = Uuid::new_v4().to_string();

            players.push(id);
        }
    }

    game.set_players(&players);
    let mut players_set = FxHashMap::<String, ServicePlayer>::default();

    debug!("Going to add players to game");
    let empty_player_names = vec![];
    let player_names = ai_player_names.as_ref().unwrap_or(&empty_player_names);

    let timestamp = get_timestamp();

    for (index, game_player) in game.players.iter().enumerate() {
        if index == 0 && player.is_some() {
            let player_clone = player.as_ref().unwrap().clone();
            players_set.insert(player_clone.id.clone(), player_clone);
        } else {
            let default_name = format!("Player {}", index);
            let service_player = ServicePlayer {
                created_at: timestamp.to_string(),
                id: game_player.clone(),
                is_ai: true,
                name: player_names.get(index).unwrap_or(&default_name).clone(),
            };
            players_set.insert(game_player.clone(), service_player);
        }
    }

    let mut auto_stop_claim_meld = FxHashSet::<PlayerId>::default();

    if player.is_some() {
        auto_stop_claim_meld.insert(player.as_ref().unwrap().id.clone());
    }

    let settings = GameSettings {
        auto_stop_claim_meld,
        ..GameSettings::default()
    };

    ServiceGame {
        created_at: timestamp,
        game,
        players: players_set,
        settings,
        updated_at: timestamp,
    }
}
