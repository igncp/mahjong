use crate::auth::{AuthHandler, UnauthorizedError, UserRole};
use crate::game_wrapper::{CreateGameOpts, GameWrapper};
use crate::http_server::base::{get_lock, DataSocketServer, DataStorage, GamesManagerData};
use crate::service_error::{ResponseCommon, ServiceError};
use crate::user_wrapper::UserWrapper;
use actix_web::{get, patch, post, web, HttpRequest, HttpResponse};
use mahjong_core::GameId;
use service_contracts::{
    UserGetGamesQuery, UserGetGamesResponse, UserLoadGameQuery, UserPatchInfoRequest,
    UserPostAIContinueRequest, UserPostBreakMeldRequest, UserPostClaimTileRequest,
    UserPostCreateGameRequest, UserPostCreateMeldRequest, UserPostDiscardTileRequest,
    UserPostDrawTileRequest, UserPostMovePlayerRequest, UserPostPassRoundRequest,
    UserPostSayMahjongRequest, UserPostSetAuthAnonRequest, UserPostSetAuthRequest,
    UserPostSetAuthResponse, UserPostSetGameSettingsRequest, UserPostSortHandRequest,
};
use tracing::debug;

#[get("/game")]
async fn user_get_games(storage: DataStorage, req: HttpRequest) -> ResponseCommon {
    let params = web::Query::<UserGetGamesQuery>::from_query(req.query_string());
    if params.is_err() {
        return Ok(HttpResponse::BadRequest().body("Invalid player id"));
    }
    let player_id = params.unwrap().player_id.clone();

    AuthHandler::new(&storage, &req).verify_user(&player_id)?;

    let player = storage.get_player(&player_id).await;

    if player.is_err() || player.unwrap().is_none() {
        return Ok(HttpResponse::BadRequest().body("Invalid player id"));
    }

    let games = storage
        .get_player_games(&Some(player_id))
        .await
        .map_err(|_| ServiceError::Custom("Error getting games"))?;

    let response = UserGetGamesResponse(games);
    Ok(HttpResponse::Ok().json(response))
}

#[get("/dashboard")]
async fn user_get_dashboard(storage: DataStorage, req: HttpRequest) -> ResponseCommon {
    let auth_handler = AuthHandler::new(&storage, &req);

    let user_id = auth_handler.get_user_from_token()?;

    let user_wrapper = UserWrapper::from_storage(&storage, &user_id).await?;
    let auth_info_summary = auth_handler.get_auth_info_summary().await?;

    user_wrapper.get_dashboard(&auth_info_summary).await
}

#[get("/game/{game_id}")]
async fn user_get_game_load(
    storage: DataStorage,
    game_id: web::Path<String>,
    req: HttpRequest,
    srv: DataSocketServer,
) -> ResponseCommon {
    let params = web::Query::<UserLoadGameQuery>::from_query(req.query_string())
        .map_err(|_| ServiceError::Custom("Invalid player id"))?;

    AuthHandler::new(&storage, &req).verify_user(&params.player_id)?;

    // Here it can't use cache because the names might have changed
    let game_wrapper = GameWrapper::from_storage_no_cache(&storage, &game_id, srv, None).await?;

    game_wrapper.user_load_game(&params.player_id)
}

#[post("/game/{game_id}/discard-tile")]
async fn user_post_game_discard_tile(
    storage: DataStorage,
    body: web::Json<UserPostDiscardTileRequest>,
    game_id: web::Path<String>,
    req: HttpRequest,
    manager: GamesManagerData,
    srv: DataSocketServer,
) -> ResponseCommon {
    debug!("Discarding tile");
    get_lock!(manager, game_id);
    let mut game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await?;
    let current_user_id = &game_wrapper.get_current_player_id()?;
    AuthHandler::new(&storage, &req).verify_user(current_user_id)?;

    game_wrapper.handle_discard_tile(false, &body.tile_id).await
}

#[post("/game/{game_id}/ai-continue")]
async fn user_post_game_ai_continue(
    storage: DataStorage,
    game_id: web::Path<String>,
    body: web::Json<UserPostAIContinueRequest>,
    manager: GamesManagerData,
    req: HttpRequest,
    srv: DataSocketServer,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_user(&body.player_id)?;

    get_lock!(manager, game_id);

    let mut game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await?;

    game_wrapper.handle_user_ai_continue(&body).await
}

#[post("/game")]
async fn user_post_game_create(
    storage: DataStorage,
    srv: DataSocketServer,
    body: web::Json<UserPostCreateGameRequest>,
    req: HttpRequest,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_user(&body.player_id)?;

    debug!("Creating game for user: {:?}", &body.player_id);
    let create_game_opts = CreateGameOpts {
        ai_player_names: body.ai_player_names.as_ref(),
        auto_sort_own: body.auto_sort_own.as_ref(),
        dead_wall: body.dead_wall.as_ref(),
        player_id: Some(&body.player_id),
    };
    let game_wrapper = GameWrapper::from_new_game(&storage, srv, &create_game_opts).await?;

    debug!("Saving game for user: {:?}", &body.player_id);

    game_wrapper.handle_user_new_game(&body.player_id).await
}

