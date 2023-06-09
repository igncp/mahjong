#![deny(clippy::use_self, clippy::shadow_unrelated)]
#![allow(clippy::await_holding_lock)]
use auth::AuthHandler;
use dotenv::dotenv;
use http_server::MahjongServer;
use sqlite_storage::SQLiteStorage;
use std::process;
use tracing::{error, info};

use crate::logs::setup_logs;

mod ai_wrapper;
mod auth;
mod common;
mod env;
mod file_storage;
mod game_wrapper;
mod games_loop;
mod graphql;
mod http_server;
mod logs;
mod socket_server;
mod socket_session;
mod sqlite_storage;
mod user_wrapper;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    setup_logs();

    let is_setup_ok = AuthHandler::verify_setup();

    if !is_setup_ok {
        error!("Auth setup is not ok, check the env variables");
        process::exit(1);
    }

    let storage = SQLiteStorage::new_dyn();

    info!("Starting the application");

    MahjongServer::start(storage).await
}
