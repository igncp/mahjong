use crate::auth::{
    AuthHandler, AuthInfoData, GetAuthInfo, GithubAuth, GithubCallbackQuery, UserRole,
};
use crate::common::Storage;
use crate::env::ENV_FRONTEND_URL;
use crate::game_wrapper::GameWrapper;
use crate::games_loop::GamesLoop;
use crate::socket::MahjongWebsocketServer;
use crate::socket::MahjongWebsocketSession;
use crate::user_wrapper::UserWrapper;
use actix::prelude::*;
use actix_cors::Cors;
use actix_web::{
    get, patch, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use mahjong_core::deck::DEFAULT_DECK;
use mahjong_core::GameId;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use service_contracts::{
    AdminGetGamesResponse, AdminPostAIContinueRequest, AdminPostBreakMeldRequest,
    AdminPostClaimTileRequest, AdminPostCreateMeldRequest, AdminPostDiscardTileRequest,
    AdminPostSayMahjongRequest, GetDeckResponse, UserGetGamesQuery, UserGetGamesResponse,
    UserLoadGameQuery, UserPatchInfoRequest, UserPostAIContinueRequest, UserPostBreakMeldRequest,
    UserPostClaimTileRequest, UserPostCreateGameRequest, UserPostCreateMeldRequest,
    UserPostDiscardTileRequest, UserPostDrawTileRequest, UserPostMovePlayerRequest,
    UserPostPassRoundRequest, UserPostSayMahjongRequest, UserPostSetAuthAnonRequest,
    UserPostSetAuthRequest, UserPostSetGameSettingsRequest, UserPostSortHandRequest,
    WebSocketQuery,
};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tracing::{debug, warn};

pub type DataStorage = web::Data<Arc<Box<dyn Storage>>>;
pub type DataSocketServer = web::Data<Arc<Mutex<Addr<MahjongWebsocketServer>>>>;

pub struct GamesManager {
    games_locks: FxHashMap<GameId, Arc<Mutex<()>>>,
}

impl GamesManager {
    fn new() -> Self {
        Self {
            games_locks: FxHashMap::default(),
        }
    }

    pub fn get_game_mutex(&mut self, game_id: &GameId) -> Arc<Mutex<()>> {
        let mutex_arc = self
            .games_locks
            .entry(game_id.clone())
            .or_insert(Arc::new(Mutex::new(())));

        mutex_arc.clone()
    }
}

pub type GamesManagerData = web::Data<Arc<Mutex<GamesManager>>>;

#[get("/health")]
async fn get_health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/v1/admin/game")]
async fn admin_get_games(storage: DataStorage, req: HttpRequest) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_admin() {
        return AuthHandler::get_unauthorized();
    }

    let games = storage.get_player_games(&None).await;

    match games {
        Ok(games) => {
            let response: AdminGetGamesResponse = games;
            HttpResponse::Ok().json(response)
        }
        Err(_) => HttpResponse::InternalServerError().body("Error getting games"),
    }
}

#[get("/v1/deck")]
async fn get_deck() -> impl Responder {
    let response = GetDeckResponse(DEFAULT_DECK.clone().0);

    HttpResponse::Ok().json(response)
}

#[get("/v1/user/game")]
async fn user_get_games(storage: DataStorage, req: HttpRequest) -> impl Responder {
    let params = web::Query::<UserGetGamesQuery>::from_query(req.query_string());
    if params.is_err() {
        return HttpResponse::BadRequest().body("Invalid player id");
    }
    let player_id = params.unwrap().player_id.clone();
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_user(&player_id) {
        return AuthHandler::get_unauthorized();
    }

    let player = storage.get_player(&player_id).await;

    if player.is_err() || player.unwrap().is_none() {
        return HttpResponse::BadRequest().body("Invalid player id");
    }

    let games = storage.get_player_games(&Some(player_id)).await;

    match games {
        Ok(games) => {
            let response = UserGetGamesResponse(games);
            HttpResponse::Ok().json(response)
        }
        Err(_) => HttpResponse::InternalServerError().body("Error getting games"),
    }
}

#[get("/v1/user/dashboard")]
async fn user_get_dashboard(storage: DataStorage, req: HttpRequest) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    let user_id = auth_handler.get_user_from_token();

    if user_id.is_none() {
        return AuthHandler::get_unauthorized();
    }

    let user_id = user_id.unwrap();

    let user_wrapper = UserWrapper::from_storage(&storage, &user_id).await;
    let auth_info_summary = auth_handler.get_auth_info_summary().await;

    if auth_info_summary.is_none() {
        return HttpResponse::InternalServerError().body("Error loading user");
    }

    let auth_info_summary = auth_info_summary.unwrap();

    user_wrapper
        .unwrap()
        .get_dashboard(&auth_info_summary)
        .await
}

