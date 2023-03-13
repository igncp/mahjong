use crate::{GamePhase, HandTile, Round, Wind, WINDS_ROUND_ORDER};

// This assumes that the players array is sorted
pub fn create_round() -> Round {
    Round {
        dealer_player_index: 0,
        player_index: 0,
        tile_claimed: None,
        wall_tile_drawn: None,
        wind: Wind::East,
    }
}

pub fn continue_round(round: &mut Round, hands: Vec<Vec<HandTile>>) -> bool {
    if round.wall_tile_drawn.is_none() {
        return false;
    }

    for hand in hands {
        if hand.len() != 13 {
            return false;
        }
    }

    round.wall_tile_drawn = None;
    round.tile_claimed = None;

    round.player_index += 1;
    if round.player_index == 4 {
        round.player_index = 0;
    }

    true
}

pub fn move_round_after_win(round: &mut Round, phase: &mut GamePhase) {
    round.wall_tile_drawn = None;
    round.tile_claimed = None;

    round.dealer_player_index += 1;
    if round.dealer_player_index == 4 {
        round.dealer_player_index = 0;
    }

    let current_wind_index = WINDS_ROUND_ORDER
        .iter()
        .position(|r| r == &round.wind)
        .unwrap();

    if round.dealer_player_index == current_wind_index {
        if current_wind_index == WINDS_ROUND_ORDER.len() - 1 {
            *phase = GamePhase::End;
        } else {
            round.dealer_player_index = current_wind_index + 1;

            round.wind = WINDS_ROUND_ORDER
                .get(round.dealer_player_index)
                .unwrap()
                .clone();
        }
    }

    round.player_index = round.dealer_player_index;
}
