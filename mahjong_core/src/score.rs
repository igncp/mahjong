// http://mahjongtime.com/hong-kong-mahjong-scoring.html

use std::collections::HashSet;

use crate::{deck::DEFAULT_DECK, Flower, Game, PlayerId, Season, Tile};

enum ScoringRule {
    AllFlowers,
    AllSeasons,
    BasePoint, // This is a custome rule until all other rules are implemented
    LastWallTile,
    NoFlowersSeasons,
    SelfDraw,
}

impl Game {
    pub fn calculate_hand_score(&mut self, winner_player: &PlayerId) {
        {
            let score = &mut self.score;
            let current_player_score = score.get(winner_player);
            if current_player_score.is_none() {
                return;
            }
        }

        let scoring_rules = self.get_scoring_rules();
        let round_points = Self::get_scoring_rules_points(&scoring_rules);

        let current_player_score = self.score.get(winner_player).unwrap();

        self.score
            .insert(winner_player.clone(), current_player_score + round_points);
    }

    fn get_scoring_rules_points(scoring_rules: &Vec<ScoringRule>) -> u32 {
        let mut round_points = 0;

        for rule in scoring_rules {
            round_points += match rule {
                ScoringRule::AllFlowers => 2,
                ScoringRule::AllSeasons => 2,
                ScoringRule::BasePoint => 1,
                ScoringRule::LastWallTile => 1,
                ScoringRule::NoFlowersSeasons => 1,
                ScoringRule::SelfDraw => 1,
            }
        }

        round_points
    }

    fn get_scoring_rules(&self) -> Vec<ScoringRule> {
        let mut rules = Vec::new();
        rules.push(ScoringRule::BasePoint);
        let winner_player = self
            .players
            .iter()
            .find(|p| self.table.hands.get(*p).unwrap().0.len() == 14)
            .unwrap();
        let winner_hand = self.table.hands.get(winner_player).unwrap().clone();

        if self.table.draw_wall.is_empty() {
            rules.push(ScoringRule::LastWallTile);
        }

        if self.round.tile_claimed.is_none() {
            rules.push(ScoringRule::SelfDraw);
        }

        let mut flowers: HashSet<Flower> = HashSet::new();
        let mut seasons: HashSet<Season> = HashSet::new();

        for tile in winner_hand.0 {
            let tile = DEFAULT_DECK.0.get(&tile.id).unwrap();

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
        }

        rules
    }
}
