#![deny(clippy::use_self, clippy::shadow_unrelated)]
pub use deck::Deck;
pub use game::{Game, GameId, GamePhase};
pub use hand::{Hand, HandTile, SetId};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
pub use tile::{Tile, TileId};

pub mod ai;
pub mod deck;
pub mod game;
pub mod game_summary;
pub mod hand;
mod log;
pub mod meld;
pub mod round;
pub mod score;
mod test_deck;
mod test_game;
mod test_game_summary;
mod test_hand;
mod test_meld;
mod test_round;
pub mod tile;
pub mod ui;

pub type PlayerId = String;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Bamboo,
    Characters,
    Dots,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuitTile {
    pub id: TileId,
    pub value: u32,
    pub suit: Suit,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Wind {
    East,
    North,
    South,
    West,
}

// Note that this order is reversed to the compass directions, since it is counter-clockwise
pub const WINDS_ROUND_ORDER: &[Wind] = &[Wind::East, Wind::South, Wind::West, Wind::North];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WindTile {
    pub id: TileId,
    pub value: Wind,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dragon {
    Green,
    Red,
    White,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DragonTile {
    pub id: TileId,
    pub value: Dragon,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Flower {
    Bamboo,
    Chrysanthemum,
    Orchid,
    Plum,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlowerTile {
    pub id: TileId,
    pub value: Flower,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Season {
    Autumn,
    Spring,
    Summer,
    Winter,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeasonTile {
    pub id: TileId,
    pub value: Season,
}

pub type Board = Vec<TileId>;
pub type Hands = FxHashMap<PlayerId, Hand>;
pub type DrawWall = Vec<TileId>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    pub board: Board,
    pub draw_wall: DrawWall,
    pub hands: Hands,
}

pub type ScoreItem = u32;
pub type Score = FxHashMap<PlayerId, ScoreItem>;
