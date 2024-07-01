use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

use crate::{Hands, PlayerId, TileId};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Board(pub Vec<TileId>);
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct DrawWall(pub Vec<TileId>);
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct BonusTiles(pub FxHashMap<PlayerId, Vec<TileId>>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    pub board: Board,
    pub draw_wall: DrawWall,
    pub hands: Hands,
    pub bonus_tiles: BonusTiles,
}

// Proxied
impl Board {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// Proxied
impl DrawWall {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn pop(&mut self) -> Option<TileId> {
        self.0.pop()
    }
}

impl BonusTiles {
    pub fn get_or_create(&mut self, player_id: &PlayerId) -> &mut Vec<TileId> {
        self.0.entry(player_id.clone()).or_default()
    }
}
