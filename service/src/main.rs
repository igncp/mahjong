use file_storage::FileStorage;
use http_server::MahjongServer;

mod common;
mod file_storage;
mod game_wrapper;
mod http_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let storage = FileStorage::new_dyn();

    MahjongServer::start(storage).await
}
