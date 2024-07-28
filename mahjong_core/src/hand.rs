use crate::{
    deck::DEFAULT_DECK,
    game::GameStyle,
    meld::{
        get_is_chow, get_is_kong, get_is_pair, get_is_pung, PlayerDiff, PossibleMeld, SetCheckOpts,
    },
    PlayerId, Tile, TileId,
};
use rustc_hash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use ts_rs::TS;

pub type SetIdContent = String;
pub type SetId = Option<SetIdContent>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HandPossibleMeld {
    pub is_mahjong: bool,
    pub tiles: Vec<TileId>,
}

impl From<PossibleMeld> for HandPossibleMeld {
    fn from(meld: PossibleMeld) -> Self {
        Self {
            is_mahjong: meld.is_mahjong,
            tiles: meld.tiles.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TS)]
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
    pub fn from_tile(tile: &Tile) -> Self {
        Self {
            id: tile.get_id(),
            set_id: None,
            concealed: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, TS)]
pub struct Hand {
    pub list: Vec<HandTile>,
    #[serde(skip)]
    pub style: Option<GameStyle>,
}

type MeldsCollection<'a> = FxHashMap<String, Vec<&'a HandTile>>;

pub struct GetHandMeldsReturn<'a> {
    pub melds: MeldsCollection<'a>,
    pub tiles_without_meld: usize,
}

// Proxied
impl Hand {
    pub fn get(&self, index: usize) -> &HandTile {
        self.list.get(index).unwrap()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn push(&mut self, tile: HandTile) {
        self.list.push(tile)
    }
}

#[derive(Debug, EnumIter, Eq, PartialEq, Clone)]
pub enum SortHandError {
    NotSortedMissingTile,
}

#[derive(Debug, EnumIter, Eq, PartialEq, Clone)]
pub enum CanSayMahjongError {
    CantDrop,
    NotPair,
}

impl Hand {
    pub fn new(list: Vec<HandTile>) -> Self {
        Self { list, style: None }
    }

    pub fn from_ref_vec(tiles: &[&HandTile]) -> Self {
        Self {
            list: tiles.iter().cloned().cloned().collect(),
            style: None,
        }
    }

    pub fn from_ids(tiles: &[TileId]) -> Self {
        Self {
            list: tiles.iter().cloned().map(HandTile::from_id).collect(),
            style: None,
        }
    }

    pub fn sort_default(&mut self) {
        self.list.sort_by(|a, b| {
            let tile_a = &DEFAULT_DECK.0.get(a.id);
            let tile_b = &DEFAULT_DECK.0.get(b.id);

            if tile_a.is_none() || tile_b.is_none() {
                return std::cmp::Ordering::Equal;
            }

            tile_a.unwrap().cmp_custom(tile_b.unwrap())
        });
    }

