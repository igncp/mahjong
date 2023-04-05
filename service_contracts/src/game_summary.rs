use mahjong_core::{
    meld::PossibleMeld, Deck, Game, GameId, GamePhase, Hand, PlayerId, Score, TileId, Wind,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundSummary {
    dealer_player_index: usize,
    pub player_index: usize,
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
    pub phase: GamePhase,
    pub player_id: PlayerId,
    pub players: Vec<PlayerId>,
    pub other_hands: HashMap<PlayerId, OtherPlayerHand>,
    pub round: RoundSummary,
    pub score: Score,
}

impl GameSummary {
    pub fn from_game(game: &Game, player_id: &PlayerId) -> Option<Self> {
        let players = game.players.clone();
        let hand = game.table.hands.get(player_id).unwrap().clone();
        let deck = game.deck.clone();
        let phase = game.phase.clone();
        let score = game.score.clone();

        let round = RoundSummary {
            dealer_player_index: game.round.dealer_player_index,
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
        })
    }

    pub fn get_current_player(&self) -> &PlayerId {
        &self.players[self.round.player_index]
    }

    pub fn get_possible_melds(&self) -> Vec<PossibleMeld> {
        let mut possible_melds: Vec<PossibleMeld> = vec![];
        let raw_melds = self.hand.get_possible_melds(None, None, &self.deck);

        // TODO: Handle the already discarded tile

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
