use crate::{
    game::{GameStyle, GameVersion, Players},
    meld::{PlayerDiff, PossibleMeld},
    Board, Game, GameId, GamePhase, Hand, HandTile, Hands, PlayerId, Score, TileId, Wind,
};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

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
pub struct OtherPlayerHands(pub FxHashMap<PlayerId, OtherPlayerHand>);

impl OtherPlayerHands {
    pub fn from_hands(hands: &Hands, player_id: &PlayerId) -> Self {
        let mut other_hands = FxHashMap::default();

        for (id, hand) in hands.0.iter() {
            if id != player_id {
                let visible_tiles = hand.0.iter().filter(|t| !t.concealed).cloned().collect();
                other_hands.insert(
                    id.clone(),
                    OtherPlayerHand {
                        tiles: hand.len(),
                        visible: Hand(visible_tiles),
                    },
                );
            }
        }

        Self(other_hands)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSummary {
    pub board: Board,
    pub draw_wall_count: usize,
    pub hand: Hand,
    pub id: GameId,
    pub other_hands: OtherPlayerHands,
    pub phase: GamePhase,
    pub player_id: PlayerId,
    pub players: Players,
    pub round: RoundSummary,
    pub score: Score,
    pub version: GameVersion,
    pub style: GameStyle,
}

impl GameSummary {
    pub fn from_game(game: &Game, player_id: &PlayerId) -> Option<Self> {
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
        let other_hands = OtherPlayerHands::from_hands(&game.table.hands, player_id);

        Some(Self {
            board: game.table.board.clone(),
            draw_wall_count,
            hand: game.table.hands.get(player_id).clone(),
            id: game.id.clone(),
            other_hands,
            phase: game.phase.clone(),
            player_id: player_id.clone(),
            players: game.players.clone(),
            round,
            score: game.score.clone(),
            version: game.version.clone(),
            style: game.style.clone(),
        })
    }

    pub fn get_current_player(&self) -> &PlayerId {
        &self.players.0[self.round.player_index]
    }

    fn get_can_claim_tile(&self) -> bool {
        self.hand.len() < self.style.tiles_after_claim() && self.round.discarded_tile.is_some()
    }

    pub fn get_possible_melds(&self) -> Vec<PossibleMeld> {
        let mut possible_melds: Vec<PossibleMeld> = vec![];
        let can_claim_tile = self.get_can_claim_tile();

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
                concealed: true,
                id: tile_id,
                set_id: None,
            };

            tested_hand.0.push(tile);
            claimed_tile = Some(tile_id);
            player_diff = Some(match player_index as i32 - current_player_index as i32 {
                -3 => 1,
                val => val,
            });
        }

        let mut raw_melds = tested_hand.get_possible_melds(player_diff, claimed_tile, true);

        tested_hand
            .get_possible_melds(player_diff, claimed_tile, false)
            .iter()
            .for_each(|m| {
                raw_melds.push(m.clone());
            });

        for raw_meld in raw_melds {
            let possible_meld = PossibleMeld {
                discard_tile: None,
                is_mahjong: raw_meld.is_mahjong,
                player_id: self.player_id.clone(),
                tiles: raw_meld.tiles.clone(),
            };

            possible_melds.push(possible_meld);
        }

        possible_melds
    }
}