    // `tiles` can be a sub-set of the whole hand
    pub fn sort_by_tiles(&mut self, tiles: &[TileId]) -> Result<(), SortHandError> {
        let hand_copy = self
            .list
            .clone()
            .iter()
            .map(|t| t.id)
            .collect::<Vec<TileId>>();
        let hand_set = hand_copy.iter().collect::<FxHashSet<&TileId>>();

        if tiles.iter().any(|t| !hand_set.contains(&t)) {
            return Err(SortHandError::NotSortedMissingTile);
        }

        self.list.sort_by(|a, b| {
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

        Ok(())
    }

    pub fn get_melds(&self) -> GetHandMeldsReturn {
        let mut melds: MeldsCollection = FxHashMap::default();
        let mut tiles_without_meld = 0;

        for hand_tile in &self.list {
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

    pub fn can_say_mahjong(&self) -> Result<(), CanSayMahjongError> {
        if !self.can_drop_tile() {
            return Err(CanSayMahjongError::CantDrop);
        }

        let tiles_without_meld: Vec<&Tile> = self
            .list
            .iter()
            .filter(|t| t.set_id.is_none())
            .map(|t| &DEFAULT_DECK.0[t.id])
            .collect();

        let is_pair = get_is_pair(&tiles_without_meld);

        if !is_pair {
            return Err(CanSayMahjongError::NotPair);
        }

        Ok(())
    }

    pub fn get_possible_melds(
        &self,
        board_tile_player_diff: PlayerDiff,
        claimed_tile: Option<TileId>,
        check_for_mahjong: bool,
    ) -> Vec<HandPossibleMeld> {
        let hand_filtered: Vec<&HandTile> =
            self.list.iter().filter(|h| h.set_id.is_none()).collect();
        let mut melds: Vec<HandPossibleMeld> = vec![];

        if check_for_mahjong {
            if self.can_say_mahjong().is_ok() {
                let tiles = self
                    .list
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
            let first_tile_full = &DEFAULT_DECK.0[first_tile];

            for second_tile_index in (first_tile_index + 1)..hand_filtered.len() {
                let second_tile = hand_filtered[second_tile_index].id;
                let second_tile_full = &DEFAULT_DECK.0[second_tile];

                if !first_tile_full.is_same_type(second_tile_full) {
                    continue;
                }

                for third_tile_index in (second_tile_index + 1)..hand_filtered.len() {
                    let third_tile = hand_filtered[third_tile_index].id;
                    let third_tile_full = &DEFAULT_DECK.0[third_tile];
                    if !first_tile_full.is_same_type(third_tile_full) {
                        continue;
                    }

                    let sub_hand = [first_tile_full, second_tile_full, third_tile_full];

                    let opts = SetCheckOpts {
                        board_tile_player_diff,
                        claimed_tile,
                        sub_hand: &sub_hand,
                    };

                    if get_is_pung(&opts) || get_is_chow(&opts) {
                        let meld = HandPossibleMeld {
                            is_mahjong: false,
                            tiles: vec![first_tile, second_tile, third_tile],
                        };
                        melds.push(meld);
                    }

                    for forth_tile in hand_filtered.iter().skip(third_tile_index + 1) {
                        let forth_tile_full = &DEFAULT_DECK.0[forth_tile.id];

                        let mut opts = opts.clone();
                        let sub_hand_inner = [
                            first_tile_full,
                            second_tile_full,
                            third_tile_full,
                            forth_tile_full,
                        ];
                        opts.sub_hand = &sub_hand_inner;

                        if get_is_kong(&opts) {
                            let meld = HandPossibleMeld {
                                is_mahjong: false,
                                tiles: vec![first_tile, second_tile, third_tile, forth_tile.id],
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
        self.list.iter().any(|t| t.id == *tile_id)
    }

    pub fn get_sets_groups(&self) -> FxHashMap<SetId, Vec<&HandTile>> {
        let mut sets: FxHashMap<SetId, Vec<&HandTile>> = FxHashMap::default();

        for tile in &self.list {
            let set_id = tile.set_id.clone();
            let list = sets.get(&set_id);

            let mut list = match list {
                Some(list) => list.clone(),
                None => vec![],
            };

            list.push(tile);

            sets.insert(set_id, list);
        }

        sets
    }

    pub fn can_drop_tile(&self) -> bool {
        self.list.len()
            == self
                .style
                .as_ref()
                .unwrap_or(&GameStyle::HongKong)
                .tiles_after_claim()
    }
}

impl From<Hand> for Vec<TileId> {
    fn from(hand: Hand) -> Self {
        hand.list.iter().map(|t| t.id).collect()
    }
}

pub type HandsMap = FxHashMap<PlayerId, Hand>;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default, TS)]
#[ts(export)]
pub struct Hands(pub HandsMap);

// Proxied
impl Hands {
    pub fn get(&self, player: &PlayerId) -> Option<Hand> {
        self.0.get(player).cloned()
    }

    pub fn remove(&mut self, player: &PlayerId) -> Hand {
        self.0.remove(player).unwrap()
    }

    pub fn insert(&mut self, player: impl AsRef<str>, hand: Hand) -> Option<Hand> {
        self.0.insert(player.as_ref().to_string(), hand)
    }
}

impl Hands {
    pub fn get_player_hand_len(&self, player: &str) -> usize {
        self.0.get(player).unwrap().len()
    }

    pub fn insert_ids(&mut self, player: &str, tiles: &[TileId]) -> &mut Self {
        self.0.insert(player.to_string(), Hand::from_ids(tiles));
        self
    }

    pub fn sort_player_hand(&mut self, player: &PlayerId) {
        let mut hand = self.0.get(player).unwrap().clone();
        hand.sort_default();
        self.0.insert(player.clone(), hand);
    }

    pub fn get_style(&self) -> GameStyle {
        self.0
            .values()
            .next()
            .cloned()
            .unwrap()
            .style
            .unwrap_or_default()
    }
}