#[post("/v1/admin/game")]
async fn admin_post_game(
    storage: DataStorage,
    srv: DataSocketServer,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_admin() {
        return AuthHandler::get_unauthorized();
    }

    let game_wrapper = GameWrapper::from_new_game(&storage, srv, None, &None).await;

    if game_wrapper.is_err() {
        return HttpResponse::InternalServerError().body("Error creating game");
    }

    let game_wrapper = game_wrapper.unwrap();

    game_wrapper.handle_admin_new_game().await
}

#[get("/v1/admin/game/{game_id}")]
async fn admin_get_game_by_id(
    storage: DataStorage,
    manager: GamesManagerData,
    game_id: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_admin() {
        return AuthHandler::get_unauthorized();
    }

    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();

    let game = storage.get_game(&game_id.to_string(), true).await;

    match game {
        Ok(game) => HttpResponse::Ok().json(game),
        Err(_) => HttpResponse::InternalServerError().body("Error loading game"),
    }
}

#[get("/v1/user/game/{game_id}")]
async fn user_get_game_load(
    storage: DataStorage,
    game_id: web::Path<String>,
    req: HttpRequest,
    srv: DataSocketServer,
) -> impl Responder {
    let params = web::Query::<UserLoadGameQuery>::from_query(req.query_string());

    let player_id = match params {
        Ok(params) => params.player_id.clone(),
        Err(_) => return HttpResponse::BadRequest().body("Invalid player id"),
    };

    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_user(&player_id) {
        return AuthHandler::get_unauthorized();
    }

    // Here it can't use cache because the names might have changed
    let game_wrapper = GameWrapper::from_storage_no_cache(&storage, &game_id, srv, None).await;
    match game_wrapper {
        Ok(game_wrapper) => game_wrapper.user_load_game(&player_id),
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/sort-hands")]
async fn admin_post_game_sort_hands(
    manager: GamesManagerData,
    storage: DataStorage,
    game_id: web::Path<String>,
    srv: DataSocketServer,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_admin() {
        return AuthHandler::get_unauthorized();
    }

    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();

    let game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_sort_hands().await,
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/draw-tile")]
async fn admin_post_game_draw_tile(
    manager: GamesManagerData,
    storage: DataStorage,
    game_id: web::Path<GameId>,
    srv: DataSocketServer,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_admin() {
        return AuthHandler::get_unauthorized();
    }

    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();

    let game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_admin_draw_tile().await,
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/move-player")]
async fn admin_post_game_move_player(
    manager: GamesManagerData,
    storage: DataStorage,
    game_id: web::Path<String>,
    srv: DataSocketServer,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_admin() {
        return AuthHandler::get_unauthorized();
    }

    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();

    let game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_admin_move_player().await,
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/break-meld")]
async fn admin_post_game_break_meld(
    manager: GamesManagerData,
    storage: DataStorage,
    body: web::Json<AdminPostBreakMeldRequest>,
    game_id: web::Path<String>,
    srv: DataSocketServer,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_admin() {
        return AuthHandler::get_unauthorized();
    }

    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();

    let game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_admin_break_meld(&body).await,
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/create-meld")]
async fn admin_post_game_create_meld(
    manager: GamesManagerData,
    storage: DataStorage,
    body: web::Json<AdminPostCreateMeldRequest>,
    game_id: web::Path<String>,
    srv: DataSocketServer,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_admin() {
        return AuthHandler::get_unauthorized();
    }

    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();

    let game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_admin_create_meld(&body).await,
        Err(err) => err,
    }
}

#[post("/v1/user/game/{game_id}/discard-tile")]
async fn user_post_game_discard_tile(
    storage: DataStorage,
    body: web::Json<UserPostDiscardTileRequest>,
    game_id: web::Path<String>,
    req: HttpRequest,
    manager: GamesManagerData,
    srv: DataSocketServer,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);
    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();
    let game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await;

    match game_wrapper {
        Ok(mut game_wrapper) => {
            if !auth_handler.verify_user(&game_wrapper.get_current_player_id()) {
                return AuthHandler::get_unauthorized();
            }
            game_wrapper.handle_discard_tile(false, &body.tile_id).await
        }
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/discard-tile")]
async fn admin_post_game_discard_tile(
    manager: GamesManagerData,
    storage: DataStorage,
    body: web::Json<AdminPostDiscardTileRequest>,
    game_id: web::Path<String>,
    srv: DataSocketServer,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_admin() {
        return AuthHandler::get_unauthorized();
    }

    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();

    let game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_discard_tile(true, &body.tile_id).await,
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/claim-tile")]
async fn admin_post_game_claim_tile(
    manager: GamesManagerData,
    storage: DataStorage,
    body: web::Json<AdminPostClaimTileRequest>,
    game_id: web::Path<String>,
    srv: DataSocketServer,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_admin() {
        return AuthHandler::get_unauthorized();
    }

    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();

    let game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_admin_claim_tile(&body.player_id).await,
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/ai-continue")]
async fn admin_post_game_ai_continue(
    manager: GamesManagerData,
    storage: DataStorage,
    game_id: web::Path<String>,
    body: web::Json<AdminPostAIContinueRequest>,
    srv: DataSocketServer,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_admin() {
        return AuthHandler::get_unauthorized();
    }

    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();

    let game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_admin_ai_continue(&body).await,
        Err(err) => err,
    }
}

#[post("/v1/user/game/{game_id}/ai-continue")]
async fn user_post_game_ai_continue(
    storage: DataStorage,
    game_id: web::Path<String>,
    body: web::Json<UserPostAIContinueRequest>,
    manager: GamesManagerData,
    req: HttpRequest,
    srv: DataSocketServer,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_user(&body.player_id) {
        return AuthHandler::get_unauthorized();
    }

    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();

    let game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_user_ai_continue(&body).await,
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/say-mahjong")]
async fn admin_post_game_say_mahjong(
    manager: GamesManagerData,
    storage: DataStorage,
    body: web::Json<AdminPostSayMahjongRequest>,
    game_id: web::Path<String>,
    srv: DataSocketServer,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_admin() {
        return AuthHandler::get_unauthorized();
    }

    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();

    let game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_admin_say_mahjong(&body.player_id).await,
        Err(err) => err,
    }
}

#[post("/v1/user/game")]
async fn user_post_game_create(
    storage: DataStorage,
    srv: DataSocketServer,
    body: web::Json<UserPostCreateGameRequest>,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    debug!("Authenticating user: {:?}", &body.player_id);

    if !auth_handler.verify_user(&body.player_id) {
        return AuthHandler::get_unauthorized();
    }

    debug!("Creating game for user: {:?}", &body.player_id);
    let game_wrapper = GameWrapper::from_new_game(
        &storage,
        srv,
        Some(body.player_id.clone()),
        &body.ai_player_names,
    )
    .await;

    if game_wrapper.is_err() {
        debug!("Error preparing game");
        return HttpResponse::InternalServerError().body("Error preparing game");
    }

    let game_wrapper = game_wrapper.unwrap();

    debug!("Saving game for user: {:?}", &body.player_id);
    game_wrapper.handle_user_new_game(&body.player_id).await
}

#[post("/v1/user/game/{game_id}/draw-tile")]
async fn user_post_game_draw_tile(
    storage: DataStorage,
    game_id: web::Path<GameId>,
    body: web::Json<UserPostDrawTileRequest>,
    srv: DataSocketServer,
    req: HttpRequest,
    manager: GamesManagerData,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_user(&body.player_id) {
        return AuthHandler::get_unauthorized();
    }

    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();

    let game_wrapper =
        GameWrapper::from_storage(&storage, &game_id, srv, Some(&body.game_version)).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_user_draw_tile(&body.player_id).await,
        Err(err) => err,
    }
}

#[post("/v1/user/game/{game_id}/move-player")]
async fn user_post_game_move_player(
    storage: DataStorage,
    game_id: web::Path<GameId>,
    body: web::Json<UserPostMovePlayerRequest>,
    srv: DataSocketServer,
    manager: GamesManagerData,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_user(&body.player_id) {
        return AuthHandler::get_unauthorized();
    }

    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();

    let game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_user_move_player(&body.player_id).await,
        Err(err) => err,
    }
}

#[post("/v1/user/game/{game_id}/sort-hand")]
async fn user_post_game_sort_hand(
    storage: DataStorage,
    game_id: web::Path<GameId>,
    body: web::Json<UserPostSortHandRequest>,
    srv: DataSocketServer,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_user(&body.player_id) {
        return AuthHandler::get_unauthorized();
    }

    debug!("Sorting hand for user: {:?}", &body.player_id);

    let game_wrapper =
        GameWrapper::from_storage(&storage, &game_id, srv, Some(&body.game_version)).await;

    match game_wrapper {
        Ok(mut game_wrapper) => {
            game_wrapper
                .handle_user_sort_hand(&body.player_id, &body.tiles)
                .await
        }
        Err(err) => err,
    }
}

#[post("/v1/user/game/{game_id}/create-meld")]
async fn user_post_game_create_meld(
    storage: DataStorage,
    game_id: web::Path<GameId>,
    body: web::Json<UserPostCreateMeldRequest>,
    srv: DataSocketServer,
    manager: GamesManagerData,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_user(&body.player_id) {
        return AuthHandler::get_unauthorized();
    }

    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();

    let game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_user_create_meld(&body).await,
        Err(err) => err,
    }
}

#[post("/v1/user/game/{game_id}/break-meld")]
async fn user_post_game_break_meld(
    storage: DataStorage,
    game_id: web::Path<GameId>,
    body: web::Json<UserPostBreakMeldRequest>,
    srv: DataSocketServer,
    manager: GamesManagerData,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_user(&body.player_id) {
        return AuthHandler::get_unauthorized();
    }

    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();

    let game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_user_break_meld(&body).await,
        Err(err) => err,
    }
}

#[post("/v1/user/game/{game_id}/claim-tile")]
async fn user_post_game_claim_tile(
    storage: DataStorage,
    body: web::Json<UserPostClaimTileRequest>,
    game_id: web::Path<String>,
    srv: DataSocketServer,
    manager: GamesManagerData,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_user(&body.player_id) {
        return AuthHandler::get_unauthorized();
    }

    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();

    let game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_user_claim_tile(&body.player_id).await,
        Err(err) => err,
    }
}

#[post("/v1/user/game/{game_id}/say-mahjong")]
async fn user_post_game_say_mahjong(
    storage: DataStorage,
    body: web::Json<UserPostSayMahjongRequest>,
    game_id: web::Path<String>,
    manager: GamesManagerData,
    srv: DataSocketServer,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_user(&body.player_id) {
        return AuthHandler::get_unauthorized();
    }

    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();

    let game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_user_say_mahjong(&body.player_id).await,
        Err(err) => err,
    }
}

#[post("/v1/user/game/{game_id}/pass-round")]
async fn user_post_game_pass_round(
    storage: DataStorage,
    body: web::Json<UserPostPassRoundRequest>,
    game_id: web::Path<GameId>,
    manager: GamesManagerData,
    srv: DataSocketServer,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_user(&body.player_id) {
        return AuthHandler::get_unauthorized();
    }

    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();

    let game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_user_pass_round(&body.player_id).await,
        Err(err) => err,
    }
}

