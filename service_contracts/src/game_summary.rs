use mahjong_core::{Deck, Game, GameId, GamePhase, Hand, PlayerId, Score, TileId, Wind};
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
            .map(|(id, hand)| {
                (
                    id.clone(),
                    OtherPlayerHand {
                        tiles: hand.0.len(),
                        visible: Hand(hand.0.iter().cloned().filter(|t| !t.concealed).collect()),
                    },
                )
            })
            .collect();

        Some(GameSummary {
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
}
