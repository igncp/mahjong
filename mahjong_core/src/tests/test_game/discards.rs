#[cfg(test)]
mod test {
    use crate::{game::DiscardTileError, Game, Tile};
    use pretty_assertions::assert_eq;
    use strum::IntoEnumIterator;

    #[test]
    fn test_discard_valid_tile_to_board() {
        let mut game = Game::from_summary(
            r#"
- P1: 一萬,二萬,三萬,四萬,五萬,六萬,七萬,八萬,九萬,一索,二索,三索,四索,五索
- P2: 八筒
- P3: 九筒
Board: 一筒,二筒,三筒
"#
            .trim(),
        );

        let discarded_tile = game.discard_tile_to_board(&Tile::id_from_summary("二萬"));

        assert_eq!(
            game.get_summary(),
            r#"
- P1: 一索,一萬,七萬,三索,三萬,九萬,二索,五索,五萬,八萬,六萬,四索,四萬
- P2: 八筒
- P3: 九筒
Board: 二萬,三筒...
Turn: P1, Dealer: P1, Round: 1, Wind: East, Phase: Beginning
Consecutive: 0, Discarded: 二萬
"#
            .trim()
        );
        assert!(discarded_tile.is_ok());
    }

    #[test]
    fn test_discard_errors() {
        for error in DiscardTileError::iter() {
            let (summary, tile_summary) = match error {
                DiscardTileError::TileIsPartOfMeld => (
                    "- P1: 一萬,三萬,四萬,五萬,六萬,七萬,八萬,九萬,一索,二索,三索 二萬,二萬,二萬
                     Turn: P1",
                    "二萬",
                ),
                DiscardTileError::TileIsExposed => (
                    "- P1: 一萬,三萬,四萬,五萬,六萬,七萬,八萬,九萬,一索,二索,三索 *二萬,二萬,二萬
                     Turn: P1",
                    "二萬",
                ),
                DiscardTileError::PlayerHasNoTile => (
                    "- P1: 一萬,三萬,四萬,五萬,六萬,七萬,八萬,九萬,一索,二索,三索,三索,三索,三索
                     Turn: P1",
                    "二萬",
                ),
                DiscardTileError::NoPlayerCanDiscard => ("Turn: P1", "三萬"),
                DiscardTileError::ClaimedAnotherTile => (
                    "- P1: 一萬,三萬,四萬,五萬,六萬,七萬,八萬,九萬,一索,二索,三索,二萬,二萬,二萬
                     Turn: P1
                     Discarded: 一萬(P1)",
                    "三萬",
                ),
            };

            let mut game = Game::from_summary(summary);
            let tile_to_discard = Tile::id_from_summary(tile_summary);

            let result = game.discard_tile_to_board(&tile_to_discard);

            assert_eq!(result, Err(error.clone()), "Test case: {:?}", error);
        }
    }
}