#[post("/v1/user/game/{game_id}/settings")]
async fn user_post_game_settings(
    storage: DataStorage,
    body: web::Json<UserPostSetGameSettingsRequest>,
    game_id: web::Path<GameId>,
    manager: GamesManagerData,
    srv: DataSocketServer,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_user(&body.player_id) {
        return AuthHandler::get_unauthorized();
    }

    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();

    let game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await;

    match game_wrapper {
        Ok(mut game_wrapper) => {
            game_wrapper
                .handle_user_set_game_settings(&body.player_id, &body.settings)
                .await
        }
        Err(err) => err,
    }
}

#[post("/v1/user")]
async fn user_post_auth(
    storage: DataStorage,
    body: web::Json<UserPostSetAuthRequest>,
    req: HttpRequest,
) -> impl Responder {
    let username = body.username.clone();
    let username = username.to_lowercase();
    let mut auth_handler = AuthHandler::new(&storage, &req);

    let user = auth_handler
        .validate_email_user(&username, &body.password)
        .await;

    if user.is_err() {
        return HttpResponse::Unauthorized().finish();
    }

    let user = user.unwrap();

    if user.is_none() {
        debug!("Creating new username: {username}");
        let result = auth_handler
            .create_email_user(
                &username,
                &body.password,
                if username == "admin" {
                    UserRole::Admin
                } else {
                    UserRole::Player
                },
            )
            .await;

        if result.is_err() {
            return HttpResponse::InternalServerError().json("Error creating user");
        }

        let data = auth_handler.generate_token();

        if data.is_err() {
            return HttpResponse::InternalServerError().json("Error generating json");
        }

        return HttpResponse::Ok().json(data.unwrap());
    }

    debug!("Handling existing user: {username}");

    let is_valid = user.unwrap();

    if is_valid {
        let data = auth_handler.generate_token();

        if data.is_err() {
            let err = data.err().unwrap();
            debug!("Error generating token: {err}");
            return HttpResponse::InternalServerError().json("Error generating json");
        }

        HttpResponse::Ok().json(data.unwrap())
    } else {
        debug!("Invalid password for username: {username}");
        HttpResponse::Unauthorized().json("E_INVALID_USER_PASS")
    }
}

