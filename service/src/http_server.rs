use crate::common::Storage;
use crate::game_wrapper;
use crate::socket_server::MahjongWebsocketServer;
use crate::socket_session::MahjongWebsocketSession;
use actix::prelude::*;
use actix_web::{get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use serde::Deserialize;
use service_contracts::{
    AdminGetGamesResponse, AdminPostCreateMeldRequest, AdminPostDiscardTileRequest,
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
    let games_ids = storage.get_games_ids().await;

    match games_ids {
        Ok(games_ids) => {
            let response: AdminGetGamesResponse = games_ids;
            HttpResponse::Ok().json(response)
        }
        Err(_) => HttpResponse::InternalServerError().body("Error creating game"),
    }
}

#[post("/v1/admin/game")]
async fn admin_post_game(storage: StorageData, srv: SocketServer) -> impl Responder {
    let game_wrapper = game_wrapper::GameWrapper::from_new_game(storage, srv).await;

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

#[post("/v1/admin/game/{game_id}/sort-hands")]
async fn admin_post_game_sort_hands(
    storage: StorageData,
    game_id: web::Path<String>,
    srv: SocketServer,
) -> impl Responder {
    let game_wrapper = game_wrapper::GameWrapper::from_storage(storage, &game_id, srv).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_sort_hands().await,
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/draw-tile")]
async fn admin_post_game_draw_tile(
    storage: StorageData,
    game_id: web::Path<String>,
    srv: SocketServer,
) -> impl Responder {
    let game_wrapper = game_wrapper::GameWrapper::from_storage(storage, &game_id, srv).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_draw_tile().await,
        Err(err) => err,
    }
}

#[post("/v1/admin/game/{game_id}/move-player")]
async fn admin_post_game_move_player(
    storage: StorageData,
    game_id: web::Path<String>,
    srv: SocketServer,
) -> impl Responder {
    let game_wrapper = game_wrapper::GameWrapper::from_storage(storage, &game_id, srv).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_move_player().await,
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
    let game_wrapper = game_wrapper::GameWrapper::from_storage(storage, &game_id, srv).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_create_meld(&body).await,
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
    let game_wrapper = game_wrapper::GameWrapper::from_storage(storage, &game_id, srv).await;

    match game_wrapper {
        Ok(mut game_wrapper) => game_wrapper.handle_discard_tile(&body.tile_id).await,
        Err(err) => err,
    }
}

#[derive(Deserialize)]
struct WebSocketQuery {
    game_id: String,
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

    let game_id = params.unwrap().game_id.clone();

    ws::start(
        MahjongWebsocketSession {
            id: rand::random(),
            hb: Instant::now(),
            room: game_id,
            name: None,
            addr: srv.get_ref().clone(),
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
        let server = MahjongWebsocketServer::new().start();

        HttpServer::new(move || {
            let storage_data: StorageData = web::Data::new(storage_arc.clone());
            App::new()
                .app_data(web::Data::new(server.clone()))
                .app_data(storage_data)
                .service(get_health)
                .service(admin_get_games)
                .service(admin_post_game)
                .service(admin_get_game_by_id)
                .service(admin_post_game_sort_hands)
                .service(admin_post_game_draw_tile)
                .service(admin_post_game_create_meld)
                .service(admin_post_game_discard_tile)
                .service(admin_post_game_move_player)
                .service(get_ws)
        })
        .bind((address, port))?
        .run()
        .await
    }
}
