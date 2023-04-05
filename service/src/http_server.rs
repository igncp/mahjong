use crate::common::Storage;
use crate::game_wrapper::GameWrapper;
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
    UserGetGamesResponse, UserLoadGameQuery, UserPostBreakMeldRequest, UserPostCreateMeldRequest,
    UserPostDiscardTileRequest, UserPostDrawTileRequest, UserPostMovePlayerRequest,
    UserPostSortHandRequest, WebSocketQuery,
};
use std::sync::Arc;
use std::time::Instant;

pub type StorageData = web::Data<Arc<Box<dyn Storage>>>;
pub type SocketServer = web::Data<Addr<MahjongWebsocketServer>>;

#[get("/health")]
async fn get_health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/v1/admin/game")]
async fn admin_get_games(storage: StorageData) -> impl Responder {
    let games_ids = storage.get_games_ids(&None).await;

    match games_ids {
        Ok(games_ids) => {
            let response: AdminGetGamesResponse = games_ids;
            HttpResponse::Ok().json(response)
        }
        Err(_) => HttpResponse::InternalServerError().body("Error creating game"),
    }
}

#[get("/v1/user/game")]
async fn user_get_games(storage: StorageData, req: HttpRequest) -> impl Responder {
    let params = web::Query::<UserGetGamesQuery>::from_query(req.query_string());
    if params.is_err() {
        return HttpResponse::BadRequest().body("Invalid player id");
    }
    let player_id = params.unwrap().player_id.clone();
    let games_ids = storage.get_games_ids(&Some(player_id)).await;

    match games_ids {
        Ok(games_ids) => {
            let response: UserGetGamesResponse = games_ids;
            HttpResponse::Ok().json(response)
        }
        Err(_) => HttpResponse::InternalServerError().body("Error creating game"),
    }
}

#[post("/v1/admin/game")]
async fn admin_post_game(storage: StorageData, srv: SocketServer) -> impl Responder {
    let game_wrapper = GameWrapper::from_new_game(storage, srv).await;

    game_wrapper.handle_new_game().await
}

