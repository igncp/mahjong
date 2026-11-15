#[cfg(test)]
mod test {
    use crate::deck::DEFAULT_DECK;
    use crate::hand::HandPossibleMeld;
    use crate::meld::{get_is_chow, get_is_kong, get_is_pung, PlayerDiff, SetCheckOpts};
    use crate::tests::utils::{fixtures_path, read_fixture};
    use crate::{Hand, Tile};
    use pretty_assertions::assert_eq;
    use std::fs;

    struct MeldFixture {
        tiles: String,
        claimed_tile: Option<String>,
        player_diff: PlayerDiff,
        expected: String,
    }

    fn read_meld_fixture(name: &str) -> MeldFixture {
        let (input, output) = read_fixture(name);
        let fields = input
            .trim()
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .collect::<Vec<_>>();
        let required = |field| {
            fields
                .iter()
                .find(|(field_name, _)| *field_name == field)
                .unwrap()
                .1
        };

        MeldFixture {
            tiles: required("Tiles").to_string(),
            claimed_tile: fields
                .iter()
                .find(|(field_name, _)| *field_name == "Claimed")
                .map(|(_, value)| value.to_string()),
            player_diff: fields
                .iter()
                .find(|(field_name, _)| *field_name == "PlayerDiff")
                .map(|(_, value)| value.parse().unwrap()),
            expected: output.trim().to_string(),
        }
    }

    fn meld_fixture_names(meld_type: &str) -> Vec<String> {
        let prefix = format!("meld_{meld_type}_");
        let mut fixtures = fs::read_dir(fixtures_path())
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().unwrap().is_dir())
            .filter_map(|entry| entry.file_name().into_string().ok())
            .filter(|name| name.starts_with(&prefix))
            .collect::<Vec<_>>();
        fixtures.sort();
        fixtures
    }

    fn check_meld(name: &str, check: impl Fn(&SetCheckOpts) -> bool) {
        let fixture = read_meld_fixture(name);
        let hand = Hand::from_summary(&fixture.tiles);
        let sub_hand = hand
            .list
            .iter()
            .map(|tile| &DEFAULT_DECK.0[tile.id])
            .collect::<Vec<&Tile>>();
        let opts = SetCheckOpts {
            board_tile_player_diff: fixture.player_diff,
            claimed_tile: fixture
                .claimed_tile
                .map(|tile| Tile::id_from_summary(&tile)),
            sub_hand: &sub_hand,
        };

        assert_eq!(
            check(&opts).to_string(),
            fixture.expected,
            "fixture: {name}"
        );
    }

    #[test]
    fn test_is_pung() {
        for name in meld_fixture_names("pung") {
            check_meld(&name, get_is_pung);
        }
    }

    #[test]
    fn test_is_chow() {
        for name in meld_fixture_names("chow") {
            check_meld(&name, get_is_chow);
        }
    }

    #[test]
    fn test_get_is_kong() {
        for name in meld_fixture_names("kong") {
            check_meld(&name, get_is_kong);
        }
    }

    #[test]
    fn test_get_possible_melds() {
        for name in meld_fixture_names("possible") {
            let fixture = read_meld_fixture(&name);
            let hand = Hand::from_summary(&fixture.tiles);
            let possible_melds = hand
                .get_possible_melds(fixture.player_diff, None, false)
                .into_iter()
                .map(|meld| {
                    let hand_possible_meld: HandPossibleMeld = meld;
                    hand_possible_meld.to_summary()
                })
                .collect::<Vec<_>>();
            let expected = match fixture.expected.as_str() {
                "None" => vec![],
                output => output.lines().collect(),
            };

            assert_eq!(possible_melds, expected, "fixture: {name}");
        }
    }
}
