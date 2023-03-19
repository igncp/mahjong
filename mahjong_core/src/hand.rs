use crate::{Deck, TileId};
use serde::{Deserialize, Serialize};

pub type SetId = Option<String>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HandTile {
    pub concealed: bool,
    pub id: TileId,
    pub set_id: SetId,
}

impl HandTile {
    pub fn from_id(id: TileId) -> Self {
        Self {
            id,
            set_id: None,
            concealed: true,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Hand(pub Vec<HandTile>);

impl Hand {
    pub fn sort_default(&mut self, deck: &Deck) {
        self.0.sort_by(|a, b| {
            let tile_a = deck.0.get(&a.id);
            let tile_b = deck.0.get(&b.id);
            if tile_a.is_none() || tile_b.is_none() {
                return std::cmp::Ordering::Equal;
            }

            let (tile_a, tile_b) = (tile_a.unwrap(), tile_b.unwrap());

            tile_a.cmp_custom(tile_b)
        });
    }
}