#[post("/game/{game_id}/join")]
async fn user_post_game_join(
    storage: DataStorage,
    game_id: web::Path<GameId>,
    srv: DataSocketServer,
    req: HttpRequest,
    manager: GamesManagerData,
) -> ResponseCommon {
    let player_id = AuthHandler::new(&storage, &req).get_user_from_token()?;

    get_lock!(manager, game_id);

    let mut game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await?;

    game_wrapper.handle_user_join_game(&player_id).await
}

#[post("/game/{game_id}/draw-tile")]
async fn user_post_game_draw_tile(
    storage: DataStorage,
    game_id: web::Path<GameId>,
    body: web::Json<UserPostDrawTileRequest>,
    srv: DataSocketServer,
    req: HttpRequest,
    manager: GamesManagerData,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_user(&body.player_id)?;

    get_lock!(manager, game_id);

    let mut game_wrapper =
        GameWrapper::from_storage(&storage, &game_id, srv, Some(&body.game_version)).await?;

    game_wrapper.handle_user_draw_tile(&body.player_id).await
}

#[post("/game/{game_id}/move-player")]
async fn user_post_game_move_player(
    storage: DataStorage,
    game_id: web::Path<GameId>,
    body: web::Json<UserPostMovePlayerRequest>,
    srv: DataSocketServer,
    manager: GamesManagerData,
    req: HttpRequest,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_user(&body.player_id)?;

    get_lock!(manager, game_id);

    let mut game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await?;

    game_wrapper.handle_user_move_player(&body.player_id).await
}

#[post("/game/{game_id}/sort-hand")]
async fn user_post_game_sort_hand(
    storage: DataStorage,
    game_id: web::Path<GameId>,
    body: web::Json<UserPostSortHandRequest>,
    srv: DataSocketServer,
    req: HttpRequest,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_user(&body.player_id)?;

    debug!("Sorting hand for user: {:?}", &body.player_id);

    let mut game_wrapper =
        GameWrapper::from_storage(&storage, &game_id, srv, Some(&body.game_version)).await?;

    game_wrapper
        .handle_user_sort_hand(&body.player_id, &body.tiles)
        .await
}

#[post("/game/{game_id}/create-meld")]
async fn user_post_game_create_meld(
    storage: DataStorage,
    game_id: web::Path<GameId>,
    body: web::Json<UserPostCreateMeldRequest>,
    srv: DataSocketServer,
    manager: GamesManagerData,
    req: HttpRequest,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_user(&body.player_id)?;

    get_lock!(manager, game_id);

    let mut game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await?;

    game_wrapper.handle_user_create_meld(&body).await
}

#[post("/game/{game_id}/break-meld")]
async fn user_post_game_break_meld(
    storage: DataStorage,
    game_id: web::Path<GameId>,
    body: web::Json<UserPostBreakMeldRequest>,
    srv: DataSocketServer,
    manager: GamesManagerData,
    req: HttpRequest,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_user(&body.player_id)?;

    get_lock!(manager, game_id);

    let mut game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await?;

    game_wrapper.handle_user_break_meld(&body).await
}

#[post("/game/{game_id}/claim-tile")]
async fn user_post_game_claim_tile(
    storage: DataStorage,
    body: web::Json<UserPostClaimTileRequest>,
    game_id: web::Path<String>,
    srv: DataSocketServer,
    manager: GamesManagerData,
    req: HttpRequest,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_user(&body.player_id)?;

    get_lock!(manager, game_id);

    let mut game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await?;

    game_wrapper.handle_user_claim_tile(&body.player_id).await
}

#[post("/game/{game_id}/say-mahjong")]
async fn user_post_game_say_mahjong(
    storage: DataStorage,
    body: web::Json<UserPostSayMahjongRequest>,
    game_id: web::Path<String>,
    manager: GamesManagerData,
    srv: DataSocketServer,
    req: HttpRequest,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_user(&body.player_id)?;

    get_lock!(manager, game_id);

    let mut game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await?;

    game_wrapper.handle_user_say_mahjong(&body.player_id).await
}

#[post("/game/{game_id}/pass-round")]
async fn user_post_game_pass_round(
    storage: DataStorage,
    body: web::Json<UserPostPassRoundRequest>,
    game_id: web::Path<GameId>,
    manager: GamesManagerData,
    srv: DataSocketServer,
    req: HttpRequest,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_user(&body.player_id)?;

    get_lock!(manager, game_id);

    let mut game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await?;

    game_wrapper.handle_user_pass_round(&body.player_id).await
}

