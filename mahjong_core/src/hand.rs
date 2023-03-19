use std::collections::HashMap;

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

type MeldsCollection<'a> = HashMap<String, Vec<&'a HandTile>>;

pub struct GetHandMeldsReturn<'a> {
    pub melds: MeldsCollection<'a>,
    pub tiles_without_meld: usize,
}

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

    pub fn get_melds(&self) -> GetHandMeldsReturn {
        let mut melds: MeldsCollection = HashMap::new();
        let mut tiles_without_meld = 0;

        for hand_tile in &self.0 {
            if hand_tile.set_id.is_none() {
                tiles_without_meld += 1;

                continue;
            }
            let set_id = hand_tile.set_id.clone().unwrap();
            let list = melds.get(&set_id);

            let mut list = match list {
                Some(list) => list.clone(),
                None => vec![],
            };

            list.push(hand_tile);

            melds.insert(set_id, list);
        }

        GetHandMeldsReturn {
            melds,
            tiles_without_meld,
        }
    }
}
