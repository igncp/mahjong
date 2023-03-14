use serde::{Deserialize, Serialize};

use crate::{
    meld::get_is_pair, Board, Deck, HandTile, Hands, Player, PlayerId, Round, Score, Table,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum GamePhase {
    Beginning,
    End,
    Playing,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    pub deck: Deck,
    pub id: String,
    pub name: String,
    pub phase: GamePhase,
    pub players: Vec<Player>,
    pub round: Round,
    pub score: Score,
    pub table: Table,
}

pub fn start_game(game: &mut Game) {
    game.phase = GamePhase::Playing;
}

pub fn can_say_mahjong(player_hand: Vec<HandTile>, deck: &Deck) -> bool {
    if player_hand.len() != 14 {
        return false;
    }

    let tiles_without_meld = player_hand
        .iter()
        .filter(|t| t.set_id.is_none())
        .map(|t| deck.get(&t.id).unwrap())
        .collect();

    get_is_pair(&tiles_without_meld)
}

pub struct ClaimTile {
    board: Board,
    hands: Hands,
    player_id: PlayerId,
    players: Vec<Player>,
    round: Round,
}

pub fn claim_tile(opts: &mut ClaimTile) -> bool {
    let player_hand = opts.hands.get_mut(&opts.player_id);
    if player_hand.is_none() {
        return false;
    }
    let player_hand = player_hand.unwrap();

    if player_hand.len() != 13 || opts.round.tile_claimed.is_none() || opts.board.is_empty() {
        return false;
    }

    let tile = opts.board.pop().unwrap();

    let mut tile_claimed = opts.round.tile_claimed.clone().unwrap();
    tile_claimed.by = Some(opts.player_id.clone());

    opts.round.tile_claimed = Some(tile_claimed);
    opts.round.player_index = opts
        .players
        .iter()
        .position(|p| p.id == opts.player_id)
        .unwrap();

    player_hand.push(HandTile {
        concealed: true,
        id: tile,
        set_id: None,
    });

    true
}
