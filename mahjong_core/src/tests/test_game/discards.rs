#[cfg(test)]
mod test {
    use crate::tests::utils::read_fixture;
    use crate::{
        game::{DiscardTileError, GameCommand},
        CommandDiscardTile, Game, Tile,
    };
    use pretty_assertions::assert_eq;
    use strum::IntoEnumIterator;

    fn read_discard_fixture(name: &str) -> (String, String, String) {
        let (input, output) = read_fixture(name);
        let (action, summary) = input.trim().split_once('\n').unwrap();
        let tile = action
            .strip_prefix("Command: CommandDiscardTile ")
            .unwrap()
            .to_string();

        (tile, summary.to_string(), output.trim().to_string())
    }

    #[test]
    fn test_discard_valid_tile_to_board() {
        let (tile_summary, summary, expected) = read_discard_fixture("discard_valid_01");
        let mut game = Game::from_summary(&summary);

        let discarded_tile = CommandDiscardTile {
            tile_id: Tile::id_from_summary(&tile_summary),
        }
        .run(&mut game);

        assert_eq!(game.get_summary(), expected);
        assert!(discarded_tile.is_ok());
    }

    #[test]
    fn test_discard_errors() {
        for error in DiscardTileError::iter() {
            let (tile_summary, summary, expected) =
                read_discard_fixture(&format!("discard_{error:?}_01"));
            let mut game = Game::from_summary(&summary);
            let tile_to_discard = Tile::id_from_summary(&tile_summary);

            let result = CommandDiscardTile {
                tile_id: tile_to_discard,
            }
            .run(&mut game);

            assert_eq!(result, Err(error.clone()), "Test case: {:?}", error);
            assert_eq!(
                expected,
                format!("Error: {error:?}"),
                "Test case: {:?}",
                error
            );
        }
    }
}