#[get("/v1/admin/game/{game_id}")]
async fn admin_get_game_by_id(storage: StorageData, game_id: web::Path<String>) -> impl Responder {
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
    req: HttpRequest,
    srv: SocketServer,
) -> impl Responder {
    let params = web::Query::<UserLoadGameQuery>::from_query(req.query_string());

    let player_id = match params {
        Ok(params) => params.player_id.clone(),
        Err(_) => return HttpResponse::BadRequest().body("Invalid player id"),
    };

    let game_wrapper = GameWrapper::from_storage(storage, &game_id, srv).await;

    match game_wrapper {
        Ok(game_wrapper) => game_wrapper.user_load_game(&player_id),
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/sort-hands")]
async fn admin_post_game_sort_hands(
    storage: StorageData,
    game_id: web::Path<String>,
    srv: SocketServer,
) -> impl Responder {
    let game_wrapper = GameWrapper::from_storage(storage, &game_id, srv).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_sort_hands().await,
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/draw-tile")]
async fn admin_post_game_draw_tile(
    storage: StorageData,
    game_id: web::Path<GameId>,
    srv: SocketServer,
) -> impl Responder {
    let game_wrapper = GameWrapper::from_storage(storage, &game_id, srv).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_admin_draw_tile().await,
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/move-player")]
async fn admin_post_game_move_player(
    storage: StorageData,
    game_id: web::Path<String>,
    srv: SocketServer,
) -> impl Responder {
    let game_wrapper = GameWrapper::from_storage(storage, &game_id, srv).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_admin_move_player().await,
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/break-meld")]
async fn admin_post_game_break_meld(
    storage: StorageData,
    body: web::Json<AdminPostBreakMeldRequest>,
    game_id: web::Path<String>,
    srv: SocketServer,
) -> impl Responder {
    let game_wrapper = GameWrapper::from_storage(storage, &game_id, srv).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_admin_break_meld(&body).await,
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/create-meld")]
async fn admin_post_game_create_meld(
    storage: StorageData,
    body: web::Json<AdminPostCreateMeldRequest>,
    game_id: web::Path<String>,
    srv: SocketServer,
) -> impl Responder {
    let game_wrapper = GameWrapper::from_storage(storage, &game_id, srv).await;

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
    srv: SocketServer,
) -> impl Responder {
    let game_wrapper = GameWrapper::from_storage(storage, &game_id, srv).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_discard_tile(false, &body.tile_id).await,
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/discard-tile")]
async fn admin_post_game_discard_tile(
    storage: StorageData,
    body: web::Json<AdminPostDiscardTileRequest>,
    game_id: web::Path<String>,
    srv: SocketServer,
) -> impl Responder {
    let game_wrapper = GameWrapper::from_storage(storage, &game_id, srv).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_discard_tile(true, &body.tile_id).await,
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/claim-tile")]
async fn admin_post_game_claim_tile(
    storage: StorageData,
    body: web::Json<AdminPostClaimTileRequest>,
    game_id: web::Path<String>,
    srv: SocketServer,
) -> impl Responder {
    let game_wrapper = GameWrapper::from_storage(storage, &game_id, srv).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_claim_tile(&body.player_id).await,
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/draw-wall-swap-tiles")]
async fn admin_post_game_swap_tiles(
    storage: StorageData,
    body: web::Json<AdminPostSwapDrawTilesRequest>,
    game_id: web::Path<String>,
    srv: SocketServer,
) -> impl Responder {
    let game_wrapper = GameWrapper::from_storage(storage, &game_id, srv).await;

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
    storage: StorageData,
    game_id: web::Path<String>,
    body: web::Json<AdminPostAIContinueRequest>,
    srv: SocketServer,
) -> impl Responder {
    let game_wrapper = GameWrapper::from_storage(storage, &game_id, srv).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_admin_ai_continue(&body).await,
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/say-mahjong")]
async fn admin_post_game_say_mahjong(
    storage: StorageData,
    body: web::Json<AdminPostSayMahjongRequest>,
    game_id: web::Path<String>,
    srv: SocketServer,
) -> impl Responder {
    let game_wrapper = GameWrapper::from_storage(storage, &game_id, srv).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_say_mahjong(&body.player_id).await,
        Err(err) => err,
    }
}

#[post("/v1/user/game/{game_id}/draw-tile")]
async fn user_post_game_draw_tile(
    storage: StorageData,
    game_id: web::Path<GameId>,
    body: web::Json<UserPostDrawTileRequest>,
    srv: SocketServer,
) -> impl Responder {
    let game_wrapper = GameWrapper::from_storage(storage, &game_id, srv).await;

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
) -> impl Responder {
    let game_wrapper = GameWrapper::from_storage(storage, &game_id, srv).await;

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
) -> impl Responder {
    let game_wrapper = GameWrapper::from_storage(storage, &game_id, srv).await;

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
) -> impl Responder {
    let game_wrapper = GameWrapper::from_storage(storage, &game_id, srv).await;

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
) -> impl Responder {
    let game_wrapper = GameWrapper::from_storage(storage, &game_id, srv).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_user_break_meld(&body).await,
        Err(err) => err,
    }
}

#[get("/v1/ws")]
async fn get_ws(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<MahjongWebsocketServer>>,
) -> Result<impl Responder, Error> {
    let params = web::Query::<WebSocketQuery>::from_query(req.query_string());

    if params.is_err() {
        return Ok(HttpResponse::BadRequest().body("Invalid query parameters"));
    }

    let params = params.unwrap();
    let game_id = params.game_id.clone();
    let player_id = params.player_id.clone();

    ws::start(
        MahjongWebsocketSession {
            addr: srv.get_ref().clone(),
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
        let storage_arc = Arc::new(storage);
        let socket_server = MahjongWebsocketServer::new().start();

        HttpServer::new(move || {
            let storage_data: StorageData = web::Data::new(storage_arc.clone());
            let cors = Cors::permissive();

            App::new()
                .wrap(cors)
                .app_data(storage_data)
                .app_data(web::Data::new(socket_server.clone()))
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
                .service(user_post_game_break_meld)
                .service(user_post_game_create_meld)
                .service(user_post_game_discard_tile)
                .service(user_post_game_draw_tile)
                .service(user_post_game_move_player)
                .service(user_post_game_sort_hand)
        })
        .bind((address, port))?
        .run()
        .await
    }
}
