#[cfg(test)]
mod test {
    use crate::{score::ScoringRule, Game};

    fn test_contains(hand: &str, bonus_tiles: &str, expected: ScoringRule) {
        let mut game = Game::new(None);
        game.start_with_players();

        game.table.hands.update_players_hands(&[hand, "", "", ""]);
        game.table.bonus_tiles.set_from_summary("0", bonus_tiles);

        let (scoring_rules, _) = game.calculate_hand_score(&"0".to_string());

        assert!(scoring_rules.contains(&expected));
    }

    fn test_not_contains(hand: &str, expected: ScoringRule) {
        let mut game = Game::new(None);
        game.start_with_players();

        game.table.hands.update_players_hands(&[hand, "", "", ""]);

        let (scoring_rules, _) = game.calculate_hand_score(&"0".to_string());

        assert!(!scoring_rules.contains(&expected));
    }

    #[test]
    fn test_common_rules() {
        test_contains("", "竹,菊,蘭,梅", ScoringRule::AllFlowers);
        test_not_contains("菊,蘭,梅", ScoringRule::AllFlowers);
        test_contains("", "春,夏,秋,冬", ScoringRule::AllSeasons);
        test_not_contains("夏,秋,冬", ScoringRule::AllSeasons);
    }
}
