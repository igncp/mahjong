pub use self::draw_wall::{DrawWall, DrawWallPlace, PositionTilesOpts};
use crate::{Hands, PlayerId, TileId};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

mod draw_wall;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, TS)]
#[ts(export)]
pub struct Board(pub Vec<TileId>);

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, TS)]
#[ts(export)]
pub struct BonusTiles(pub FxHashMap<PlayerId, Vec<TileId>>);

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
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

impl BonusTiles {
    pub fn get_or_create(&mut self, player_id: &PlayerId) -> &mut Vec<TileId> {
        self.0.entry(player_id.clone()).or_default()
    }
}
