#[cfg(test)]
mod test {
    use crate::{game::GameStyle, round::Round, GamePhase, Hand, HandTile, Hands, Wind};
    use pretty_assertions::assert_eq;
    use rustc_hash::FxHashMap;

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
                    wind: Wind::West,
                    wall_tile_drawn: Some(3),
                    ..Round::new(&GameStyle::HongKong)
                },
                Round {
                    dealer_player_index: 2,
                    player_index: 3,
                    wind: Wind::West,
                    wall_tile_drawn: None,
                    ..Round::new(&GameStyle::HongKong)
                },
            ),
            (
                Round {
                    dealer_player_index: 3,
                    player_index: 3,
                    wind: Wind::West,
                    wall_tile_drawn: Some(3),
                    ..Round::new(&GameStyle::HongKong)
                },
                Round {
                    dealer_player_index: 3,
                    player_index: 0,
                    wind: Wind::West,
                    wall_tile_drawn: None,
                    ..Round::new(&GameStyle::HongKong)
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
            let mut hands = Hands(FxHashMap::default());

            hands.0.insert("0".to_string(), Hand::default());

            for _ in 0..13 {
                hands.0.get_mut("0").unwrap().0.push(HandTile {
                    concealed: false,
                    id: 0,
                    set_id: None,
                });
            }

            round.next(&hands);

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
                    wind: Wind::West,
                    wall_tile_drawn: Some(2),
                    ..Round::new(&GameStyle::HongKong)
                },
                Round {
                    dealer_player_index: 3,
                    player_index: 3,
                    wind: Wind::West,
                    ..Round::new(&GameStyle::HongKong)
                },
                GamePhase::Playing,
            ),
            (
                Round {
                    dealer_player_index: 0,
                    player_index: 3,
                    wind: Wind::South,
                    wall_tile_drawn: Some(2),
                    ..Round::new(&GameStyle::HongKong)
                },
                Round {
                    dealer_player_index: 2,
                    player_index: 2,
                    wind: Wind::West,
                    ..Round::new(&GameStyle::HongKong)
                },
                GamePhase::Playing,
            ),
            (
                Round {
                    dealer_player_index: 2,
                    player_index: 1,
                    wind: Wind::North,
                    wall_tile_drawn: Some(1),
                    ..Round::new(&GameStyle::HongKong)
                },
                Round {
                    dealer_player_index: 3,
                    player_index: 3,
                    wind: Wind::North,
                    ..Round::new(&GameStyle::HongKong)
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
