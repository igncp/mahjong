use self::user::get_user_scope;
use crate::auth::{
    AuthHandler, AuthInfoData, GetAuthInfo, GithubAuth, GithubCallbackQuery, UnauthorizedError,
};
use crate::common::Storage;
use crate::env::ENV_FRONTEND_URL;
use crate::games_loop::GamesLoop;
use crate::http_server::admin::get_admin_scope;
pub use crate::http_server::base::{DataSocketServer, DataStorage, GamesManager};
use crate::service_error::{ResponseCommon, ServiceError};
use crate::socket::{MahjongWebsocketServer, MahjongWebsocketSession};
use actix::prelude::*;
use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{
    dev::{fn_service, ServiceRequest, ServiceResponse},
    get,
    http::StatusCode,
    post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use mahjong_core::deck::DEFAULT_DECK;
use serde::{Deserialize, Serialize};
use service_contracts::{GetDeckResponse, WebSocketQuery};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tracing::warn;

mod admin;
mod base;
mod user;

#[get("/api/health")]
async fn get_health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/api/v1/deck")]
async fn get_deck() -> ResponseCommon {
    let response = GetDeckResponse(DEFAULT_DECK.clone().0);

    Ok(HttpResponse::Ok().json(response))
}

#[get("/api/v1/github_callback")]
async fn github_callback(req: HttpRequest, storage: DataStorage) -> Result<impl Responder, Error> {
    let query = web::Query::<GithubCallbackQuery>::from_query(req.query_string());

    if query.is_err() {
        return Ok(HttpResponse::BadRequest().json("Invalid query"));
    }

    let query = query.unwrap();

    let mut auth_handler = AuthHandler::new(&storage, &req);
    let result = GithubAuth::handle_callback(query, &storage, &mut auth_handler)
        .await
        .ok_or(ServiceError::Custom("Error handling callback"))?;

    let response_qs =
        serde_qs::to_string(&result).map_err(|_| ServiceError::Custom("Error parsing response"))?;

    let frontend_url = std::env::var(ENV_FRONTEND_URL).unwrap();
    let redirect = HttpResponse::Found()
        .append_header(("Location", format!("{}?{}", frontend_url, response_qs)))
        .finish();

    Ok(redirect)
}

#[get("/api/v1/ws")]
async fn get_ws(
    req: HttpRequest,
    stream: web::Payload,
    srv: DataSocketServer,
    storage: DataStorage,
) -> Result<impl Responder, Error> {
    let params = web::Query::<WebSocketQuery>::from_query(req.query_string())
        .map_err(|_| ServiceError::Custom("Invalid query parameters"))?;

    let auth_handler = AuthHandler::new(&storage, &req);

    if (params.player_id.is_some()
        && !auth_handler.verify_user_token(&params.player_id.clone().unwrap(), &params.token))
        || (params.player_id.is_none() && !auth_handler.verify_admin_token(&params.token))
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
            room: MahjongWebsocketSession::get_room_id(&params.game_id, params.player_id.as_ref()),
        },
        &req,
        stream,
    )
}

#[post("/api/v1/test/delete-games")]
async fn test_post_delete_games(req: HttpRequest, storage: DataStorage) -> ResponseCommon {
    let user_id = AuthHandler::new(&storage, &req).get_user_from_token()?;

    // If deleting for normal users, should check if any active running at the moment by using
    // the web socket

    let auth_info = storage
        .get_auth_info(GetAuthInfo::PlayerId(user_id.clone()))
        .await
        .map_err(|_| UnauthorizedError)?
        .ok_or(UnauthorizedError)?;

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

    storage
        .delete_games(&games_ids)
        .await
        .map_err(|_| ServiceError::Custom("Error deleting games"))?;

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

pub async fn start_server(storage: Box<dyn Storage>) -> std::io::Result<()> {
    let port = 3000;
    let address = "0.0.0.0";

    warn!("Starting the Mahjong HTTP server on http://{address}:{port}");

    let games_manager = GamesManager::default();
    let games_manager_arc = Arc::new(Mutex::new(games_manager));
    let loop_games_manager_arc = games_manager_arc.clone();
    let storage_arc = Arc::new(storage);
    let loop_storage_arc = storage_arc.clone();
    let socket_server = Arc::new(Mutex::new(MahjongWebsocketServer::default().start()));
    let loop_socket_server = socket_server.clone();

    GamesLoop::new(loop_storage_arc, loop_socket_server, loop_games_manager_arc).run();

    HttpServer::new(move || {
        let storage_data: DataStorage = web::Data::new(storage_arc.clone());
        let games_manager_data = web::Data::new(games_manager_arc.clone());
        let cors = Cors::permissive();
        let endpoints_server = socket_server.clone();

        let user_scope = get_user_scope();
        let admin_scope = get_admin_scope();

        let static_files = Files::new("/", "./static")
            .index_file("index.html")
            .redirect_to_slash_directory()
            .default_handler(fn_service(|req: ServiceRequest| async {
                let (req, _) = req.into_parts();

                let file = NamedFile::open_async("./static/404/index.html").await?;

                let res = file
                    .customize()
                    .with_status(StatusCode::NOT_FOUND)
                    .respond_to(&req)
                    .map_into_boxed_body();

                Ok(ServiceResponse::new(req, res))
            }));

        App::new()
            .app_data(games_manager_data)
            .app_data(storage_data)
            .app_data(web::Data::new(endpoints_server))
            .service(admin_scope)
            .service(get_deck)
            .service(get_health)
            .service(get_ws)
            .service(github_callback)
            .service(test_post_delete_games)
            .service(user_scope)
            .service(static_files)
            .wrap(cors)
    })
    .bind((address, port))?
    .run()
    .await
}
