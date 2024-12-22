pub use self::decide_dealer::{DecideDealerWinds, SetInitialWindsError};
use crate::{
    game::GameStyle, macros::derive_game_common, Game, GamePhase, Hands, PlayerId, TileId, Wind,
    WINDS_ROUND_ORDER,
};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use ts_rs::TS;

mod decide_dealer;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct RoundTileClaimed {
    pub by: Option<PlayerId>,
    pub from: PlayerId,
    pub id: TileId,
}

pub type TileClaimed = Option<RoundTileClaimed>;

derive_game_common! {
#[derive(TS)]
#[ts(export)]
pub struct Round {
    pub consecutive_same_seats: usize,
    pub dealer_player_index: usize,
    pub player_index: usize,
    pub east_player_index: usize,
    pub round_index: u32,
    #[serde(skip)]
    pub style: GameStyle,
    pub tile_claimed: TileClaimed,
    pub wall_tile_drawn: Option<TileId>,
    pub wind: Wind,
    pub initial_winds: Option<u8>,
}}

#[derive(Debug, EnumIter, Eq, PartialEq, Clone)]
pub enum NextTurnError {
    StuckWallTileNotDrawn,
    StuckHandNotReady(PlayerId),
}

impl Round {
    pub fn new(game_style: &GameStyle) -> Self {
        // This assumes that the players array is sorted
        Self {
            consecutive_same_seats: 0,
            dealer_player_index: 0,
            player_index: 0,
            round_index: 0,
            style: game_style.clone(),
            tile_claimed: None,
            wall_tile_drawn: None,
            wind: Wind::East,
            east_player_index: 0,
            initial_winds: None,
        }
    }

    pub fn get_claimable_tile(&self, player_id: &PlayerId) -> Option<TileId> {
        let tile_claimed = self.tile_claimed.clone()?;

        if tile_claimed.by.is_some() || tile_claimed.from == *player_id {
            return None;
        }

        Some(tile_claimed.id)
    }

    pub fn get_initial_winds_slice(&self) -> DecideDealerWinds {
        DecideDealerWinds::from_number(self.initial_winds)
    }

    pub fn get_player_wind(&self, players: &[PlayerId], player_id: &PlayerId) -> Wind {
        let player_index = players.iter().position(|p| p == player_id).unwrap();
        let wind_index = (player_index + 4 - self.east_player_index) % 4;
        WINDS_ROUND_ORDER[wind_index].clone()
    }
}

impl Round {
    pub fn next_turn(&mut self, hands: &Hands) -> Result<(), NextTurnError> {
        if self.wall_tile_drawn.is_none() {
            return Err(NextTurnError::StuckWallTileNotDrawn);
        }

        let expected_tiles = hands.get_style().tiles_after_claim() - 1;

        for hand_player in hands.0.keys() {
            let hand = hands.get(hand_player);
            if hand.is_none() {
                return Err(NextTurnError::StuckHandNotReady(hand_player.clone()));
            }
            let hand = hand.unwrap();
            if hand.len() != expected_tiles {
                return Err(NextTurnError::StuckHandNotReady(hand_player.clone()));
            }
        }

        self.wall_tile_drawn = None;
        self.tile_claimed = None;

        self.player_index += 1;
        if self.player_index == Game::get_players_num(&self.style) {
            self.player_index = 0;
        }

        Ok(())
    }

    fn common_next_round(&mut self, phase: &mut GamePhase) {
        let mut current_wind_index = WINDS_ROUND_ORDER
            .iter()
            .position(|r| r == &self.wind)
            .unwrap();

        self.consecutive_same_seats = 0;
        self.dealer_player_index += 1;
        if self.dealer_player_index == Game::get_players_num(&self.style) {
            self.dealer_player_index = 0;
        }

        if self.dealer_player_index == self.east_player_index {
            current_wind_index += 1;

            if current_wind_index == WINDS_ROUND_ORDER.len() {
                *phase = GamePhase::End;
                return;
            }

            self.wind = WINDS_ROUND_ORDER.get(current_wind_index).unwrap().clone();
        }

        self.player_index = self.dealer_player_index;
    }

    pub fn move_after_win(&mut self, phase: &mut GamePhase, winner_player_index: usize) {
        self.wall_tile_drawn = None;
        self.tile_claimed = None;
        self.round_index += 1;

        let max_consecutive_same_seats = self.style.max_consecutive_same_seats();

        if winner_player_index == self.dealer_player_index
            && self.consecutive_same_seats < max_consecutive_same_seats
        {
            self.player_index = self.dealer_player_index;
            self.consecutive_same_seats += 1;
            return;
        }

        self.common_next_round(phase)
    }

    pub fn move_after_draw(&mut self, phase: &mut GamePhase) {
        self.wall_tile_drawn = None;
        self.tile_claimed = None;
        self.round_index += 1;

        let max_consecutive_same_seats = self.style.max_consecutive_same_seats();

        if self.consecutive_same_seats < max_consecutive_same_seats {
            self.player_index = self.dealer_player_index;
            self.consecutive_same_seats += 1;
            return;
        }

        self.common_next_round(phase)
    }

    pub fn set_initial_winds(
        &mut self,
        winds: Option<[Wind; 4]>,
    ) -> Result<(), SetInitialWindsError> {
        let winds = DecideDealerWinds::new(winds)?;
        self.initial_winds = winds.to_number();
        Ok(())
    }
}
