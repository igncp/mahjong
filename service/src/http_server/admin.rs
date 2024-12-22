#![allow(clippy::await_holding_lock)]
use crate::auth::AuthHandler;
use crate::game_wrapper::{CreateGameOpts, GameWrapper};
use crate::http_server::base::{get_lock, GamesManagerData};
pub use crate::http_server::base::{DataSocketServer, DataStorage};
use crate::service_error::{ResponseCommon, ServiceError};
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use mahjong_core::GameId;
use service_contracts::{
    AdminGetGamesResponse, AdminPostAIContinueRequest, AdminPostBreakMeldRequest,
    AdminPostClaimTileRequest, AdminPostCreateMeldRequest, AdminPostDiscardTileRequest,
    AdminPostSayMahjongRequest,
};

#[get("/game")]
async fn admin_get_games(storage: DataStorage, req: HttpRequest) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_admin()?;

    let response: AdminGetGamesResponse = storage
        .get_player_games(&None)
        .await
        .map_err(|_| ServiceError::Custom("Error getting games"))?;

    Ok(HttpResponse::Ok().json(response))
}

#[post("/game")]
async fn admin_post_game(
    storage: DataStorage,
    srv: DataSocketServer,
    req: HttpRequest,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_admin()?;

    let new_game_opts = CreateGameOpts::default();
    let game_wrapper = GameWrapper::from_new_game(&storage, srv, &new_game_opts)
        .await
        .map_err(|_| ServiceError::Custom("Error creating game"))?;

    game_wrapper.handle_admin_new_game().await
}

#[get("/game/{game_id}")]
async fn admin_get_game_by_id(
    storage: DataStorage,
    manager: GamesManagerData,
    game_id: web::Path<String>,
    req: HttpRequest,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_admin()?;

    get_lock!(manager, game_id);

    let game = storage
        .get_game(&game_id.to_string(), true)
        .await
        .map_err(|_| ServiceError::Custom("Error loading game"))?;

    Ok(HttpResponse::Ok().json(game))
}

#[post("/game/{game_id}/sort-hands")]
async fn admin_post_game_sort_hands(
    manager: GamesManagerData,
    storage: DataStorage,
    game_id: web::Path<String>,
    srv: DataSocketServer,
    req: HttpRequest,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_admin()?;

    get_lock!(manager, game_id);

    let mut game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await?;

    game_wrapper.handle_sort_hands().await
}

#[post("/game/{game_id}/draw-tile")]
async fn admin_post_game_draw_tile(
    manager: GamesManagerData,
    storage: DataStorage,
    game_id: web::Path<GameId>,
    srv: DataSocketServer,
    req: HttpRequest,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_admin()?;

    get_lock!(manager, game_id);

    let mut game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await?;

    game_wrapper.handle_admin_draw_tile().await
}

#[post("/game/{game_id}/move-player")]
async fn admin_post_game_move_player(
    manager: GamesManagerData,
    storage: DataStorage,
    game_id: web::Path<String>,
    srv: DataSocketServer,
    req: HttpRequest,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_admin()?;

    get_lock!(manager, game_id);

    let mut game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await?;

    game_wrapper.handle_admin_move_player().await
}

#[post("/game/{game_id}/break-meld")]
async fn admin_post_game_break_meld(
    manager: GamesManagerData,
    storage: DataStorage,
    body: web::Json<AdminPostBreakMeldRequest>,
    game_id: web::Path<String>,
    srv: DataSocketServer,
    req: HttpRequest,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_admin()?;

    get_lock!(manager, game_id);

    let mut game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await?;

    game_wrapper.handle_admin_break_meld(&body).await
}

#[post("/game/{game_id}/create-meld")]
async fn admin_post_game_create_meld(
    manager: GamesManagerData,
    storage: DataStorage,
    body: web::Json<AdminPostCreateMeldRequest>,
    game_id: web::Path<String>,
    srv: DataSocketServer,
    req: HttpRequest,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_admin()?;

    get_lock!(manager, game_id);

    let mut game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await?;

    game_wrapper.handle_admin_create_meld(&body).await
}

#[post("/game/{game_id}/discard-tile")]
async fn admin_post_game_discard_tile(
    manager: GamesManagerData,
    storage: DataStorage,
    body: web::Json<AdminPostDiscardTileRequest>,
    game_id: web::Path<String>,
    srv: DataSocketServer,
    req: HttpRequest,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_admin()?;

    get_lock!(manager, game_id);

    let mut game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await?;

    game_wrapper.handle_discard_tile_admin(&body.tile_id).await
}

#[post("/game/{game_id}/claim-tile")]
async fn admin_post_game_claim_tile(
    manager: GamesManagerData,
    storage: DataStorage,
    body: web::Json<AdminPostClaimTileRequest>,
    game_id: web::Path<String>,
    srv: DataSocketServer,
    req: HttpRequest,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_admin()?;

    get_lock!(manager, game_id);

    let mut game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await?;

    game_wrapper.handle_admin_claim_tile(&body.player_id).await
}

#[post("/game/{game_id}/ai-continue")]
async fn admin_post_game_ai_continue(
    manager: GamesManagerData,
    storage: DataStorage,
    game_id: web::Path<String>,
    body: web::Json<AdminPostAIContinueRequest>,
    srv: DataSocketServer,
    req: HttpRequest,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_admin()?;

    get_lock!(manager, game_id);

    let mut game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await?;

    game_wrapper.handle_admin_ai_continue(&body).await
}

#[post("/game/{game_id}/say-mahjong")]
async fn admin_post_game_say_mahjong(
    manager: GamesManagerData,
    storage: DataStorage,
    body: web::Json<AdminPostSayMahjongRequest>,
    game_id: web::Path<String>,
    srv: DataSocketServer,
    req: HttpRequest,
) -> ResponseCommon {
    AuthHandler::new(&storage, &req).verify_admin()?;

    get_lock!(manager, game_id);

    let mut game_wrapper = GameWrapper::from_storage(&storage, &game_id, srv, None).await?;

    game_wrapper.handle_admin_say_mahjong(&body.player_id).await
}

pub fn get_admin_scope() -> actix_web::Scope {
    web::scope("/api/v1/admin")
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
}
