#![allow(clippy::await_holding_lock)]
use crate::common::Storage;
use crate::socket::MahjongWebsocketServer;
use actix::prelude::*;
use actix_web::web;
use mahjong_core::GameId;
use rustc_hash::FxHashMap;
use std::sync::{Arc, Mutex};

pub type DataStorage = web::Data<Arc<Box<dyn Storage>>>;
pub type DataSocketServer = web::Data<Arc<Mutex<Addr<MahjongWebsocketServer>>>>;

#[derive(Default)]
pub struct GamesManager {
    games_locks: FxHashMap<GameId, Arc<Mutex<()>>>,
}

impl GamesManager {
    pub fn get_game_mutex(&mut self, game_id: &GameId) -> Arc<Mutex<()>> {
        let mutex_arc = self
            .games_locks
            .entry(game_id.clone())
            .or_insert(Arc::new(Mutex::new(())));

        mutex_arc.clone()
    }
}

pub type GamesManagerData = web::Data<Arc<Mutex<GamesManager>>>;

macro_rules! get_lock {
    ($manager:expr, $game_id:expr) => {
        let game_lock = { $manager.lock().unwrap().get_game_mutex(&$game_id) };
        let _game_lock = game_lock.lock().unwrap();
    };
}

pub(crate) use get_lock;
