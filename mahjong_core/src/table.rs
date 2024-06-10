use serde::{Deserialize, Serialize};

use crate::{Hands, TileId};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Board(pub Vec<TileId>);
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct DrawWall(pub Vec<TileId>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    pub board: Board,
    pub draw_wall: DrawWall,
    pub hands: Hands,
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
}
