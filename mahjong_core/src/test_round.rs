#[cfg(test)]
mod test {
    use crate::{GamePhase, HandTile, Round, Wind};

    fn compare_rounds(round: &Round, expected_round: &Round, test_index: usize) {
        assert_eq!(
            round.player_index, expected_round.player_index,
            "test_index: {test_index}",
        );
        assert_eq!(
            round.dealer_player_index, expected_round.dealer_player_index,
            "test_index: {test_index}",
        );
        assert_eq!(
            round.tile_claimed, expected_round.tile_claimed,
            "test_index: {test_index}",
        );
        assert_eq!(round.wind, expected_round.wind, "test_index: {test_index}",);
        assert_eq!(
            round.wall_tile_drawn, expected_round.wall_tile_drawn,
            "test_index: {test_index}",
        );
    }

    type ContinueRoundFixture = (Round, Round);
    fn get_continue_round_fixtures() -> Vec<ContinueRoundFixture> {
        let fixtures: Vec<ContinueRoundFixture> = vec![
            (
                Round {
                    dealer_player_index: 2,
                    player_index: 2,
                    tile_claimed: None,
                    wind: Wind::West,
                    wall_tile_drawn: Some(3),
                },
                Round {
                    dealer_player_index: 2,
                    player_index: 3,
                    tile_claimed: None,
                    wind: Wind::West,
                    wall_tile_drawn: None,
                },
            ),
            (
                Round {
                    dealer_player_index: 3,
                    player_index: 3,
                    tile_claimed: None,
                    wind: Wind::West,
                    wall_tile_drawn: Some(3),
                },
                Round {
                    dealer_player_index: 3,
                    player_index: 0,
                    tile_claimed: None,
                    wind: Wind::West,
                    wall_tile_drawn: None,
                },
            ),
        ];

        fixtures
    }

    #[test]
    fn test_round_next() {
        for (test_index, (round, expected_round)) in
            get_continue_round_fixtures().iter().enumerate()
        {
            let mut round = round.clone();
            let mut hands: Vec<Vec<HandTile>> = vec![vec![]];

            for _ in 0..13 {
                hands[0].push(HandTile {
                    concealed: false,
                    id: 0,
                    set_id: None,
                });
            }

            round.next(hands);

            compare_rounds(&round, expected_round, test_index);
        }
    }

    type MoveRoundFixture = (Round, Round, GamePhase);
    fn get_move_rounds_fixtures() -> Vec<MoveRoundFixture> {
        let fixtures: Vec<MoveRoundFixture> = vec![
            (
                Round {
                    dealer_player_index: 2,
                    player_index: 2,
                    tile_claimed: None,
                    wind: Wind::West,
                    wall_tile_drawn: Some(2),
                },
                Round {
                    dealer_player_index: 3,
                    player_index: 3,
                    tile_claimed: None,
                    wind: Wind::West,
                    wall_tile_drawn: None,
                },
                GamePhase::Playing,
            ),
            (
                Round {
                    dealer_player_index: 0,
                    player_index: 3,
                    tile_claimed: None,
                    wind: Wind::South,
                    wall_tile_drawn: Some(2),
                },
                Round {
                    dealer_player_index: 2,
                    player_index: 2,
                    tile_claimed: None,
                    wind: Wind::West,
                    wall_tile_drawn: None,
                },
                GamePhase::Playing,
            ),
            (
                Round {
                    dealer_player_index: 2,
                    player_index: 1,
                    tile_claimed: None,
                    wind: Wind::North,
                    wall_tile_drawn: Some(1),
                },
                Round {
                    dealer_player_index: 3,
                    player_index: 3,
                    tile_claimed: None,
                    wind: Wind::North,
                    wall_tile_drawn: None,
                },
                GamePhase::End,
            ),
        ];

        fixtures
    }

    #[test]
    fn test_move_round_after_win() {
        for (test_index, (round, expected_round, expected_phase)) in
            get_move_rounds_fixtures().iter().enumerate()
        {
            let mut round = round.clone();
            let mut initial_phase = GamePhase::Playing;
            round.move_after_win(&mut initial_phase);

            compare_rounds(&round, expected_round, test_index);
            assert_eq!(initial_phase, *expected_phase, "test_index: {test_index}",);
        }
    }
}
