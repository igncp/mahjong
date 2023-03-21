use mahjong_core::{Deck, Game, GameId, GamePhase, Hand, Player, PlayerId, Score, TileId, Wind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundSummary {
    dealer_player_index: usize,
    pub player_index: usize,
    wind: Wind,
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
    pub players: Vec<Player>,
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

        Some(GameSummary {
            board,
            score,
            deck,
            draw_wall_count,
            hand,
            id: game.id.clone(),
            phase,
            player_id: player_id.clone(),
            players,
            round,
        })
    }

    pub fn get_current_player(&self) -> &Player {
        &self.players[self.round.player_index]
    }
}
