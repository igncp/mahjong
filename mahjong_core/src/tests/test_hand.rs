#[cfg(test)]
mod test {
    use crate::{
        hand::{CanSayMahjongError, SortHandError},
        Hand, Tile,
    };
    use pretty_assertions::assert_eq;
    use strum::IntoEnumIterator;

    #[test]
    fn test_sort_by_tiles() {
        for error in SortHandError::iter() {
            let (summary, tiles) = match error {
                SortHandError::NotSortedMissingTile => ("一萬,二萬,三萬", "一筒,二筒"),
            };

            let result = Hand::from_summary(summary).sort_by_tiles(&Tile::ids_from_summary(tiles));

            assert_eq!(error, result.unwrap_err());
        }

        let mut hand = Hand::from_summary("二萬,四萬,三萬 一筒,三筒,二筒");

        // It sorts with the new order
        hand.sort_by_tiles(&Tile::ids_from_summary("四萬,三萬,二萬"))
            .unwrap();

        let result_ids = hand.list.iter().map(|t| t.id).collect::<Vec<_>>();

        assert_eq!(
            result_ids,
            Tile::ids_from_summary("四萬,三萬,二萬,一筒,三筒,二筒"),
        );
    }

    #[test]
    fn test_can_say_mahjong() {
        for error in CanSayMahjongError::iter() {
            let summary = match error {
                CanSayMahjongError::CantDrop => "",
                CanSayMahjongError::NotPair => {
                    "一萬,二萬 一筒,一筒,一筒 二筒,二筒,二筒 三筒,三筒,三筒 四筒,四筒,四筒"
                }
            };

            let result = Hand::from_summary(summary).can_say_mahjong();

            assert_eq!(error, result.unwrap_err());
        }

        let correct_result = Hand::from_summary(
            "一萬,一萬 一筒,一筒,一筒 二筒,二筒,二筒 三筒,三筒,三筒 四筒,四筒,四筒",
        )
        .can_say_mahjong();

        assert_eq!(correct_result, Ok(()));
    }
}
