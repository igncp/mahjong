use crate::{
    deck::DEFAULT_DECK, round::TileClaimed, HandTile, PlayerId, SetId, Suit, SuitTile, Tile, TileId,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub type PlayerDiff = Option<i32>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub enum MeldType {
    Chow,
    Kong,
    Pair,
    Pung,
}

impl MeldType {
    pub fn from_tiles(tiles: &[TileId]) -> Option<Self> {
        if tiles.len() < 2 || tiles.len() > 4 {
            return None;
        }

        let tiles = tiles
            .iter()
            .map(|t| &DEFAULT_DECK.0[*t])
            .collect::<Vec<&Tile>>();

        let opts = SetCheckOpts {
            board_tile_player_diff: None,
            claimed_tile: None,
            sub_hand: &tiles,
        };

        if get_is_pung(&opts) {
            return Some(Self::Pung);
        }

        if get_is_chow(&opts) {
            return Some(Self::Chow);
        }

        if get_is_kong(&opts) {
            return Some(Self::Kong);
        }

        if get_is_pair(opts.sub_hand) {
            return Some(Self::Pair);
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct SetCheckOpts<'a> {
    pub board_tile_player_diff: PlayerDiff,
    pub claimed_tile: Option<TileId>,
    pub sub_hand: &'a [&'a Tile],
}

pub fn get_is_pung(opts: &SetCheckOpts) -> bool {
    if opts.sub_hand.len() != 3 {
        return false;
    }

    let mut last_tile = opts.sub_hand[0];
    if last_tile.is_bonus() {
        return false;
    }

    for tile_index in 1..3 {
        let tile = opts.sub_hand[tile_index];

        if !tile.is_same_content(last_tile) {
            return false;
        }

        last_tile = tile;
    }

    true
}

// This approach is used for performance
const DUMMY_SUIT: SuitTile = SuitTile {
    id: 0,
    value: 0,
    suit: crate::Suit::Dots,
};

pub fn get_is_chow(opts: &SetCheckOpts) -> bool {
    if opts.sub_hand.len() != 3 {
        return false;
    };

    if let Some(board_tile_player_diff) = opts.board_tile_player_diff {
        if let Some(claimed_tile) = opts.claimed_tile {
            if board_tile_player_diff != 1 {
                let has_same_claimed_tile =
                    opts.sub_hand.iter().any(|t| t.get_id() == claimed_tile);

                if has_same_claimed_tile {
                    return false;
                }
            }
        }
    }

    let mut suit_tiles: [&SuitTile; 3] = [&DUMMY_SUIT, &DUMMY_SUIT, &DUMMY_SUIT];
    let mut suit: Option<Suit> = None;

    for (idx, tile) in opts.sub_hand.iter().enumerate() {
        match tile {
            Tile::Suit(suit_tile) => {
                if suit.is_some() {
                    if Some(suit_tile.suit) != suit {
                        return false;
                    }
                } else {
                    suit = Some(suit_tile.suit);
                }
                suit_tiles[idx] = suit_tile
            }
            _ => {
                return false;
            }
        }
    }

    suit_tiles.sort_by(|a, b| a.value.cmp(&b.value));

    let mut last_tile = suit_tiles[0];

    for tile in suit_tiles.iter().skip(1).take(2) {
        if last_tile.value + 1 != tile.value {
            return false;
        }

        last_tile = tile;
    }

    true
}

pub fn get_is_kong(opts: &SetCheckOpts) -> bool {
    if opts.sub_hand.len() != 4 {
        return false;
    }

    let mut last_tile = opts.sub_hand[0];
    if last_tile.is_bonus() {
        return false;
    }

    for tile_index in 1..4 {
        let tile = opts.sub_hand[tile_index];

        if !tile.is_same_content(last_tile) {
            return false;
        }

        last_tile = tile;
    }

    true
}

pub fn get_tile_claimed_id_for_user(
    player_id: &PlayerId,
    tile_claimed: &TileClaimed,
) -> Option<TileId> {
    if tile_claimed.is_none() {
        return None;
    }

    let tile_claimed = tile_claimed.clone().unwrap();

    tile_claimed.clone().by?;

    if tile_claimed.by.unwrap() == *player_id {
        return Some(tile_claimed.id);
    }

    None
}

pub struct RemoveMeldOpts {
    hand: Vec<HandTile>,
    set_id: SetId,
}

pub fn remove_meld(opts: RemoveMeldOpts) {
    let mut meld_tiles = opts
        .hand
        .iter()
        .filter(|h| h.set_id == opts.set_id)
        .cloned()
        .collect::<Vec<HandTile>>();

    for meld_tile in meld_tiles.clone() {
        if !meld_tile.concealed {
            return;
        }
    }

    meld_tiles.iter_mut().for_each(|t| {
        t.set_id = None;
    });
}

pub fn get_is_pair(hand: &[&Tile]) -> bool {
    if hand.len() != 2 {
        return false;
    }

    hand[0].is_same_content(hand[1])
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct PossibleMeld {
    pub discard_tile: Option<TileId>,
    pub is_concealed: bool,
    pub is_mahjong: bool,
    pub is_upgrade: bool,
    pub player_id: PlayerId,
    pub tiles: Vec<TileId>,
}

impl PossibleMeld {
    pub fn sort_tiles(&mut self) {
        self.tiles.sort_by(|a, b| {
            let tile_a = &DEFAULT_DECK.0[*a];
            let tile_b = &DEFAULT_DECK.0[*b];

            tile_a.cmp_custom(tile_b)
        });
    }
}
