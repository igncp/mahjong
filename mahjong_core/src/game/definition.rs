use crate::{macros::derive_game_common, round::Round, Score, Table};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use super::Players;

derive_game_common! {
#[derive(PartialEq)]
pub enum GamePhase {
    Beginning,
    End,
    Playing,
}}

derive_game_common! {
#[derive(PartialEq, Eq)]
pub enum GameStyle {
    HongKong,
}}

pub type GameId = String;
pub type GameVersion = String;

derive_game_common! {
pub struct Game {
    pub id: GameId,
    pub name: String,
    pub phase: GamePhase,
    pub players: Players,
    pub round: Round,
    pub score: Score,
    pub table: Table,
    pub version: GameVersion,
    pub style: GameStyle,
}}

impl Game {
    pub fn get_players_num(style: &GameStyle) -> usize {
        match style {
            GameStyle::HongKong => 4,
        }
    }
}

impl Display for GamePhase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Beginning => write!(f, "Beginning"),
            Self::End => write!(f, "End"),
            Self::Playing => write!(f, "Playing"),
        }
    }
}

const STYLE_HONG_KONG: &str = "Hong Kong";

impl Display for GameStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HongKong => f.write_str(STYLE_HONG_KONG),
        }
    }
}

impl FromStr for GameStyle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            STYLE_HONG_KONG => Ok(Self::HongKong),
            _ => Err(()),
        }
    }
}

impl GameStyle {
    pub fn tiles_after_claim(&self) -> usize {
        match self {
            Self::HongKong => 14,
        }
    }
}