#[post("/v1/user-anonymous")]
async fn user_post_auth_anonymous(
    storage: DataStorage,
    body: web::Json<UserPostSetAuthAnonRequest>,
    req: HttpRequest,
) -> impl Responder {
    let id_token = body.id_token.clone();
    let mut auth_handler = AuthHandler::new(&storage, &req);

    let user = auth_handler.validate_anon_user(&id_token).await;

    if user.is_err() {
        return HttpResponse::Unauthorized().finish();
    }

    let user = user.unwrap();

    if user.is_none() {
        debug!("Creating new anonymous user");

        let result = auth_handler
            .create_anonymous_user(&id_token, UserRole::Player)
            .await;

        if result.is_err() {
            return HttpResponse::InternalServerError().json("Error creating user");
        }

        let data = auth_handler.generate_token();

        if data.is_err() {
            return HttpResponse::InternalServerError().json("Error generating json");
        }

        return HttpResponse::Ok().json(data.unwrap());
    }

    debug!("Handling existing anonymous user: {id_token}");

    let is_valid = user.unwrap();

    if is_valid {
        let data = auth_handler.generate_token();

        if data.is_err() {
            let err = data.err().unwrap();
            debug!("Error generating token: {err}");
            return HttpResponse::InternalServerError().json("Error generating json");
        }

        HttpResponse::Ok().json(data.unwrap())
    } else {
        debug!("Invalid anonymous token");
        HttpResponse::Unauthorized().json("E_INVALID_USER_PASS")
    }
}

