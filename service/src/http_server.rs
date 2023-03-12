use crate::{common::Storage, game_wrapper::create_game};
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::sync::Arc;

type StorageData = web::Data<Arc<Box<dyn Storage>>>;

#[get("/health")]
async fn get_health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[derive(Debug, Deserialize)]
pub struct AdminGameGetQuery {
    id: String,
}

#[get("/v1/admin/game")]
async fn admin_game_get(storage: StorageData, req: HttpRequest) -> impl Responder {
    let params = web::Query::<AdminGameGetQuery>::from_query(req.query_string());
    if params.is_err() {
        return HttpResponse::BadRequest().body("Invalid query params");
    }
    let game_id = params.unwrap().id.clone();
    let game = storage.get_game(&game_id).await;

    match game {
        Ok(game) => HttpResponse::Ok().json(game),
        Err(_) => HttpResponse::InternalServerError().body("Error loading game"),
    }
}

#[post("/v1/admin/game")]
async fn admin_game_post(storage: StorageData) -> impl Responder {
    let game = create_game();
    let save_result = storage.save_game(&game).await;

    match save_result {
        Ok(_) => HttpResponse::Ok().json(game),
        Err(_) => HttpResponse::InternalServerError().body("Error creating game"),
    }
}

pub struct MahjongServer {}

impl MahjongServer {
    pub async fn start(storage: Box<dyn Storage>) -> std::io::Result<()> {
        let port = 3000;
        let address = "0.0.0.0";

        println!("Starting the Mahjong HTTP server on port http://{address}:{port}");
        let storage_arc = Arc::new(storage);

        HttpServer::new(move || {
            let storage_data: StorageData = web::Data::new(storage_arc.clone());
            App::new()
                .app_data(storage_data)
                .service(get_health)
                .service(admin_game_post)
                .service(admin_game_get)
        })
        .bind((address, port))?
        .run()
        .await
    }
}
