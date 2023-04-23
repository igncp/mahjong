use crate::auth::{AuthHandler, UserRole};
use crate::common::Storage;
use crate::game_wrapper::GameWrapper;
use crate::games_loop::GamesLoop;
use crate::socket_server::MahjongWebsocketServer;
use crate::socket_session::MahjongWebsocketSession;
use actix::prelude::*;
use actix_cors::Cors;
use actix_web::{get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use mahjong_core::GameId;
use service_contracts::{
    AdminGetGamesResponse, AdminPostAIContinueRequest, AdminPostBreakMeldRequest,
    AdminPostClaimTileRequest, AdminPostCreateMeldRequest, AdminPostDiscardTileRequest,
    AdminPostSayMahjongRequest, AdminPostSwapDrawTilesRequest, UserGetGamesQuery,
    UserGetGamesResponse, UserLoadGameQuery, UserPostAIContinueRequest, UserPostBreakMeldRequest,
    UserPostClaimTileRequest, UserPostCreateGameRequest, UserPostCreateMeldRequest,
    UserPostDiscardTileRequest, UserPostDrawTileRequest, UserPostMovePlayerRequest,
    UserPostSayMahjongRequest, UserPostSetAuthRequest, UserPostSetGameSettingsRequest,
    UserPostSortHandRequest, WebSocketQuery,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub type StorageData = web::Data<Arc<Box<dyn Storage>>>;
pub type SocketServer = web::Data<Arc<Mutex<Addr<MahjongWebsocketServer>>>>;

pub struct GamesManager {
    games_locks: HashMap<GameId, Arc<Mutex<()>>>,
}

impl GamesManager {
    fn new() -> Self {
        Self {
            games_locks: HashMap::new(),
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
async fn admin_get_games(storage: StorageData, req: HttpRequest) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_admin() {
        return AuthHandler::get_unauthorized();
    }

    let games_ids = storage.get_games_ids(&None).await;

    match games_ids {
        Ok(games_ids) => {
            let response: AdminGetGamesResponse = games_ids;
            HttpResponse::Ok().json(response)
        }
        Err(_) => HttpResponse::InternalServerError().body("Error getting games"),
    }
}

#[get("/v1/user/game")]
async fn user_get_games(storage: StorageData, req: HttpRequest) -> impl Responder {
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

    let games_ids = storage.get_games_ids(&Some(player_id)).await;

    match games_ids {
        Ok(games_ids) => {
            let response: UserGetGamesResponse = games_ids;
            HttpResponse::Ok().json(response)
        }
        Err(_) => HttpResponse::InternalServerError().body("Error getting games"),
    }
}

#[post("/v1/admin/game")]
async fn admin_post_game(
    storage: StorageData,
    srv: SocketServer,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_admin() {
        return AuthHandler::get_unauthorized();
    }

    let game_wrapper = GameWrapper::from_new_game(&storage, srv, None).await;

    if game_wrapper.is_err() {
        return HttpResponse::InternalServerError().body("Error creating game");
    }

    let game_wrapper = game_wrapper.unwrap();

    game_wrapper.handle_admin_new_game().await
}

#[get("/v1/admin/game/{game_id}")]
async fn admin_get_game_by_id(
    storage: StorageData,
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

    let game = storage.get_game(&game_id.to_string()).await;

    match game {
        Ok(game) => HttpResponse::Ok().json(game),
        Err(_) => HttpResponse::InternalServerError().body("Error loading game"),
    }
}

#[get("/v1/user/game/{game_id}")]
async fn user_get_game_load(
    storage: StorageData,
    game_id: web::Path<String>,
    manager: GamesManagerData,
    req: HttpRequest,
    srv: SocketServer,
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

    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
    let _game_lock = game_lock.lock().unwrap();

    let game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await;
    match game_wrapper {
        Ok(game_wrapper) => game_wrapper.user_load_game(&player_id),
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/sort-hands")]
async fn admin_post_game_sort_hands(
    manager: GamesManagerData,
    storage: StorageData,
    game_id: web::Path<String>,
    srv: SocketServer,
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
    storage: StorageData,
    game_id: web::Path<GameId>,
    srv: SocketServer,
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
    storage: StorageData,
    game_id: web::Path<String>,
    srv: SocketServer,
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
    storage: StorageData,
    body: web::Json<AdminPostBreakMeldRequest>,
    game_id: web::Path<String>,
    srv: SocketServer,
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
    storage: StorageData,
    body: web::Json<AdminPostCreateMeldRequest>,
    game_id: web::Path<String>,
    srv: SocketServer,
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
    storage: StorageData,
    body: web::Json<UserPostDiscardTileRequest>,
    game_id: web::Path<String>,
    req: HttpRequest,
    manager: GamesManagerData,
    srv: SocketServer,
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
    storage: StorageData,
    body: web::Json<AdminPostDiscardTileRequest>,
    game_id: web::Path<String>,
    srv: SocketServer,
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
    storage: StorageData,
    body: web::Json<AdminPostClaimTileRequest>,
    game_id: web::Path<String>,
    srv: SocketServer,
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

#[post("/v1/admin/game/{game_id}/draw-wall-swap-tiles")]
async fn admin_post_game_swap_tiles(
    manager: GamesManagerData,
    storage: StorageData,
    body: web::Json<AdminPostSwapDrawTilesRequest>,
    game_id: web::Path<String>,
    srv: SocketServer,
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
        Ok(mut game_wrapper) => {
            game_wrapper
                .handle_draw_wall_swap_tiles(&body.tile_id_a, &body.tile_id_b)
                .await
        }
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/ai-continue")]
async fn admin_post_game_ai_continue(
    manager: GamesManagerData,
    storage: StorageData,
    game_id: web::Path<String>,
    body: web::Json<AdminPostAIContinueRequest>,
    srv: SocketServer,
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
    storage: StorageData,
    game_id: web::Path<String>,
    body: web::Json<UserPostAIContinueRequest>,
    manager: GamesManagerData,
    req: HttpRequest,
    srv: SocketServer,
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
    storage: StorageData,
    body: web::Json<AdminPostSayMahjongRequest>,
    game_id: web::Path<String>,
    srv: SocketServer,
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
    storage: StorageData,
    srv: SocketServer,
    body: web::Json<UserPostCreateGameRequest>,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_user(&body.player_id) {
        return AuthHandler::get_unauthorized();
    }

    let game_wrapper =
        GameWrapper::from_new_game(&storage, srv, Some(body.player_id.clone())).await;

    if game_wrapper.is_err() {
        return HttpResponse::InternalServerError().body("Error preparing game");
    }

    let game_wrapper = game_wrapper.unwrap();

    game_wrapper.handle_user_new_game(&body.player_id).await
}

#[post("/v1/user/game/{game_id}/draw-tile")]
async fn user_post_game_draw_tile(
    storage: StorageData,
    game_id: web::Path<GameId>,
    body: web::Json<UserPostDrawTileRequest>,
    srv: SocketServer,
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
    storage: StorageData,
    game_id: web::Path<GameId>,
    body: web::Json<UserPostMovePlayerRequest>,
    srv: SocketServer,
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
    storage: StorageData,
    game_id: web::Path<GameId>,
    body: web::Json<UserPostSortHandRequest>,
    srv: SocketServer,
    manager: GamesManagerData,
    req: HttpRequest,
) -> impl Responder {
    let auth_handler = AuthHandler::new(&storage, &req);

    if !auth_handler.verify_user(&body.player_id) {
        return AuthHandler::get_unauthorized();
    }

    let game_lock = {
        let mut manager_lock = manager.lock().unwrap();
        manager_lock.get_game_mutex(&game_id)
    };
    let _game_lock = game_lock.lock().unwrap();

    let game_wrapper =
        GameWrapper::from_storage(&storage, &game_id, srv, Some(&body.game_version)).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_user_sort_hand(&body.player_id).await,
        Err(err) => err,
    }
}

#[post("/v1/user/game/{game_id}/create-meld")]
async fn user_post_game_create_meld(
    storage: StorageData,
    game_id: web::Path<GameId>,
    body: web::Json<UserPostCreateMeldRequest>,
    srv: SocketServer,
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
    storage: StorageData,
    game_id: web::Path<GameId>,
    body: web::Json<UserPostBreakMeldRequest>,
    srv: SocketServer,
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
    storage: StorageData,
    body: web::Json<UserPostClaimTileRequest>,
    game_id: web::Path<String>,
    srv: SocketServer,
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
    storage: StorageData,
    body: web::Json<UserPostSayMahjongRequest>,
    game_id: web::Path<String>,
    manager: GamesManagerData,
    srv: SocketServer,
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

#[post("/v1/user/game/{game_id}/settings")]
async fn user_post_game_settings(
    storage: StorageData,
    body: web::Json<UserPostSetGameSettingsRequest>,
    game_id: web::Path<GameId>,
    manager: GamesManagerData,
    srv: SocketServer,
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
    storage: StorageData,
    body: web::Json<UserPostSetAuthRequest>,
    req: HttpRequest,
) -> impl Responder {
    let mut auth_handler = AuthHandler::new(&storage, &req);

    let user = auth_handler
        .validate_user(&body.username, &body.password)
        .await;

    if user.is_err() {
        return HttpResponse::Unauthorized().finish();
    }

    let user = user.unwrap();

    if user.is_none() {
        let result = auth_handler
            .create_user(
                &body.username,
                &body.password,
                if body.username == "admin" {
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

    let is_valid = user.unwrap();

    if is_valid {
        let data = auth_handler.generate_token();

        if data.is_err() {
            return HttpResponse::InternalServerError().json("Error generating json");
        }

        HttpResponse::Ok().json(data.unwrap())
    } else {
        HttpResponse::Unauthorized().json("Invalid username or password")
    }
}

#[get("/v1/ws")]
async fn get_ws(
    req: HttpRequest,
    stream: web::Payload,
    srv: SocketServer,
    storage: StorageData,
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
            name: None,
            room: MahjongWebsocketSession::get_room_id(&game_id, player_id.as_ref()),
        },
        &req,
        stream,
    )
}

pub struct MahjongServer;

impl MahjongServer {
    pub async fn start(storage: Box<dyn Storage>) -> std::io::Result<()> {
        let port = 3000;
        let address = "0.0.0.0";

        println!("Starting the Mahjong HTTP server on port http://{address}:{port}");

        let games_manager = GamesManager::new();
        let games_manager_arc = Arc::new(Mutex::new(games_manager));
        let loop_games_manager_arc = games_manager_arc.clone();
        let storage_arc = Arc::new(storage);
        let loop_storage_arc = storage_arc.clone();
        let socket_server = Arc::new(Mutex::new(MahjongWebsocketServer::new().start()));
        let loop_socket_server = socket_server.clone();

        GamesLoop::new(loop_storage_arc, loop_socket_server, loop_games_manager_arc).run();

        HttpServer::new(move || {
            let storage_data: StorageData = web::Data::new(storage_arc.clone());
            let games_manager_data = web::Data::new(games_manager_arc.clone());
            let cors = Cors::permissive();
            let endpoints_server = socket_server.clone();

            App::new()
                .wrap(cors)
                .app_data(storage_data)
                .app_data(games_manager_data)
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
                .service(admin_post_game_swap_tiles)
                .service(get_health)
                .service(get_ws)
                .service(user_get_game_load)
                .service(user_get_games)
                .service(user_post_auth)
                .service(user_post_game_ai_continue)
                .service(user_post_game_break_meld)
                .service(user_post_game_claim_tile)
                .service(user_post_game_create)
                .service(user_post_game_create_meld)
                .service(user_post_game_discard_tile)
                .service(user_post_game_draw_tile)
                .service(user_post_game_move_player)
                .service(user_post_game_say_mahjong)
                .service(user_post_game_settings)
                .service(user_post_game_sort_hand)
        })
        .bind((address, port))?
        .run()
        .await
    }
}
