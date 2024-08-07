#![deny(clippy::use_self, clippy::shadow_unrelated)]
pub use deck::Deck;
pub use game::{Game, GameId, GamePhase, PlayerId, Players};
pub use hand::{Hand, HandTile, SetId};
pub use hand::{Hands, HandsMap};
pub use score::{Score, ScoreItem, ScoreMap};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
pub use table::{Board, BonusTiles, DrawWall, DrawWallPlace, Table};
pub use tile::{Tile, TileId};
use ts_rs::TS;

pub mod ai;
pub mod deck;
pub mod game;
pub mod game_summary;
pub mod hand;
mod log;
mod macros;
pub mod meld;
pub mod round;
pub mod score;
#[cfg(feature = "summary")]
mod summary_view;
mod table;
#[cfg(test)]
mod tests;
pub mod tile;
pub mod ui;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, TS)]
#[ts(export)]
pub enum Suit {
    Bamboo,
    Characters,
    Dots,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SuitTile {
    pub id: TileId,
    pub value: u32,
    pub suit: Suit,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, TS)]
#[ts(export)]
pub enum Wind {
    East,
    North,
    South,
    West,
}

// Note that this order is reversed to the compass directions, since it is counter-clockwise
pub const WINDS_ROUND_ORDER: &[Wind; 4] = &[Wind::East, Wind::South, Wind::West, Wind::North];
pub const FLOWERS_ORDER: &[Flower] = &[
    Flower::Plum,
    Flower::Orchid,
    Flower::Chrysanthemum,
    Flower::Bamboo,
];
pub const SEASONS_ORDER: &[Season] = &[
    Season::Spring,
    Season::Summer,
    Season::Autumn,
    Season::Winter,
];

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct WindTile {
    pub id: TileId,
    pub value: Wind,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, TS)]
#[ts(export)]
pub enum Dragon {
    Green,
    Red,
    White,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DragonTile {
    pub id: TileId,
    pub value: Dragon,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, TS)]
#[ts(export)]
pub enum Flower {
    Bamboo,
    Chrysanthemum,
    Orchid,
    Plum,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct FlowerTile {
    pub id: TileId,
    pub value: Flower,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, TS)]
#[ts(export)]
pub enum Season {
    Autumn,
    Spring,
    Summer,
    Winter,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SeasonTile {
    pub id: TileId,
    pub value: Season,
}

impl FromStr for Wind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "東" => Ok(Self::East),
            "北" => Ok(Self::North),
            "南" => Ok(Self::South),
            "西" => Ok(Self::West),
            _ => Err(()),
        }
    }
}

impl Display for Wind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let wind_str = match self {
            Self::East => "東",
            Self::North => "北",
            Self::South => "南",
            Self::West => "西",
        };
        write!(f, "{}", wind_str)
    }
}
