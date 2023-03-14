use crate::{Deck, HandTile, Hands, Player, PlayerId, Round, SetId, SuitTile, Tile, TileId};

pub type PlayerDiff = Option<i32>;

#[derive(Debug, Clone)]
pub struct SetCheckOpts<'a> {
    pub board_tile_player_diff: PlayerDiff,
    pub claimed_tile: Option<TileId>,
    pub deck: &'a Deck,
    pub sub_hand: &'a Vec<TileId>,
}

pub fn get_is_pung(opts: &SetCheckOpts) -> bool {
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

    for tile_id in opts.sub_hand.clone() {
        let tile = opts.deck.get(&tile_id);

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
        let last_tile = opts.deck.get(&last_tile_id);
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

pub struct GetBoardTilePlayerDiff {
    hand: Vec<HandTile>,
    players: Vec<Player>,
    round: Round,
    player_id: PlayerId,
}

pub fn get_board_tile_player_diff(opts: GetBoardTilePlayerDiff) -> PlayerDiff {
    let tile_claimed = opts.round.tile_claimed;

    if let Some(tile_claimed) = tile_claimed {
        tile_claimed.by?;

        if !opts.hand.iter().any(|h| h.id == tile_claimed.id) {
            return None;
        }

        let player_index = opts.players.iter().position(|p| p.id == opts.player_id);
        let other_player_index = opts.players.iter().position(|p| p.id == tile_claimed.from);

        if player_index.is_none() || other_player_index.is_none() {
            return None;
        }

        let player_index = player_index.unwrap();
        let other_player_index = other_player_index.unwrap();

        return Some(player_index as i32 - other_player_index as i32);
    }

    None
}

pub struct GetPossibleMelds {
    pub board_tile_player_diff: PlayerDiff,
    pub claimed_tile: Option<TileId>,
    pub deck: Deck,
    pub hand: Vec<HandTile>,
}

pub type Meld = Vec<TileId>;

pub fn get_possible_melds(opts: &GetPossibleMelds) -> Vec<Meld> {
    let hand_filtered: Vec<HandTile> = opts
        .hand
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
                    board_tile_player_diff: opts.board_tile_player_diff,
                    claimed_tile: opts.claimed_tile,
                    deck: &opts.deck,
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

    melds
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

pub fn get_is_pair(hand: &Vec<&Tile>) -> bool {
    if hand.len() != 2 {
        return false;
    }

    hand[0].is_same_content(hand[1])
}

pub struct BreakMeldOpts {
    hands: Hands,
    player_id: PlayerId,
    set_id: SetId,
}

pub fn break_meld(opts: &mut BreakMeldOpts) -> bool {
    let hand = opts.hands.get(&opts.player_id);

    if hand.is_none() {
        return false;
    }

    let mut meld_tiles = hand
        .unwrap()
        .iter()
        .filter(|h| h.set_id == opts.set_id)
        .cloned()
        .collect::<Vec<HandTile>>();

    if meld_tiles.is_empty() {
        return false;
    }

    if meld_tiles.iter().any(|t| !t.concealed) {
        return false;
    }

    meld_tiles.iter_mut().for_each(|t| {
        t.set_id = None;
    });

    true
}
