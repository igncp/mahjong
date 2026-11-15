#[cfg(test)]
mod test {
    use crate::{
        game::{ClaimTileError, DrawError, DrawTileResult, GameCommand},
        tests::utils::{fixture_names, read_fixture},
        CommandClaimTile, CommandDrawTile, CommandInitialDraw, CommandPrepareTable, Game,
        GamePhase,
    };
    use pretty_assertions::assert_eq;
    use strum::IntoEnumIterator;

    fn parse_fixture(name: &str) -> (String, String, String) {
        let (input, output) = read_fixture(name);
        let (command, summary) = input.trim().split_once('\n').unwrap();

        (
            command.to_string(),
            summary.to_string(),
            output.trim().to_string(),
        )
    }

    #[test]
    fn test_prepare_table() {
        for name in fixture_names("command_prepare_table_") {
            let (command, summary, expected) = parse_fixture(&name);
            let mut game = Game::from_summary(&summary);

            assert_eq!(command, "Command: CommandPrepareTable", "fixture: {name}");
            CommandPrepareTable {
                with_dead_wall: false,
                shuffle_tiles: false,
            }
            .run(&mut game)
            .unwrap_or_else(|never| match never {});

            assert_eq!(game.phase, GamePhase::InitialDraw, "fixture: {name}");
            assert_eq!(expected, "Result: Prepared", "fixture: {name}");
        }
    }

    #[test]
    fn test_draw_tile() {
        for name in fixture_names("command_draw_tile_") {
            let (command, summary, expected) = parse_fixture(&name);
            let mut game = Game::from_summary(&summary);

            assert_eq!(command, "Command: CommandDrawTile", "fixture: {name}");
            let result = CommandDrawTile
                .run(&mut game)
                .unwrap_or_else(|never| match never {});
            let actual = render_draw_result(result);

            assert_eq!(actual, expected, "fixture: {name}");
        }
    }

    #[test]
    fn test_claim_tile() {
        let name = "command_claim_tile_01";
        let (command, summary, expected) = parse_fixture(name);
        let mut game = Game::from_summary(&summary);
        let player_id = parse_claim_player(&command);

        assert_eq!(
            CommandClaimTile {
                player_id: &player_id
            }
            .run(&mut game),
            Ok(())
        );
        assert_eq!(expected, "Result: Claimed");
    }

    #[test]
    fn test_claim_tile_errors() {
        for error in ClaimTileError::iter() {
            let name = format!("command_claim_tile_{error:?}_01");
            let (command, summary, expected) = parse_fixture(&name);
            let mut game = Game::from_summary(&summary);
            let player_id = parse_claim_player(&command);

            assert_eq!(
                CommandClaimTile {
                    player_id: &player_id
                }
                .run(&mut game),
                Err(error.clone()),
                "fixture: {name}"
            );
            assert_eq!(expected, format!("Error: {error:?}"), "fixture: {name}");
        }
    }

    fn parse_claim_player(command: &str) -> String {
        command
            .strip_prefix("Command: CommandClaimTile P")
            .unwrap()
            .parse::<usize>()
            .unwrap()
            .checked_sub(1)
            .unwrap()
            .to_string()
    }

    #[test]
    fn test_initial_draw_errors() {
        for error in DrawError::iter() {
            let name = format!("command_initial_draw_{error:?}_01");
            let (command, summary, expected) = parse_fixture(&name);
            let mut game = Game::from_summary(&summary);

            assert_eq!(command, "Command: CommandInitialDraw", "fixture: {name}");
            assert_eq!(
                CommandInitialDraw.run(&mut game),
                Err(error.clone()),
                "fixture: {name}"
            );
            assert_eq!(expected, format!("Error: {error:?}"), "fixture: {name}");
        }
    }

    fn render_draw_result(result: DrawTileResult) -> String {
        match result {
            DrawTileResult::AlreadyDrawn => "Result: AlreadyDrawn".to_string(),
            DrawTileResult::Bonus(tile) | DrawTileResult::Normal(tile) => {
                format!(
                    "Result: Normal {}",
                    crate::summary_view::print_game_tile(&crate::deck::DEFAULT_DECK.0[tile])
                )
            }
            DrawTileResult::WallExhausted => "Result: WallExhausted".to_string(),
        }
    }
}
