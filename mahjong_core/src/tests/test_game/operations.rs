#[cfg(test)]
mod test {
    use crate::{
        game::{BreakMeldError, CreateMeldError},
        Game, Tile,
    };
    use pretty_assertions::assert_eq;
    use strum::IntoEnumIterator;

    #[test]
    fn test_break_meld() {
        for error in BreakMeldError::iter() {
            let (summary, player_id) = match error {
                BreakMeldError::MeldIsKong => (
                    "- P1: 一萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬 一索,二索,三索 二萬,二萬,二萬,二萬
                     Turn: P1",
                    "0",
                ),
                BreakMeldError::MissingHand => (
                    "- P1: 一萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬 一索,二索,三索 二萬,二萬,二萬
                     Turn: P1",
                    "missing",
                ),
                BreakMeldError::TileIsExposed => (
                    "- P1: 一萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬 一索,二索,三索 *二萬,二萬,二萬
                     Turn: P1",
                    "0",
                ),
            };

            let mut game = Game::from_summary(summary);
            let meld_id = game.get_meld_id_from_summary("0", "二萬");

            let result = game.break_meld(&player_id.to_string(), &meld_id);

            assert_eq!(result, Err(error.clone()), "Test case: {:?}", error);
        }

        let mut game_correct = Game::from_summary(
            "- P1: 一萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬 一索,二索,三索 二萬,二萬,二萬
             Turn: P1",
        );

        let meld_id = game_correct.get_meld_id_from_summary("0", "二萬");

        let result = game_correct.break_meld(&"0".to_string(), &meld_id);

        assert_eq!(result, Ok(()), "Test case correct");
    }

    #[test]
    fn test_create_meld() {
        for error in CreateMeldError::iter() {
            let (summary, tiles_summary) = match error {
                CreateMeldError::EndRound => ("", ""),
                CreateMeldError::NotMeld => (
                    "- P1: 一萬,三萬,四萬,五萬,六萬,七萬,八萬,九萬,一索,二索,三索,二萬,二萬,二萬
                     Turn: P1",
                    "九萬,一索,二索",
                ),
                CreateMeldError::TileIsPartOfMeld => (
                    "- P1: 一萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬 一索,二索,三索 二萬,二萬,二萬
                     Turn: P1",
                    "一索,二索,三索",
                ),
            };

            if summary.is_empty() {
                continue;
            }

            let mut game = Game::from_summary(summary);
            let tiles = Tile::ids_from_summary(tiles_summary);

            let result = game.create_meld(&"0".to_string(), &tiles, false, false);

            assert_eq!(result, Err(error.clone()), "Test case: {:?}", error);
        }

        let mut game_correct = Game::from_summary(
            "- P1: 一萬,三萬,四萬,五萬,六萬,七萬,八萬,九萬,一索,二索,三索,二萬,二萬,二萬
            Turn: P1",
        );

        let result = game_correct.create_meld(
            &"0".to_string(),
            &Tile::ids_from_summary("一索,二索,三索"),
            false,
            false,
        );

        assert_eq!(result, Ok(()), "Test case correct");
    }
}
