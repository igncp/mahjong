use crate::{
    ai_wrapper::AIWrapper,
    http_server::{DataSocketServer, DataStorage},
    service_error::{ResponseCommon, ServiceError},
    socket::{MahjongWebsocketSession, SocketClientMessage},
    time::get_timestamp,
};
use actix_web::{web, HttpResponse};
use mahjong_core::{
    game::{DrawTileResult, GameVersion},
    Game, GamePhase, PlayerId, Players, TileId,
};
use rustc_hash::FxHashMap;
use service_contracts::{
    AdminPostAIContinueRequest, AdminPostAIContinueResponse, AdminPostBreakMeldRequest,
    AdminPostBreakMeldResponse, AdminPostClaimTileResponse, AdminPostCreateMeldRequest,
    AdminPostCreateMeldResponse, AdminPostDiscardTileResponse, AdminPostDrawTileResponse,
    AdminPostMovePlayerResponse, AdminPostSayMahjongResponse, GameSettings, GameSettingsSummary,
    ServiceGame, ServiceGameSummary, ServicePlayer, SocketMessage, UserPostAIContinueRequest,
    UserPostAIContinueResponse, UserPostBreakMeldRequest, UserPostBreakMeldResponse,
    UserPostCreateGameResponse, UserPostCreateMeldRequest, UserPostCreateMeldResponse,
    UserPostDiscardTileResponse, UserPostDrawTileResponse, UserPostJoinGameResponse,
    UserPostMovePlayerResponse, UserPostPassRoundResponse, UserPostSayMahjongResponse,
    UserPostSetGameSettingsResponse, UserPostSortHandResponse,
};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;

pub struct GameWrapper<'a> {
    service_game: ServiceGame,
    socket_server: DataSocketServer,
    storage: &'a DataStorage,
}

#[derive(Default)]
pub struct CreateGameOpts<'a> {
    pub ai_player_names: Option<&'a Vec<String>>,
    pub auto_sort_own: Option<&'a bool>,
    pub dead_wall: Option<&'a bool>,
    pub player_id: Option<&'a PlayerId>,
}

impl<'a> GameWrapper<'a> {
    async fn from_storage_base(
        storage: &'a DataStorage,
        game_id: &web::Path<String>,
        socket_server: DataSocketServer,
        game_version: Option<&'a GameVersion>,
        use_cache: bool,
    ) -> Result<GameWrapper<'a>, ServiceError> {
        let game = storage.get_game(&game_id.to_string(), use_cache).await;

        if game.is_err() {
            return Err(ServiceError::ErrorLoadingGame);
        }

        let game_content = game.unwrap();

        if game_content.is_none() {
            return Err(ServiceError::GameNotFound);
        }

        let service_game = game_content.unwrap();

