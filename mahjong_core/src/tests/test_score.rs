#[cfg(test)]
mod test {
    use crate::{score::ScoringRule, Game};
    use strum::IntoEnumIterator;

    #[test]
    fn test_all_rules_has() {
        for score_rule in ScoringRule::iter() {
            let base_hand =
                "_ 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬, 一筒,二筒,三筒 四筒,四筒";
            let game_summary = match score_rule {
                ScoringRule::AllFlowers => {
                    format!("- P1: {base_hand} 竹,菊,蘭,梅")
                }
                ScoringRule::AllSeasons => format!("- P1: {base_hand} 冬,春,秋,夏"),
                ScoringRule::AllInTriplets => {
                    "- P1: _ 一萬,一萬,一萬 三筒,三筒,三筒 四筒,四筒,四筒 四萬,四萬,四萬 四筒,四筒"
                        .to_string()
                }
                ScoringRule::GreatDragons => {
                    "- P1: _ 白,白,白 發,發,發 中,中,中 一萬,二萬,三萬 四筒,四筒".to_string()
                }
                ScoringRule::BasePoint => format!("- P1: {base_hand} 四筒,四筒"),
                ScoringRule::LastWallTile => format!("- P1: {base_hand}"),
                ScoringRule::NoFlowersSeasons => format!("- P1: {base_hand}"),
                ScoringRule::SeatFlower => format!("- P1: {base_hand} 竹,菊,蘭,梅"),
                ScoringRule::SeatSeason => format!("- P1: {base_hand} 冬,春,秋,夏"),
                ScoringRule::SelfDraw => String::new(),
                ScoringRule::CommonHand => format!("- P1: {base_hand}"),
            };

            if game_summary.is_empty() {
                continue;
            }

            let mut game = Game::from_summary(&game_summary);
            game.score.insert("0", 0);

            let (scoring_rules, _) = game.calculate_hand_score(&"0".to_string());

            assert!(scoring_rules.contains(&score_rule));
        }
    }

    #[test]
    fn test_all_rules_has_not() {
        for score_rule in ScoringRule::iter() {
            let base_hand =
                "_ 一萬,二萬,三萬 四萬,五萬,六萬 七萬,八萬,九萬, 一筒,二筒,三筒 四筒,四筒";
            let game_summary = match score_rule {
                ScoringRule::CommonHand => {
                    "- P1: _ 一萬,一萬,一萬 四萬,五萬,六萬 七萬,八萬,九萬 一筒,二筒,三筒 四筒,四筒"
                        .to_string()
                }
                ScoringRule::GreatDragons => {
                    format!("- P1: {base_hand}")
                }
                ScoringRule::AllInTriplets => {
                    format!("- P1: {base_hand}")
                }
                ScoringRule::AllFlowers => {
                    format!("- P1: {base_hand} 竹,菊,蘭")
                }
                ScoringRule::AllSeasons => format!("- P1: {base_hand} 冬,春,秋"),
                ScoringRule::BasePoint => String::new(),
                ScoringRule::LastWallTile => format!(
                    "- P1: {base_hand}
                    Wall: 一萬"
                ),
                ScoringRule::NoFlowersSeasons => format!("- P1: {base_hand} 春"),
                ScoringRule::SeatFlower => format!("- P1: {base_hand} 竹"),
                ScoringRule::SeatSeason => format!("- P1: {base_hand} 冬"),
                ScoringRule::SelfDraw => String::new(),
            };

            if game_summary.is_empty() {
                continue;
            }

            let mut game = Game::from_summary(&game_summary);
            game.score.insert("0", 0);

            let (scoring_rules, _) = game.calculate_hand_score(&"0".to_string());

            assert!(!scoring_rules.contains(&score_rule), "Rule: {}", score_rule);
        }
    }
}