#[post("/game/{game_id}/settings")]
async fn user_post_game_settings(
    storage: DataStorage,
    body: web::Json<UserPostSetGameSettingsRequest>,
    game_id: web::Path<GameId>,
    manager: GamesManagerData,
    srv: DataSocketServer,
    req: HttpRequest,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_user(&body.player_id)?;

    get_lock!(manager, game_id);

    let mut game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await?;

    game_wrapper
        .handle_user_set_game_settings(&body.player_id, &body.settings)
        .await
}

#[post("/")]
async fn user_post_auth(
    storage: DataStorage,
    body: web::Json<UserPostSetAuthRequest>,
    req: HttpRequest,
) -> ResponseCommon {
    let username = body.username.clone();
    let username = username.to_lowercase();
    let mut auth_handler = AuthHandler::new(&storage, &req);

    let user = auth_handler
        .validate_email_user(&username, &body.password)
        .await
        .map_err(|_| UnauthorizedError)?;

    if user.is_none() {
        debug!("Creating new username: {username}");
        auth_handler
            .create_email_user(
                &username,
                &body.password,
                if username == "admin" {
                    UserRole::Admin
                } else {
                    UserRole::Player
                },
            )
            .await
            .map_err(|_| ServiceError::Custom("Error creating user"))?;

        let data = auth_handler
            .generate_token()
            .map_err(|_| ServiceError::Custom("Error generating token"))?;

        return Ok(HttpResponse::Ok().json(data));
    }

    debug!("Handling existing user: {username}");

    let is_valid = user.unwrap();

    if is_valid {
        let data = auth_handler.generate_token();

        if data.is_err() {
            let err = data.err().unwrap();
            debug!("Error generating token: {err}");
            return Ok(HttpResponse::InternalServerError().json("Error generating json"));
        }

        Ok(HttpResponse::Ok().json(data.unwrap()))
    } else {
        debug!("Invalid password for username: {username}");
        Ok(HttpResponse::Unauthorized().json("E_INVALID_USER_PASS"))
    }
}

#[post("/anonymous")]
async fn user_post_auth_anonymous(
    storage: DataStorage,
    body: web::Json<UserPostSetAuthAnonRequest>,
    req: HttpRequest,
) -> ResponseCommon {
    let id_token = body.id_token.clone();
    let mut auth_handler = AuthHandler::new(&storage, &req);

    let user = auth_handler.validate_anon_user(&id_token).await?;

    if user.is_none() {
        debug!("Creating new anonymous user");

        auth_handler
            .create_anonymous_user(&id_token, UserRole::Player)
            .await
            .map_err(|_| ServiceError::Custom("Error creating user"))?;

        let data: UserPostSetAuthResponse = auth_handler
            .generate_token()
            .map_err(|_| ServiceError::Custom("Error generating json"))?;

        return Ok(HttpResponse::Ok().json(data));
    }

    debug!("Handling existing anonymous user: {id_token}");

    let is_valid = user.unwrap();

    if is_valid {
        let data = auth_handler.generate_token();

        if data.is_err() {
            let err = data.err().unwrap();
            debug!("Error generating token: {err}");
            return Ok(HttpResponse::InternalServerError().json("Error generating json"));
        }

        Ok(HttpResponse::Ok().json(data.unwrap()))
    } else {
        debug!("Invalid anonymous token");
        Ok(HttpResponse::Unauthorized().json("E_INVALID_USER_PASS"))
    }
}

#[get("/info/{user_id}")]
async fn user_get_info(
    storage: DataStorage,
    req: HttpRequest,
    user_id: web::Path<String>,
) -> ResponseCommon {
    // For now only allow getting the information of the current user
    AuthHandler::new(&storage, &req).verify_user(&user_id)?;

    let user_wrapper = UserWrapper::from_storage(&storage, &user_id).await?;

    user_wrapper.get_info().await
}

#[patch("/info/{player_id}")]
async fn user_patch_info(
    storage: DataStorage,
    body: web::Json<UserPatchInfoRequest>,
    user_id: web::Path<String>,
    req: HttpRequest,
) -> ResponseCommon {
    // For now only allow getting the information of the current user
    AuthHandler::new(&storage, &req).verify_user(&user_id)?;

    let mut user_wrapper = UserWrapper::from_storage(&storage, &user_id).await?;

    user_wrapper.update_info(&body).await
}

pub fn get_user_scope() -> actix_web::Scope {
    web::scope("/v1/user")
        .service(user_get_dashboard)
        .service(user_get_game_load)
        .service(user_get_games)
        .service(user_get_info)
        .service(user_patch_info)
        .service(user_post_auth)
        .service(user_post_auth_anonymous)
        .service(user_post_game_ai_continue)
        .service(user_post_game_break_meld)
        .service(user_post_game_claim_tile)
        .service(user_post_game_create)
        .service(user_post_game_create_meld)
        .service(user_post_game_discard_tile)
        .service(user_post_game_draw_tile)
        .service(user_post_game_join)
        .service(user_post_game_move_player)
        .service(user_post_game_pass_round)
        .service(user_post_game_say_mahjong)
        .service(user_post_game_settings)
        .service(user_post_game_sort_hand)
}
