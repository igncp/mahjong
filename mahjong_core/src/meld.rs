use crate::{Deck, Tile, TileId};

pub struct SetCheckOpts {
    // board_tile_player_diff: Option<usize>,
    // claimed_tile: Option<TileId>,
    pub deck: Deck,
    pub sub_hand: Vec<TileId>,
}

pub fn get_is_pung(opts: SetCheckOpts) -> bool {
    if opts.sub_hand.len() != 3 {
        return false;
    }

    let mut last_tile_id = opts.sub_hand[0];

    for tile_index in 1..3 {
        let tile_id = opts.sub_hand[tile_index];
        let last_tile = opts.deck.get(&last_tile_id);
        let tile = opts.deck.get(&tile_id);

        if tile.is_none() || last_tile.is_none() {
            return false;
        }

        let tile = tile.unwrap();
        let last_tile = last_tile.unwrap();

        match tile {
            Tile::Season(_) | Tile::Flower(_) => {
                return false;
            }
            other_tile => {
                if !other_tile.is_same_content(last_tile) {
                    return false;
                }
            }
        }

        last_tile_id = tile.get_id();
    }

    true
}
