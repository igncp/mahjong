use crate::{
    meld::{get_is_chow, get_is_kong, get_is_pair, get_is_pung, Meld, PlayerDiff, SetCheckOpts},
    Deck, TileId,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type SetIdContent = String;
pub type SetId = Option<SetIdContent>;

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

    pub fn can_say_mahjong(&self, deck: &Deck) -> bool {
        if self.0.len() != 14 {
            return false;
        }

        let tiles_without_meld = self
            .0
            .iter()
            .filter(|t| t.set_id.is_none())
            .map(|t| deck.0.get(&t.id).unwrap())
            .collect();

        get_is_pair(&tiles_without_meld)
    }

    pub fn get_possible_melds(
        &self,
        board_tile_player_diff: PlayerDiff,
        claimed_tile: Option<TileId>,
        deck: &Deck,
    ) -> Vec<Meld> {
        let hand_filtered: Vec<HandTile> = self
            .0
            .iter()
            .filter(|h| h.set_id.is_none())
            .cloned()
            .collect();
        let mut melds: Vec<Meld> = vec![];

        for first_tile_index in 0..hand_filtered.len() {
            for second_tile_index in first_tile_index + 1..hand_filtered.len() {
                for third_tile_index in second_tile_index + 1..hand_filtered.len() {
                    let first_tile = hand_filtered[first_tile_index].id;
                    let second_tile = hand_filtered[second_tile_index].id;
                    let third_tile = hand_filtered[third_tile_index].id;
                    let sub_hand = vec![first_tile, second_tile, third_tile];

                    let opts = SetCheckOpts {
                        board_tile_player_diff,
                        claimed_tile,
                        deck,
                        sub_hand: &sub_hand,
                    };

                    if get_is_pung(&opts) || get_is_chow(&opts) {
                        melds.push(sub_hand.clone());
                    }

                    for forth_tile in hand_filtered.iter().skip(third_tile_index + 1) {
                        let mut full_sub_hand = sub_hand.clone();
                        full_sub_hand.push(forth_tile.id);
                        let mut opts = opts.clone();
                        opts.sub_hand = &full_sub_hand;

                        if get_is_kong(&opts) {
                            melds.push(full_sub_hand.clone());
                        }
                    }
                }
            }
        }

        if self.can_say_mahjong(deck) {
            melds.push(
                self.0
                    .iter()
                    .filter(|t| t.set_id.is_none())
                    .map(|t| t.id)
                    .collect(),
            );
        }

        melds
    }
}
