#[cfg(test)]
mod test {
    use crate::hand::HandPossibleMeld;
    use crate::meld::{get_is_chow, get_is_kong, get_is_pung, PlayerDiff, SetCheckOpts};
    use crate::{Hand, Tile};
    use pretty_assertions::assert_eq;

    const PUNG_FIXTURES: &[(&str, bool)] = &[
        ("一筒,一筒,一筒", true),
        ("一筒,二筒,一筒", false),
        ("北,北,北", true),
        ("竹,竹,竹", false),
        ("北,北,七筒", false),
    ];

    const CHOW_FIXTURES: &[(&str, bool, Option<&'static str>, PlayerDiff)] = &[
        ("一筒,三筒,二筒", true, None, None),
        ("一筒,三筒,二筒", true, Some("三筒"), Some(1)),
        ("一筒,三筒,二筒", false, Some("三筒"), Some(-1)),
        ("一筒,一筒,一筒", false, Some("三筒"), None),
    ];

    const KONG_FIXTURES: &[(&str, bool)] = &[
        ("西,西,七筒,七筒", false),
        ("西,西,西,西", true),
        ("一筒,二筒,三筒,四筒", false),
    ];

    const POSSIBLE_MELDS_FIXTURES: &[(&str, PlayerDiff, &[&str])] = &[
        ("二筒,一筒,三筒", Some(0), &["一筒,三筒,二筒 NO"]),
        ("一筒,二筒,四筒", None, &[]),
        (
            "三筒,一筒,二筒,五筒,五筒,五筒",
            None,
            &["一筒,三筒,二筒 NO", "五筒,五筒,五筒 NO"],
        ),
    ];

    enum MeldFixtureArray {
        Small(&'static [(&'static str, bool)]),
        Full(&'static [(&'static str, bool, Option<&'static str>, PlayerDiff)]),
    }

    fn test_meld(arr: MeldFixtureArray, func: impl Fn(&SetCheckOpts) -> bool) {
        let parsed_arr = match arr {
            MeldFixtureArray::Small(arr) => &arr
                .iter()
                .map(|(a, b)| (*a, *b, None, None))
                .collect::<Vec<_>>(),
            MeldFixtureArray::Full(arr) => &arr.to_vec(),
        };

        for (index, (tiles, expected_result, claimed_tile, board_tile_player_diff)) in
            parsed_arr.iter().enumerate()
        {
            let sub_hand = Hand::from_summary(tiles);
            let opts = SetCheckOpts {
                board_tile_player_diff: *board_tile_player_diff,
                claimed_tile: claimed_tile.map(Tile::id_from_summary),
                sub_hand: &sub_hand.into(),
            };

            assert_eq!(func(&opts), *expected_result, "index: {index}");
        }
    }

    #[test]
    fn test_is_pung() {
        test_meld(MeldFixtureArray::Small(PUNG_FIXTURES), get_is_pung);
    }

    #[test]
    fn test_is_chow() {
        test_meld(MeldFixtureArray::Full(CHOW_FIXTURES), get_is_chow);
    }

    #[test]
    fn test_get_is_kong() {
        test_meld(MeldFixtureArray::Small(KONG_FIXTURES), get_is_kong);
    }

    #[test]
    fn test_get_possible_melds() {
        for (index, (hand_summary, player_diff, expected_meld_summary)) in
            POSSIBLE_MELDS_FIXTURES.iter().enumerate()
        {
            let hand = Hand::from_summary(hand_summary);
            let possible_melds = hand
                .get_possible_melds(*player_diff, None, false)
                .into_iter()
                .map(|m| {
                    let hand_possible_meld: HandPossibleMeld = m;
                    hand_possible_meld.to_summary()
                })
                .collect::<Vec<_>>();

            assert_eq!(possible_melds, *expected_meld_summary, "index: {index}");
        }
    }
}
