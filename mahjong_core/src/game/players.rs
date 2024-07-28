use crate::macros::derive_game_common;
use rand::{seq::SliceRandom, thread_rng};
use ts_rs::TS;
use uuid::Uuid;

pub type PlayerId = String;

pub type PlayersVec = Vec<PlayerId>;

derive_game_common! {
#[derive(Default, TS)]
pub struct Players(pub PlayersVec);
}

impl Players {
    pub fn new_player() -> PlayerId {
        Uuid::new_v4().to_string()
    }
}

// Proxied methods
impl Players {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> std::slice::Iter<PlayerId> {
        self.0.iter()
    }

    pub fn get(&self, index: usize) -> Option<&PlayerId> {
        self.0.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn push(&mut self, player_id: PlayerId) {
        self.0.push(player_id);
    }

    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut thread_rng());
    }

    pub fn first(&self) -> &PlayerId {
        self.0.first().unwrap()
    }
}
