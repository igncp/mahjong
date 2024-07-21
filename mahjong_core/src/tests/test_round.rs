#[cfg(test)]
mod test {
    use crate::{
        game::GameStyle,
        round::{DecideDealerWinds, NextTurnError, Round},
        Game, GamePhase, Wind, WINDS_ROUND_ORDER,
    };
    use pretty_assertions::assert_eq;
    use strum::IntoEnumIterator;

    fn compare_rounds(round: &Round, expected_round: &Round, test_index: usize) {
        assert_eq!(
            round.player_index, expected_round.player_index,
            "player index check - test_index: {test_index}, {:?}, {:?}",
            round, expected_round,
        );
        assert_eq!(
            round.dealer_player_index, expected_round.dealer_player_index,
            "dealer index check - test_index: {test_index}",
        );
        assert_eq!(
            round.tile_claimed, expected_round.tile_claimed,
            "tile claimed check - test_index: {test_index}",
        );
        assert_eq!(round.wind, expected_round.wind, "test_index: {test_index}",);
        assert_eq!(
            round.wall_tile_drawn, expected_round.wall_tile_drawn,
            "wall tile check - test_index: {test_index}",
        );
        assert_eq!(
            round.consecutive_same_seats, expected_round.consecutive_same_seats,
            "consecutive game seats - test_index: {test_index}",
        );
        assert_eq!(
            round.round_index, expected_round.round_index,
            "round index - test_index: {test_index}",
        );
    }

    const NEXT_ROUND_FIXTURES: &[(&str, &str)] = &[
        (
            // The round should not change when moving to then next turn
            "Turn: P3, Dealer: P3, Round: 5
             Drawn: 一萬",
            "Turn: P4, Dealer: P3, Round: 5",
        ),
        (
            "Turn: P4, Dealer: P4
             Drawn: 一萬",
            "Turn: P1, Dealer: P4",
        ),
        (
            // No tile drawn, can't move next
            "Turn: P4, Dealer: P4, Round: 5",
            "Turn: P4, Dealer: P4, Round: 5",
        ),
        (
            // Invalid hand, can't move next
            "- P1: 一萬
             Turn: P3, Dealer: P3, Round: 5
             Drawn: 一萬",
            "Turn: P3, Dealer: P3, Round: 5
             Drawn: 一萬",
        ),
    ];

    #[test]
    fn test_round_next_turn() {
        for (test_index, (round_summary, expected_round_summary)) in
            NEXT_ROUND_FIXTURES.iter().enumerate()
        {
            let parsed_summary = if !round_summary.starts_with('-') {
                format!(
                    "- P1: 一萬,二萬,三萬,四萬,五萬,六萬,七萬,八萬,九萬,一筒,二筒,三筒,四筒
                     - P2: 一萬,二萬,三萬,四萬,五萬,六萬,七萬,八萬,九萬,一筒,二筒,三筒,四筒
                     - P3: 一萬,二萬,三萬,四萬,五萬,六萬,七萬,八萬,九萬,一筒,二筒,三筒,四筒
                     - P4: 一萬,二萬,三萬,四萬,五萬,六萬,七萬,八萬,九萬,一筒,二筒,三筒,四筒
                     {round_summary}",
                )
            } else {
                round_summary.to_string()
            };
            let mut game = Game::from_summary(&parsed_summary);
            let expected_game = Game::from_summary(expected_round_summary);

            game.round.next_turn(&game.table.hands).unwrap_or_default();

            compare_rounds(&game.round, &expected_game.round, test_index);
        }
    }

    #[test]
    fn test_round_next_turn_errors() {
        let valid_hand = "一萬,二萬,三萬,四萬,五萬,六萬,七萬,八萬,九萬,一筒,二筒,三筒,四筒";
        for (test_index, error) in NextTurnError::iter().enumerate() {
            let (summary, parsed_err) = match error.clone() {
                NextTurnError::StuckWallTileNotDrawn => (
                    format!(
                        "- P1: {valid_hand}
                         - P2: {valid_hand}
                         - P3: {valid_hand}
                         - P4: {valid_hand}
                         Turn: P3, Dealer: P3, Round: 5"
                    ),
                    error,
                ),
                NextTurnError::StuckHandNotReady(_) => (
                    format!(
                        "- P1: {valid_hand}
                         - P2: {valid_hand}
                         - P3: {valid_hand},一萬
                         - P4: {valid_hand}
                         Turn: P3, Dealer: P3, Round: 5
                         Drawn: 一萬"
                    ),
                    NextTurnError::StuckHandNotReady("2".to_string()),
                ),
            };
            let mut game = Game::from_summary(&summary);

            let result = game.round.next_turn(&game.table.hands);

            assert_eq!(parsed_err, result.unwrap_err(), "Test index: {test_index}",);
        }
    }

    const MOVE_WIN_FIXTURES: &[(&str, &str, usize)] = &[
        (
            "Turn: P3, Dealer: P3, Round: 5, Wind: 西, Phase: Playing
             Consecutive: 2, Drawn: 一萬",
            "Turn: P4, Dealer: P4, Round: 6, Wind: 西, Phase: Playing",
            0,
        ),
        (
            "Turn: P1, Dealer: P1, Round: 5, Wind: 南, Phase: Playing
             Consecutive: 0, Drawn: 一萬, First East: P2",
            "Turn: P2, Dealer: P2, Round: 6, Wind: 西, Phase: Playing",
            1,
        ),
        (
            "Turn: P2, Dealer: P4, Round: 1, Wind: 北, Phase: Playing
             Consecutive: 0, Drawn: 一萬",
            "Turn: P2, Dealer: P1, Round: 2, Wind: 北, Phase: End",
            1,
        ),
    ];

    #[test]
    fn test_move_round_after_win() {
        for (test_index, (round_summary, expected_round_summary, winner_player_index)) in
            MOVE_WIN_FIXTURES.iter().enumerate()
        {
            let mut round = Round::from_summary(round_summary);
            let mut initial_phase = GamePhase::Playing;
            round.move_after_win(&mut initial_phase, *winner_player_index);

            let expected_game = Game::from_summary(expected_round_summary);

            compare_rounds(&round, &expected_game.round, test_index);
            assert_eq!(
                initial_phase, expected_game.phase,
                "test_index: {test_index}",
            );
        }
    }

    #[test]
    fn test_initial_winds() {
        let mut round = Round::new(&GameStyle::HongKong);
        assert_eq!(round.initial_winds, None);

        let winds: [Wind; 4] = [Wind::North, Wind::West, Wind::South, Wind::East];
        round.set_initial_winds(Some(winds.clone())).unwrap();
        assert_eq!(round.initial_winds, Some(23));
        assert_eq!(
            round.get_initial_winds_slice(),
            DecideDealerWinds::new(Some(winds)).unwrap()
        );

        let winds_2 = [Wind::South, Wind::North, Wind::East, Wind::West];
        round.set_initial_winds(Some(winds_2.clone())).unwrap();
        assert_eq!(round.initial_winds, Some(10));
        assert_eq!(
            round.get_initial_winds_slice(),
            DecideDealerWinds::new(Some(winds_2)).unwrap()
        );

        let winds_3 = WINDS_ROUND_ORDER.clone();
        round.set_initial_winds(Some(winds_3.clone())).unwrap();
        assert_eq!(round.initial_winds, Some(0));
        assert_eq!(
            round.get_initial_winds_slice(),
            DecideDealerWinds::new(Some(winds_3)).unwrap()
        );
    }
}
