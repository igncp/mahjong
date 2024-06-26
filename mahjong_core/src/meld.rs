use crate::{
    deck::DEFAULT_DECK, round::TileClaimed, HandTile, PlayerId, SetId, SuitTile, Tile, TileId,
};
use serde::{Deserialize, Serialize};

pub type PlayerDiff = Option<i32>;

#[derive(Debug, Clone)]
pub struct SetCheckOpts<'a> {
    pub board_tile_player_diff: PlayerDiff,
    pub claimed_tile: Option<TileId>,
    pub sub_hand: &'a Vec<TileId>,
}

pub fn get_is_pung(opts: &SetCheckOpts) -> bool {
    if opts.sub_hand.len() != 3 {
        return false;
    }

    let mut last_tile_id = opts.sub_hand[0];

    for tile_index in 1..3 {
        let tile_id = opts.sub_hand[tile_index];
        let last_tile = DEFAULT_DECK.0.get(&last_tile_id);
        let tile = DEFAULT_DECK.0.get(&tile_id);

        if tile.is_none() || last_tile.is_none() {
            return false;
        }

        let tile = tile.unwrap();

        if tile.is_bonus() {
            return false;
        }

        let last_tile = last_tile.unwrap();

        if !tile.is_same_content(last_tile) {
            return false;
        }

        last_tile_id = tile.get_id();
    }

    true
}

pub fn get_is_chow(opts: &SetCheckOpts) -> bool {
    if opts.sub_hand.len() != 3 {
        return false;
    };

    if let Some(board_tile_player_diff) = opts.board_tile_player_diff {
        if let Some(claimed_tile) = opts.claimed_tile {
            if board_tile_player_diff != 1 {
                let has_same_claimed_tile = opts.sub_hand.iter().any(|t| t == &claimed_tile);

                if has_same_claimed_tile {
                    return false;
                }
            }
        }
    }

    let mut suit_tiles: Vec<SuitTile> = vec![];

    for tile_id in opts.sub_hand {
        let tile = DEFAULT_DECK.0.get(tile_id);

        if tile.is_none() {
            return false;
        }

        let tile = tile.unwrap();

        match tile {
            Tile::Suit(suit_tile) => {
                suit_tiles.push(suit_tile.clone());
            }
            _ => {
                return false;
            }
        }
    }

    suit_tiles.sort_by(|a, b| a.value.cmp(&b.value));

    let mut last_tile_id = suit_tiles[0].id;

    for tile_index in 1..3 {
        let last_tile = DEFAULT_DECK.0.get(&last_tile_id);
        let tile = suit_tiles.get(tile_index);

        if tile.is_none() || last_tile.is_none() {
            return false;
        }

        let tile = tile.unwrap();
        let last_tile = last_tile.unwrap();

        let last_tile = match last_tile {
            Tile::Suit(suit_tile) => suit_tile,
            _ => {
                return false;
            }
        };

        if last_tile.suit != tile.suit || last_tile.value + 1 != tile.value {
            return false;
        }

        last_tile_id = tile.id;
    }

    true
}

pub fn get_is_kong(opts: &SetCheckOpts) -> bool {
    if opts.sub_hand.len() != 4 {
        return false;
    }

    let mut last_tile_id = opts.sub_hand[0];

    for tile_index in 1..4 {
        let tile_id = opts.sub_hand[tile_index];
        let last_tile = DEFAULT_DECK.0.get(&last_tile_id);
        let tile = DEFAULT_DECK.0.get(&tile_id);

        if tile.is_none() || last_tile.is_none() {
            return false;
        }

        let tile = tile.unwrap();
        let last_tile = last_tile.unwrap();

        if tile.is_bonus() || !tile.is_same_content(last_tile) {
            return false;
        }

        last_tile_id = tile_id;
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PossibleMeld {
    pub discard_tile: Option<TileId>,
    pub is_mahjong: bool,
    pub player_id: PlayerId,
    pub tiles: Vec<TileId>,
}

impl PossibleMeld {
    pub fn sort_tiles(&mut self) {
        self.tiles.sort_by(|a, b| {
            let tile_a = &DEFAULT_DECK.0[a];
            let tile_b = &DEFAULT_DECK.0[b];

            tile_a.cmp_custom(tile_b)
        });
    }
}