#[get("/v1/user/info/{user_id}")]
async fn user_get_info(
    storage: DataStorage,
    req: HttpRequest,
    user_id: web::Path<String>,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    // For now only allow getting the information of the current user
    if !auth_handler.verify_user(&user_id) {
        return AuthHandler::get_unauthorized();
    }

    let user_wrapper = UserWrapper::from_storage(&storage, &user_id).await;

    if user_wrapper.is_err() {
        return HttpResponse::InternalServerError().json("Error getting user info");
    }

    user_wrapper.unwrap().get_info().await
}

#[patch("/v1/user/info/{player_id}")]
async fn user_patch_info(
    storage: DataStorage,
    body: web::Json<UserPatchInfoRequest>,
    user_id: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    // For now only allow getting the information of the current user
    if !auth_handler.verify_user(&user_id) {
        return AuthHandler::get_unauthorized();
    }

    let user_wrapper = UserWrapper::from_storage(&storage, &user_id).await;

    if user_wrapper.is_err() {
        return HttpResponse::InternalServerError().json("Error getting user info");
    }

    user_wrapper.unwrap().update_info(&body).await
}

#[get("/v1/github_callback")]
async fn github_callback(req: HttpRequest, storage: DataStorage) -> Result<impl Responder, Error> {
    let query = web::Query::<GithubCallbackQuery>::from_query(req.query_string());

    if query.is_err() {
        return Ok(HttpResponse::BadRequest().json("Invalid query"));
    }

    let query = query.unwrap();

    let mut auth_handler = AuthHandler::new(&storage, &req);
    let result = GithubAuth::handle_callback(query, &storage, &mut auth_handler).await;

    if result.is_none() {
        return Ok(HttpResponse::InternalServerError().json("Error handling callback"));
    }

    let response_body = result.unwrap();
    let response_qs = serde_qs::to_string(&response_body).unwrap();

    let frontend_url = std::env::var(ENV_FRONTEND_URL).unwrap();
    let redirect = HttpResponse::Found()
        .append_header(("Location", format!("{}?{}", frontend_url, response_qs)))
        .finish();

    Ok(redirect)
}

