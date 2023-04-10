#![deny(clippy::use_self, clippy::shadow_unrelated)]
#![allow(clippy::await_holding_lock)]
use auth::AuthHandler;
use http_server::MahjongServer;
use sqlite_storage::SQLiteStorage;
use std::process;

mod auth;
mod common;
mod env;
mod file_storage;
mod game_wrapper;
mod games_loop;
mod http_server;
mod socket_server;
mod socket_session;
mod sqlite_storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let is_setup_ok = AuthHandler::verify_setup();

    if !is_setup_ok {
        println!("Auth setup is not ok, check the env variables");
        process::exit(1);
    }

    let storage = SQLiteStorage::new_dyn();

    MahjongServer::start(storage).await
}
