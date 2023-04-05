use crate::{GamePhase, Hands, Round, Wind, WINDS_ROUND_ORDER};

impl Default for Round {
    fn default() -> Self {
        // This assumes that the players array is sorted
        Self {
            dealer_player_index: 0,
            player_index: 0,
            tile_claimed: None,
            wall_tile_drawn: None,
            wind: Wind::East,
        }
    }
}

impl Round {
    pub fn next(&mut self, hands: &Hands) -> bool {
        if self.wall_tile_drawn.is_none() {
            return false;
        }

        for hand in hands.values() {
            if hand.0.len() != 13 {
                return false;
            }
        }

        self.wall_tile_drawn = None;
        self.tile_claimed = None;

        self.player_index += 1;
        if self.player_index == 4 {
            self.player_index = 0;
        }

        true
    }

    pub fn move_after_win(&mut self, phase: &mut GamePhase) {
        self.wall_tile_drawn = None;
        self.tile_claimed = None;

        self.dealer_player_index += 1;
        if self.dealer_player_index == 4 {
            self.dealer_player_index = 0;
        }

        let current_wind_index = WINDS_ROUND_ORDER
            .iter()
            .position(|r| r == &self.wind)
            .unwrap();

        if self.dealer_player_index == current_wind_index {
            if current_wind_index == WINDS_ROUND_ORDER.len() - 1 {
                *phase = GamePhase::End;
            } else {
                self.dealer_player_index = current_wind_index + 1;

                self.wind = WINDS_ROUND_ORDER
                    .get(self.dealer_player_index)
                    .unwrap()
                    .clone();
            }
        }

        self.player_index = self.dealer_player_index;
    }
}
