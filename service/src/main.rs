#![deny(clippy::use_self, clippy::shadow_unrelated)]
use file_storage::FileStorage;
use http_server::MahjongServer;

mod common;
mod file_storage;
mod game_wrapper;
mod http_server;
mod socket_server;
mod socket_session;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let storage = FileStorage::new_dyn();

    MahjongServer::start(storage).await
}
