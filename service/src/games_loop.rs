use crate::common::Storage;
use crate::game_wrapper::GameWrapper;
use crate::http_server::GamesManager;
use crate::socket_server::{ListSessions, MahjongWebsocketServer};
use actix::{spawn, Addr};
use actix_web::rt::time;
use actix_web::web;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct GamesLoop {
    storage: Arc<Box<dyn Storage>>,
    socket: Arc<Mutex<Addr<MahjongWebsocketServer>>>,
    manager: Arc<Mutex<GamesManager>>,
}

impl GamesLoop {
    pub fn new(
        storage: Arc<Box<dyn Storage>>,
        socket: Arc<Mutex<Addr<MahjongWebsocketServer>>>,
        manager: Arc<Mutex<GamesManager>>,
    ) -> Self {
        Self {
            manager,
            socket,
            storage,
        }
    }

    pub fn run(&self) {
        let storage = self.storage.clone();
        let socket = self.socket.clone();
        let manager = self.manager.clone();

        spawn(async move {
            let mut interval = time::interval(Duration::from_millis(1000));

            loop {
                interval.tick().await;
                let message_response;
                {
                    let server = socket.lock();
                    if server.is_err() {
                        continue;
                    }
                    message_response = server.unwrap().send(ListSessions);
                }

                let sessions = message_response.await.unwrap();

                for room in sessions.keys() {
                    let game_id = room.split('_').collect::<Vec<&str>>()[0];
                    let game_id = web::Path::from(game_id.to_string());
                    let storage = web::Data::new(storage.clone());
                    let server = web::Data::new(socket.clone());

                    let game_lock = { manager.lock().unwrap().get_game_mutex(&game_id) };
                    let _game_lock = game_lock.lock().unwrap();

                    let game_wrapper =
                        GameWrapper::from_storage(&storage, &game_id, server, None).await;

                    if game_wrapper.is_err() {
                        continue;
                    }

                    game_wrapper.unwrap().handle_server_ai_continue().await;
                }
            }
        });
    }
}