#[get("/v1/ws")]
async fn get_ws(
    req: HttpRequest,
    stream: web::Payload,
    srv: DataSocketServer,
    storage: DataStorage,
) -> Result<impl Responder, Error> {
    let params = web::Query::<WebSocketQuery>::from_query(req.query_string());

    if params.is_err() {
        return Ok(HttpResponse::BadRequest().body("Invalid query parameters"));
    }

    let params = params.unwrap();
    let game_id = params.game_id.clone();
    let player_id = params.player_id.clone();

    let auth_handler = AuthHandler::new(&storage, &req);

    if (player_id.is_some()
        && !auth_handler.verify_user_token(&player_id.clone().unwrap(), &params.token))
        || (player_id.is_none() && !auth_handler.verify_admin_token(&params.token))
    {
        return Ok(AuthHandler::get_unauthorized());
    }

    let addr = loop {
        if let Ok(srv) = srv.lock() {
            break srv.clone();
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    };

    ws::start(
        MahjongWebsocketSession {
            addr,
            hb: Instant::now(),
            id: rand::random(),
            room: MahjongWebsocketSession::get_room_id(&game_id, player_id.as_ref()),
        },
        &req,
        stream,
    )
}

#[post("/v1/test/delete-games")]
async fn test_post_delete_games(
    req: HttpRequest,
    storage: DataStorage,
) -> Result<impl Responder, Error> {
    let auth_handler = AuthHandler::new(&storage, &req);

    let user_id = auth_handler.get_user_from_token();

    if user_id.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let user_id = user_id.unwrap();

    // If deleting for normal users, should check if any active running at the moment by using
    // the web socket

    let auth_info = storage
        .get_auth_info(GetAuthInfo::PlayerId(user_id.clone()))
        .await;

    if auth_info.is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let auth_info = auth_info.unwrap();

    if auth_info.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let auth_info = auth_info.unwrap();

    if let AuthInfoData::Email(auth_info_email) = auth_info.data {
        if auth_info_email.username != "test" {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    } else {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let games = storage.get_player_games(&Some(user_id.clone())).await;
    if games.is_err() {
        return Ok(HttpResponse::InternalServerError().finish());
    }
    let games = games.unwrap();
    let games_ids: Vec<_> = games.iter().map(|g| g.id.clone()).collect();

    let result = storage.delete_games(&games_ids).await;

    if result.is_err() {
        return Ok(HttpResponse::InternalServerError().finish());
    }

    #[derive(Deserialize, Serialize)]
    struct DeleteGames {
        test_delete_games: bool,
    }

    Ok(HttpResponse::Ok().json({
        DeleteGames {
            test_delete_games: true,
        }
    }))
}

pub struct MahjongServer;

impl MahjongServer {
    pub async fn start(storage: Box<dyn Storage>) -> std::io::Result<()> {
        let port = 3000;
        let address = "0.0.0.0";

        warn!("Starting the Mahjong HTTP server on port http://{address}:{port}");

        let games_manager = GamesManager::new();
        let games_manager_arc = Arc::new(Mutex::new(games_manager));
        let loop_games_manager_arc = games_manager_arc.clone();
        let storage_arc = Arc::new(storage);
        let loop_storage_arc = storage_arc.clone();
        let socket_server = Arc::new(Mutex::new(MahjongWebsocketServer::new().start()));
        let loop_socket_server = socket_server.clone();

        GamesLoop::new(loop_storage_arc, loop_socket_server, loop_games_manager_arc).run();

        HttpServer::new(move || {
            let storage_data: DataStorage = web::Data::new(storage_arc.clone());
            let games_manager_data = web::Data::new(games_manager_arc.clone());
            let cors = Cors::permissive();
            let endpoints_server = socket_server.clone();

            App::new()
                .app_data(games_manager_data)
                .app_data(storage_data)
                .app_data(web::Data::new(endpoints_server))
                .service(admin_get_game_by_id)
                .service(admin_get_games)
                .service(admin_post_game)
                .service(admin_post_game_ai_continue)
                .service(admin_post_game_break_meld)
                .service(admin_post_game_claim_tile)
                .service(admin_post_game_create_meld)
                .service(admin_post_game_discard_tile)
                .service(admin_post_game_draw_tile)
                .service(admin_post_game_move_player)
                .service(admin_post_game_say_mahjong)
                .service(admin_post_game_sort_hands)
                .service(get_deck)
                .service(get_health)
                .service(get_ws)
                .service(github_callback)
                .service(user_get_game_load)
                .service(user_get_games)
                .service(user_get_info)
                .service(user_get_dashboard)
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
                .service(user_post_game_move_player)
                .service(user_post_game_pass_round)
                .service(user_post_game_say_mahjong)
                .service(user_post_game_settings)
                .service(user_post_game_sort_hand)
                .service(test_post_delete_games)
                .wrap(cors)
        })
        .bind((address, port))?
        .run()
        .await
    }
}
