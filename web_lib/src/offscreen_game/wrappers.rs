use mahjong_core::{score::ScoringRule, Wind};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_name = Wind)]
pub enum WindWasm {
    East,
    North,
    South,
    West,
}

impl From<Wind> for WindWasm {
    fn from(wind: Wind) -> Self {
        match wind {
            Wind::East => Self::East,
            Wind::South => Self::South,
            Wind::West => Self::West,
            Wind::North => Self::North,
        }
    }
}

impl From<WindWasm> for Wind {
    fn from(wind: WindWasm) -> Self {
        match wind {
            WindWasm::East => Self::East,
            WindWasm::South => Self::South,
            WindWasm::West => Self::West,
            WindWasm::North => Self::North,
        }
    }
}

#[wasm_bindgen(js_name = ScoringRule)]
pub enum ScoringRuleWasm {
    AllFlowers,
    AllInTriplets,
    AllSeasons,
    BasePoint,
    CommonHand,
    GreatDragons,
    LastWallTile,
    NoFlowersSeasons,
    SeatFlower,
    SeatSeason,
    SelfDraw,
}

impl From<ScoringRule> for ScoringRuleWasm {
    fn from(rule: ScoringRule) -> Self {
        match rule {
            ScoringRule::AllFlowers => Self::AllFlowers,
            ScoringRule::AllInTriplets => Self::AllInTriplets,
            ScoringRule::AllSeasons => Self::AllSeasons,
            ScoringRule::BasePoint => Self::BasePoint,
            ScoringRule::CommonHand => Self::CommonHand,
            ScoringRule::GreatDragons => Self::GreatDragons,
            ScoringRule::LastWallTile => Self::LastWallTile,
            ScoringRule::NoFlowersSeasons => Self::NoFlowersSeasons,
            ScoringRule::SeatFlower => Self::SeatFlower,
            ScoringRule::SeatSeason => Self::SeatSeason,
            ScoringRule::SelfDraw => Self::SelfDraw,
        }
    }
}

impl From<ScoringRuleWasm> for ScoringRule {
    fn from(rule: ScoringRuleWasm) -> Self {
        match rule {
            ScoringRuleWasm::AllFlowers => Self::AllFlowers,
            ScoringRuleWasm::AllInTriplets => Self::AllInTriplets,
            ScoringRuleWasm::AllSeasons => Self::AllSeasons,
            ScoringRuleWasm::BasePoint => Self::BasePoint,
            ScoringRuleWasm::CommonHand => Self::CommonHand,
            ScoringRuleWasm::GreatDragons => Self::GreatDragons,
            ScoringRuleWasm::LastWallTile => Self::LastWallTile,
            ScoringRuleWasm::NoFlowersSeasons => Self::NoFlowersSeasons,
            ScoringRuleWasm::SeatFlower => Self::SeatFlower,
            ScoringRuleWasm::SeatSeason => Self::SeatSeason,
            ScoringRuleWasm::SelfDraw => Self::SelfDraw,
        }
    }
}