        if game_version.is_some() && service_game.game.version != *game_version.unwrap() {
            return Err(ServiceError::GameVersionMismatch);
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
    ) -> Result<GameWrapper<'a>, ServiceError> {
        Self::from_storage_base(storage, game_id, socket_server, game_version, true).await
    }

    pub async fn from_storage_no_cache(
        storage: &'a DataStorage,
        game_id: &web::Path<String>,
        socket_server: DataSocketServer,
        game_version: Option<&'a GameVersion>,
    ) -> Result<GameWrapper<'a>, ServiceError> {
        Self::from_storage_base(storage, game_id, socket_server, game_version, false).await
    }

    pub async fn from_new_game(
        storage: &'a DataStorage,
        socket_server: DataSocketServer,
        opts: &'a CreateGameOpts<'a>,
    ) -> Result<GameWrapper<'a>, ServiceError> {
        let service_player = if let Some(player_id) = opts.player_id {
            let player_content = storage
                .get_player(player_id)
                .await
                .map_err(|_| {
                    debug!("Player not found with error");
                    ServiceError::Custom("Player not found")
                })?
                .ok_or_else(|| {
                    debug!("Player not found with none");
                    ServiceError::Custom("Player not found")
                })?;

            Some(player_content)
        } else {
            None
        };
        let service_game = create_game(&service_player, opts);

        Ok(Self {
            storage,
            service_game,
            socket_server,
        })
    }

    pub async fn handle_admin_say_mahjong(&mut self, player_id: &PlayerId) -> ResponseCommon {
        self.service_game
            .game
            .say_mahjong(player_id)
            .map_err(|_| ServiceError::Custom("Error saying mahjong"))?;

        self.sync_game_updated();

        let response: &AdminPostSayMahjongResponse = &self.service_game;

        self.save_and_return(response, "Error saying mahjong").await
    }

    pub async fn handle_user_say_mahjong(&mut self, player_id: &PlayerId) -> ResponseCommon {
        self.service_game
            .game
            .say_mahjong(player_id)
            .map_err(|_| ServiceError::Custom("Error saying mahjong"))?;

        self.sync_game_updated();

        let game = ServiceGameSummary::from_service_game(&self.service_game, player_id).unwrap();
        let response = UserPostSayMahjongResponse(game);

        self.save_and_return(&response, "Error saying mahjong")
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

    async fn save_and_return<A>(&self, data: A, err_msg: &'static str) -> ResponseCommon
    where
        A: serde::Serialize,
    {
        self.storage
            .save_game(&self.service_game)
            .await
            .map_err(|_| ServiceError::Custom(err_msg))?;

        self.sync_game();

        Ok(HttpResponse::Ok().json(data))
    }

    pub async fn handle_admin_new_game(&self) -> ResponseCommon {
        self.save_and_return(&self.service_game, "Error creating game")
            .await
    }

    pub async fn handle_user_new_game(&self, player_id: &PlayerId) -> ResponseCommon {
        let game_summary = ServiceGameSummary::from_service_game(&self.service_game, player_id)
            .ok_or(ServiceError::Custom("Error creating game"))?;
        let response = UserPostCreateGameResponse(game_summary);

        self.save_and_return(response, "Error creating game").await
    }

    pub fn user_load_game(&self, player_id: &PlayerId) -> ResponseCommon {
        match ServiceGameSummary::from_service_game(&self.service_game, player_id) {
            None => Err(ServiceError::ErrorLoadingGame.into()),
            Some(summary) => Ok(HttpResponse::Ok().json(summary)),
        }
    }

    pub async fn handle_sort_hands(&mut self) -> ResponseCommon {
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
    ) -> ResponseCommon {
        if settings.fixed_settings {
            return Ok(HttpResponse::BadRequest().body("Cannot change fixed settings"));
        }

        let existing_settings = self.service_game.settings.clone();
        self.service_game.settings = settings.to_game_settings(player_id, &existing_settings);

        let game_summary =
            ServiceGameSummary::from_service_game(&self.service_game, player_id).unwrap();
        let response = UserPostSetGameSettingsResponse(game_summary);

        self.sync_game_updated();

        self.save_and_return(response, "Error setting game settings")
            .await
    }

    pub async fn handle_admin_draw_tile(&mut self) -> ResponseCommon {
        self.service_game.game.draw_tile_from_wall();

        self.sync_game_updated();

        let current_player_id = self
            .service_game
            .game
            .get_current_player()
            .ok_or(ServiceError::Custom("Error getting current player"))?;

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

    pub async fn handle_user_draw_tile(&mut self, player_id: &PlayerId) -> ResponseCommon {
        let current_player = self
            .service_game
            .game
            .get_current_player()
            .ok_or(ServiceError::Custom("Error getting current player"))?;

        if &current_player != player_id {
            return Ok(HttpResponse::BadRequest().body("Not your turn"));
        }

        let tile_drawn = self.service_game.game.draw_tile_from_wall();

        match tile_drawn {
            DrawTileResult::WallExhausted | DrawTileResult::AlreadyDrawn => {
                return Ok(HttpResponse::BadRequest().body("Error when drawing tile"));
            }
            DrawTileResult::Normal(_) | DrawTileResult::Bonus(_) => {}
        }

        self.sync_game_updated();

        let game = ServiceGameSummary::from_service_game(&self.service_game, player_id).unwrap();
        let response = UserPostDrawTileResponse(game);

        self.save_and_return(&response, "Error when drawing tile")
            .await
    }

    pub async fn handle_user_pass_round(&mut self, player_id: &PlayerId) -> ResponseCommon {
        self.service_game
            .game
            .pass_null_round()
            .map_err(|_| ServiceError::Custom("Error passing round"))?;

        self.sync_game_updated();

        let game_summary =
            ServiceGameSummary::from_service_game(&self.service_game, player_id).unwrap();
        let response = UserPostPassRoundResponse(game_summary);

        self.save_and_return(&response, "Error passing round").await
    }

    pub async fn handle_admin_ai_continue(
        &mut self,
        body: &AdminPostAIContinueRequest,
    ) -> ResponseCommon {
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
    ) -> ResponseCommon {
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

    pub async fn handle_server_ai_continue(&mut self) -> ResponseCommon {
        if !self.service_game.settings.ai_enabled {
            // This response is not used
            return Ok(HttpResponse::BadRequest().body("AI disabled"));
        }

        let mut standard_ai = AIWrapper::new(&mut self.service_game, None);

        standard_ai.play_action();

        self.sync_game_updated();

        self.save_and_return(&self.service_game.game, "Error with AI action")
            .await
    }

    pub async fn handle_user_move_player(&mut self, player_id: &PlayerId) -> ResponseCommon {
        let current_player = self
            .service_game
            .game
            .get_current_player()
            .ok_or(ServiceError::Custom("Error getting current player"))?;

        let are_more_real = self.service_game.game.players.iter().any(|p_id| {
            let id = p_id.to_owned();
            let player = self.service_game.players.get(&id).unwrap();
            !player.is_ai && p_id != player_id
        });

        if are_more_real && current_player != player_id.clone() {
            return Ok(HttpResponse::BadRequest().body("Not your turn"));
        }

        self.service_game
            .game
            .round
            .next_turn(&self.service_game.game.table.hands)
            .map_err(|_| ServiceError::Custom("Error moving player"))?;

        self.sync_game_updated();

        let game_summary =
            ServiceGameSummary::from_service_game(&self.service_game, player_id).unwrap();
        let response = UserPostMovePlayerResponse(game_summary);

        self.save_and_return(response, "Error moving player").await
    }

    pub async fn handle_discard_tile(
        &mut self,
        is_admin: bool,
        tile_id: &TileId,
    ) -> ResponseCommon {
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
            let player_id = self
                .service_game
                .game
                .get_current_player()
                .ok_or(ServiceError::Custom("Error getting current player"))?;

            let game_summary = ServiceGameSummary::from_service_game(&game, &player_id).unwrap();
            let response = UserPostDiscardTileResponse(game_summary);

            self.save_and_return(&response, "Error when discarding the tile")
                .await
        }
    }

    pub async fn handle_admin_break_meld(
        &mut self,
        body: &AdminPostBreakMeldRequest,
    ) -> ResponseCommon {
        self.service_game
            .game
            .break_meld(&body.player_id, &body.set_id)
            .map_err(|_| ServiceError::Custom("Error when breaking meld"))?;

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
    ) -> ResponseCommon {
        self.service_game
            .game
            .break_meld(&body.player_id, &body.set_id)
            .map_err(|_| ServiceError::Custom("Error when breaking meld"))?;

        self.sync_game_updated();

        let game =
            ServiceGameSummary::from_service_game(&self.service_game, &body.player_id).unwrap();
        let response = UserPostBreakMeldResponse(game);

        self.save_and_return(&response, "Error when breaking meld")
            .await
    }

    pub async fn handle_admin_create_meld(
        &mut self,
        body: &AdminPostCreateMeldRequest,
    ) -> ResponseCommon {
        self.service_game
            .game
            .create_meld(
                &body.player_id,
                &body.tiles.clone().into_iter().collect::<Vec<TileId>>(),
                body.is_upgrade,
                body.is_concealed,
            )
            .map_err(|_| ServiceError::Custom("Error when creating meld"))?;

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
    ) -> ResponseCommon {
        self.service_game
            .game
            .create_meld(
                &body.player_id,
                &body.tiles.clone().into_iter().collect::<Vec<TileId>>(),
                body.is_upgrade,
                body.is_concealed,
            )
            .map_err(|_| ServiceError::Custom("Error when creating meld"))?;

        self.sync_game_updated();

        let game =
            ServiceGameSummary::from_service_game(&self.service_game, &body.player_id).unwrap();
        let response = UserPostCreateMeldResponse(game);

        self.save_and_return(&response, "Error when creating meld")
            .await
    }

    pub async fn handle_admin_move_player(&mut self) -> ResponseCommon {
        let success = self
            .service_game
            .game
            .round
            .next_turn(&self.service_game.game.table.hands);

        match success {
            Ok(_) => {
                self.sync_game_updated();

                let response = AdminPostMovePlayerResponse(self.service_game.clone());

                self.save_and_return(response, "Error moving player").await
            }
            Err(_) => Ok(HttpResponse::BadRequest().body("Error when moving player")),
        }
    }

    pub async fn handle_user_sort_hand(
        &mut self,
        player_id: &PlayerId,
        tiles: &Option<Vec<TileId>>,
    ) -> ResponseCommon {
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
            hand.sort_by_tiles(tiles.as_ref().unwrap())
                .map_err(|_| ServiceError::Custom("Error sorting hand"))?;
        }

        self.sync_game_updated();

        let game_summary =
            ServiceGameSummary::from_service_game(&self.service_game, player_id).unwrap();
        let response = UserPostSortHandResponse(game_summary);

        debug!("Sorted hand for player: {:?}", player_id);

        self.save_and_return(&response, "Error sorting hand").await
    }

    pub async fn handle_admin_claim_tile(&mut self, player_id: &PlayerId) -> ResponseCommon {
        let success = self.service_game.game.claim_tile(player_id);

        if success {
            self.sync_game_updated();
            let response = AdminPostClaimTileResponse(self.service_game.to_owned());

            self.save_and_return(response, "Error claiming tile").await
        } else {
            Ok(HttpResponse::BadRequest().body("Error claiming tile"))
        }
    }

    pub async fn handle_user_claim_tile(&mut self, player_id: &PlayerId) -> ResponseCommon {
        let success = self.service_game.game.claim_tile(player_id);

        if success {
            self.sync_game_updated();

            let response =
                ServiceGameSummary::from_service_game(&self.service_game, player_id).unwrap();

            self.save_and_return(response, "Error claiming tile").await
        } else {
            Ok(HttpResponse::BadRequest().body("Error claiming tile"))
        }
    }

    pub fn get_current_player_id(&self) -> Result<PlayerId, ServiceError> {
        self.service_game
            .game
            .get_current_player()
            .ok_or(ServiceError::Custom("No current player"))
    }

    pub async fn handle_user_join_game(&mut self, player_id: &PlayerId) -> ResponseCommon {
        if self.service_game.game.players.0.contains(player_id) {
            return Err(ServiceError::Custom("Player already in game").into());
        }

        if self.service_game.game.players.len() >= 4 {
            return Err(ServiceError::Custom("Game is full").into());
        }

        if self.service_game.game.phase != GamePhase::WaitingPlayers {
            return Err(ServiceError::Custom("Game is empty").into());
        }

        self.service_game.game.players.push(player_id.clone());

        if !self.service_game.settings.auto_sort_players.is_empty() {
            self.service_game
                .settings
                .auto_sort_players
                .insert(player_id.clone());
        }

        self.sync_game_updated();

        let response = UserPostJoinGameResponse(player_id.clone());

        self.save_and_return(response, "Error joining game").await
    }

    fn sync_game_updated(&mut self) {
        self.service_game.updated_at = get_timestamp();
        self.service_game.game.update_version();
    }
}

fn create_game(player: &Option<ServicePlayer>, opts: &CreateGameOpts) -> ServiceGame {
    let mut players = Players::default();

    debug!("Going to create new game players");

    let mut game = Game {
        name: "Custom Game".to_string(),
        ..Game::new(None)
    };

    game.update_id(None);

    if player.is_some() {
        players.push(player.as_ref().unwrap().id.clone());
    }

    for _ in opts.ai_player_names.unwrap_or(&vec![]).iter() {
        let id = Players::new_player();

        players.push(id);
    }

    game.players = players;
    let mut players_set = FxHashMap::<String, ServicePlayer>::default();

    debug!("Going to add players to game");
    let empty_player_names = vec![];
    let player_names = opts.ai_player_names.unwrap_or(&empty_player_names);

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
                name: player_names.get(index + 1).unwrap_or(&default_name).clone(),
            };
            players_set.insert(game_player.clone(), service_player);
        }
    }

    let mut settings = GameSettings::default();

    if let Some(dead_wall) = opts.dead_wall {
        settings.dead_wall = *dead_wall;
    }

    if let Some(player) = player {
        if let Some(auto_sort_own) = opts.auto_sort_own {
            if *auto_sort_own {
                settings.auto_sort_players.insert(player.id.clone());
            }
        }
        settings.auto_stop_claim_meld.insert(player.id.clone());
    }

    ServiceGame {
        created_at: timestamp,
        game,
        players: players_set,
        settings,
        updated_at: timestamp,
    }
}
