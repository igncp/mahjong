// http://mahjongtime.com/hong-kong-mahjong-scoring.html
// https://en.wikipedia.org/wiki/Hong_Kong_mahjong_scoring_rules

use crate::{
    deck::DEFAULT_DECK, meld::MeldType, Flower, Game, PlayerId, Season, Tile, FLOWERS_ORDER,
    SEASONS_ORDER, WINDS_ROUND_ORDER,
};
use rustc_hash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use ts_rs::TS;

pub type ScoreItem = u32;
pub type ScoreMap = FxHashMap<PlayerId, ScoreItem>;

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
pub struct Score(pub ScoreMap);

// Proxied
impl Score {
    pub fn get(&self, player_id: &PlayerId) -> Option<&ScoreItem> {
        self.0.get(player_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&PlayerId, &ScoreItem)> {
        self.0.iter()
    }
}

// Proxied
impl Score {
    pub fn insert(&mut self, player_id: impl AsRef<str>, score: ScoreItem) {
        self.0.insert(player_id.as_ref().to_string(), score);
    }

    pub fn remove(&mut self, player_id: &PlayerId) -> ScoreItem {
        self.0.remove(player_id).unwrap()
    }
}

impl Score {
    pub fn new(players: &Vec<PlayerId>) -> Self {
        let mut score = ScoreMap::default();

        for player_id in players {
            score.insert(player_id.clone(), 0);
        }

        Self(score)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter)]
pub enum ScoringRule {
    AllFlowers,
    AllInTriplets,
    AllSeasons,
    BasePoint, // This is a custom rule until all other rules are implemented
    CommonHand,
    GreatDragons,
    LastWallTile,
    NoFlowersSeasons,
    SeatFlower,
    SeatSeason,
    SelfDraw,
    Purity,
}

impl Game {
    fn get_scoring_rules_points(scoring_rules: &Vec<ScoringRule>) -> u32 {
        enum ScoringRulePoints {
            Addition(u32),
            Multiplier(u32),
        }
        let mut round_points = 0;
        let mut points: Vec<ScoringRulePoints> = vec![];

        for rule in scoring_rules {
            points.push(match rule {
                ScoringRule::AllFlowers => ScoringRulePoints::Addition(2),
                ScoringRule::AllInTriplets => ScoringRulePoints::Addition(3),
                ScoringRule::AllSeasons => ScoringRulePoints::Addition(2),
                ScoringRule::BasePoint => ScoringRulePoints::Addition(1),
                ScoringRule::CommonHand => ScoringRulePoints::Addition(1),
                ScoringRule::GreatDragons => ScoringRulePoints::Addition(8),
                ScoringRule::LastWallTile => ScoringRulePoints::Addition(1),
                ScoringRule::NoFlowersSeasons => ScoringRulePoints::Addition(1),
                ScoringRule::SeatFlower => ScoringRulePoints::Addition(1),
                ScoringRule::SeatSeason => ScoringRulePoints::Addition(1),
                ScoringRule::SelfDraw => ScoringRulePoints::Addition(1),
                ScoringRule::Purity => ScoringRulePoints::Multiplier(6),
            });
        }

        round_points += points
            .iter()
            .filter_map(|point| {
                if let ScoringRulePoints::Addition(value) = point {
                    Some(*value)
                } else {
                    None
                }
            })
            .sum::<u32>();
        let multiplier = points
            .iter()
            .filter_map(|point| {
                if let ScoringRulePoints::Multiplier(value) = point {
                    Some(*value)
                } else {
                    None
                }
            })
            .product::<u32>();
        round_points *= multiplier.max(1);

        round_points
    }

    fn get_scoring_rules(&self, winner_player: &PlayerId) -> Vec<ScoringRule> {
        let mut rules = Vec::new();
        rules.push(ScoringRule::BasePoint);
        let empty_bonus = vec![];
        let winner_hand = self.table.hands.0.get(winner_player).unwrap();
        let winner_melds = winner_hand.get_melds();
        let melds_without_pair = winner_melds
            .melds
            .iter()
            .filter(|meld| meld.meld_type != MeldType::Pair)
            .collect::<Vec<_>>();

        let winner_bonus = self
            .table
            .bonus_tiles
            .0
            .get(winner_player)
            .unwrap_or(&empty_bonus);

        if winner_melds.melds.iter().all(|meld| {
            let tile = &DEFAULT_DECK.0[meld.tiles[0]];

            matches!(tile, Tile::Suit(_))
        }) {
            rules.push(ScoringRule::Purity);
        }

        if melds_without_pair
            .iter()
            .all(|meld| meld.meld_type == MeldType::Chow)
        {
            rules.push(ScoringRule::CommonHand);
        }

        if melds_without_pair
            .iter()
            .all(|meld| meld.meld_type == MeldType::Pung || meld.meld_type == MeldType::Kong)
        {
            rules.push(ScoringRule::AllInTriplets);
        }

        if melds_without_pair
            .iter()
            .filter(|meld| {
                if meld.meld_type == MeldType::Chow {
                    return false;
                }

                let tile = &DEFAULT_DECK.0[meld.tiles[0]];

                matches!(tile, Tile::Dragon(_))
            })
            .count()
            == 3
        {
            rules.push(ScoringRule::GreatDragons);
        }

        if self.table.draw_wall.is_empty() {
            rules.push(ScoringRule::LastWallTile);
        }

        if self.round.tile_claimed.is_none() {
            rules.push(ScoringRule::SelfDraw);
        }

        let mut flowers: FxHashSet<Flower> = FxHashSet::default();
        let mut seasons: FxHashSet<Season> = FxHashSet::default();

        for tile_id in winner_bonus {
            let tile = &DEFAULT_DECK.0[*tile_id];
            match tile {
                Tile::Flower(flower) => {
                    flowers.insert(flower.value.clone());
                }
                Tile::Season(season) => {
                    seasons.insert(season.value.clone());
                }
                _ => {}
            }
        }

        if flowers.is_empty() && seasons.is_empty() {
            rules.push(ScoringRule::NoFlowersSeasons);
        } else {
            if flowers.len() == 4 {
                rules.push(ScoringRule::AllFlowers);
            }

            if seasons.len() == 4 {
                rules.push(ScoringRule::AllSeasons);
            }

            let player_wind = self.round.get_player_wind(&self.players.0, winner_player);
            let has_seat_flower = flowers.iter().any(|flower| {
                let flower_index = FLOWERS_ORDER.iter().position(|f| f == flower).unwrap();
                WINDS_ROUND_ORDER[flower_index] == player_wind
            });
            let has_seat_season = seasons.iter().any(|season| {
                let season_index = SEASONS_ORDER.iter().position(|s| s == season).unwrap();
                WINDS_ROUND_ORDER[season_index] == player_wind
            });

            if has_seat_flower {
                rules.push(ScoringRule::SeatFlower);
            }

            if has_seat_season {
                rules.push(ScoringRule::SeatSeason);
            }
        }

        rules
    }
}

impl Game {
    pub fn calculate_hand_score(&mut self, winner_player: &PlayerId) -> (Vec<ScoringRule>, u32) {
        {
            let score = &mut self.score;
            let current_player_score = score.get(winner_player);
            if current_player_score.is_none() {
                return (vec![], 0);
            }
        }

        let scoring_rules = self.get_scoring_rules(winner_player);
        let round_points = Self::get_scoring_rules_points(&scoring_rules);

        let current_player_score = self.score.get(winner_player).unwrap();

        self.score
            .insert(winner_player, current_player_score + round_points);

        (scoring_rules, round_points)
    }
}
