use crate::{
    game::GameVersion,
    meld::{PlayerDiff, PossibleMeld},
    Deck, Game, GameId, GamePhase, Hand, HandTile, PlayerId, Score, TileId, Wind,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundSummary {
    dealer_player_index: usize,
    pub player_index: usize,
    pub discarded_tile: Option<TileId>,
    wind: Wind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtherPlayerHand {
    pub tiles: usize,
    pub visible: Hand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSummary {
    pub board: Vec<TileId>,
    pub deck: Deck,
    pub draw_wall_count: usize,
    pub hand: Hand,
    pub id: GameId,
    pub other_hands: HashMap<PlayerId, OtherPlayerHand>,
    pub phase: GamePhase,
    pub player_id: PlayerId,
    pub players: Vec<PlayerId>,
    pub round: RoundSummary,
    pub score: Score,
    pub version: GameVersion,
}

impl GameSummary {
    pub fn from_game(game: &Game, player_id: &PlayerId) -> Option<Self> {
        let players = game.players.clone();
        let hand = game.table.hands.get(player_id).unwrap().clone();
        let deck = game.deck.clone();
        let phase = game.phase.clone();
        let score = game.score.clone();

        let discarded_tile = if game.round.tile_claimed.is_some() {
            let tile_claimed = game.round.tile_claimed.as_ref().unwrap();
            if tile_claimed.by.is_none() {
                Some(tile_claimed.id)
            } else {
                None
            }
        } else {
            None
        };

        let round = RoundSummary {
            dealer_player_index: game.round.dealer_player_index,
            discarded_tile,
            player_index: game.round.player_index,
            wind: game.round.wind.clone(),
        };

        let draw_wall_count = game.table.draw_wall.len();
        let board = game.table.board.clone();
        let other_hands = game
            .table
            .hands
            .iter()
            .filter(|(id, _)| id != &player_id)
            .map(|(id, other_hand)| {
                (
                    id.clone(),
                    OtherPlayerHand {
                        tiles: other_hand.0.len(),
                        visible: Hand(
                            other_hand
                                .0
                                .iter()
                                .cloned()
                                .filter(|t| !t.concealed)
                                .collect(),
                        ),
                    },
                )
            })
            .collect();

        Some(Self {
            board,
            deck,
            draw_wall_count,
            hand,
            id: game.id.clone(),
            other_hands,
            phase,
            player_id: player_id.clone(),
            players,
            round,
            score,
            version: game.version.clone(),
        })
    }

    pub fn get_current_player(&self) -> &PlayerId {
        &self.players[self.round.player_index]
    }

    pub fn get_possible_melds(&self) -> Vec<PossibleMeld> {
        let mut possible_melds: Vec<PossibleMeld> = vec![];
        let can_claim_tile = self.hand.0.len() == 13 && self.round.discarded_tile.is_some();

        let mut claimed_tile: Option<TileId> = None;
        let mut tested_hand = self.hand.clone();
        let mut player_diff: PlayerDiff = None;
        let player_index = self
            .players
            .iter()
            .position(|p| p == &self.player_id)
            .unwrap();
        let current_player_index = self.round.player_index;

        if can_claim_tile {
            let tile_id = self.round.discarded_tile.unwrap();
            let tile = HandTile {
                concealed: false,
                id: tile_id,
                set_id: None,
            };

            tested_hand.0.push(tile);
            claimed_tile = Some(tile_id);
            player_diff = Some(player_index as i32 - current_player_index as i32);
        }

        let raw_melds = tested_hand.get_possible_melds(player_diff, claimed_tile, &self.deck);

        for raw_meld in raw_melds {
            let possible_meld = PossibleMeld {
                discard_tile: None,
                player_id: self.player_id.clone(),
                tiles: raw_meld.clone(),
            };

            possible_melds.push(possible_meld);
        }

        possible_melds
    }
}
