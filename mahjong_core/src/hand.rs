use crate::{
    deck::DEFAULT_DECK,
    meld::{get_is_chow, get_is_kong, get_is_pair, get_is_pung, PlayerDiff, SetCheckOpts},
    TileId,
};
use rustc_hash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};

pub type SetIdContent = String;
pub type SetId = Option<SetIdContent>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HandPossibleMeld {
    pub is_mahjong: bool,
    pub tiles: Vec<TileId>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, juniper::GraphQLObject)]
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

type MeldsCollection<'a> = FxHashMap<String, Vec<&'a HandTile>>;

pub struct GetHandMeldsReturn<'a> {
    pub melds: MeldsCollection<'a>,
    pub tiles_without_meld: usize,
}

impl Hand {
    pub fn sort_default(&mut self) {
        self.0.sort_by(|a, b| {
            let tile_a = DEFAULT_DECK.0.get(&a.id);
            let tile_b = DEFAULT_DECK.0.get(&b.id);
            if tile_a.is_none() || tile_b.is_none() {
                return std::cmp::Ordering::Equal;
            }

            let (tile_a, tile_b) = (tile_a.unwrap(), tile_b.unwrap());

            tile_a.cmp_custom(tile_b)
        });
    }

    // `tiles` can be a sub-set of the whole hand
    pub fn sort_by_tiles(&mut self, tiles: &[TileId]) -> bool {
        let hand_copy = self.0.clone().iter().map(|t| t.id).collect::<Vec<TileId>>();
        let hand_set = hand_copy.iter().collect::<FxHashSet<&TileId>>();

        if tiles.iter().any(|t| !hand_set.contains(&t)) {
            return false;
        }

        self.0.sort_by(|a, b| {
            let tile_a = tiles.iter().position(|t| *t == a.id);
            let tile_b = tiles.iter().position(|t| *t == b.id);

            if tile_a.is_none() && tile_b.is_none() {
                return std::cmp::Ordering::Equal;
            }

            if tile_a.is_none() {
                return std::cmp::Ordering::Greater;
            }

            if tile_b.is_none() {
                return std::cmp::Ordering::Less;
            }

            let (tile_a, tile_b) = (tile_a.unwrap(), tile_b.unwrap());

            tile_a.cmp(&tile_b)
        });

        true
    }

    pub fn get_melds(&self) -> GetHandMeldsReturn {
        let mut melds: MeldsCollection = FxHashMap::default();
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

    pub fn can_say_mahjong(&self) -> bool {
        if self.0.len() != 14 {
            return false;
        }

        let tiles_without_meld = self
            .0
            .iter()
            .filter(|t| t.set_id.is_none())
            .map(|t| DEFAULT_DECK.0.get(&t.id).unwrap())
            .collect();

        get_is_pair(&tiles_without_meld)
    }

    pub fn get_possible_melds(
        &self,
        board_tile_player_diff: PlayerDiff,
        claimed_tile: Option<TileId>,
        check_for_mahjong: bool,
    ) -> Vec<HandPossibleMeld> {
        let hand_filtered: Vec<&HandTile> = self.0.iter().filter(|h| h.set_id.is_none()).collect();
        let mut melds: Vec<HandPossibleMeld> = vec![];

        if check_for_mahjong {
            if self.can_say_mahjong() {
                let tiles = self
                    .0
                    .iter()
                    .filter(|t| t.set_id.is_none())
                    .map(|t| t.id)
                    .collect();
                let meld = HandPossibleMeld {
                    is_mahjong: true,
                    tiles,
                };

                melds.push(meld);
            }

            return melds;
        }

        for first_tile_index in 0..hand_filtered.len() {
            let first_tile = hand_filtered[first_tile_index].id;
            let first_tile_full = DEFAULT_DECK.0.get(&first_tile);
            if first_tile_full.is_none() {
                continue;
            }
            let first_tile_full = first_tile_full.unwrap();

            for second_tile_index in (first_tile_index + 1)..hand_filtered.len() {
                let second_tile = hand_filtered[second_tile_index].id;
                let second_tile_full = DEFAULT_DECK.0.get(&second_tile);
                if second_tile_full.is_none() {
                    continue;
                }
                let second_tile_full = second_tile_full.unwrap();

                if !first_tile_full.is_same_type(second_tile_full) {
                    continue;
                }

                for third_tile_index in (second_tile_index + 1)..hand_filtered.len() {
                    let third_tile = hand_filtered[third_tile_index].id;
                    let third_tile_full = DEFAULT_DECK.0.get(&third_tile);
                    if third_tile_full.is_none() {
                        continue;
                    }
                    let third_tile_full = third_tile_full.unwrap();
                    if !first_tile_full.is_same_type(third_tile_full) {
                        continue;
                    }

                    let sub_hand = vec![first_tile, second_tile, third_tile];

                    let opts = SetCheckOpts {
                        board_tile_player_diff,
                        claimed_tile,
                        sub_hand: &sub_hand,
                    };

                    if get_is_pung(&opts) || get_is_chow(&opts) {
                        let meld = HandPossibleMeld {
                            is_mahjong: false,
                            tiles: sub_hand.clone(),
                        };
                        melds.push(meld);
                    }

                    for forth_tile in hand_filtered.iter().skip(third_tile_index + 1) {
                        let forth_tile_full = DEFAULT_DECK.0.get(&forth_tile.id);
                        if forth_tile_full.is_none() {
                            continue;
                        }
                        let forth_tile_full = forth_tile_full.unwrap();
                        if !first_tile_full.is_same_type(forth_tile_full) {
                            continue;
                        }

                        let mut full_sub_hand = sub_hand.clone();
                        full_sub_hand.push(forth_tile.id);
                        let mut opts = opts.clone();
                        opts.sub_hand = &full_sub_hand;

                        if get_is_kong(&opts) {
                            let meld = HandPossibleMeld {
                                is_mahjong: false,
                                tiles: full_sub_hand.clone(),
                            };
                            melds.push(meld);
                        }
                    }
                }
            }
        }

        melds
    }

    pub fn get_has_tile(&self, tile_id: &TileId) -> bool {
        self.0.iter().any(|t| t.id == *tile_id)
    }
}
