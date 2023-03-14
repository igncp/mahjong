use std::collections::HashMap;

pub use game::{Game, GamePhase};
use serde::{Deserialize, Serialize};
pub use tile::{Tile, TileId};

pub mod deck;
pub mod game;
pub mod meld;
pub mod round;
mod test_deck;
mod test_meld;
mod test_round;
pub mod tile;

type PlayerId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub id: PlayerId,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum Suit {
    Bamboo,
    Characters,
    Dots,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuitTile {
    id: TileId,
    value: u32,
    suit: Suit,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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
    id: TileId,
    value: Wind,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Dragon {
    Green,
    Red,
    White,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DragonTile {
    id: TileId,
    value: Dragon,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Flower {
    Bamboo,
    Chrysanthemum,
    Orchid,
    Plum,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlowerTile {
    id: TileId,
    value: Flower,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Season {
    Autumn,
    Spring,
    Summer,
    Winter,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeasonTile {
    id: TileId,
    value: Season,
}

pub type SetId = Option<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HandTile {
    concealed: bool,
    id: TileId,
    set_id: SetId,
}

pub type Hand = Vec<HandTile>;
pub type Board = Vec<TileId>;
pub type Hands = HashMap<PlayerId, Hand>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    board: Board,
    draw_wall: Vec<TileId>,
    hands: Hands,
}

pub type Score = HashMap<PlayerId, u32>;
pub type Deck = HashMap<TileId, Tile>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RoundTileClaimed {
    by: Option<PlayerId>,
    from: PlayerId,
    id: TileId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Round {
    dealer_player_index: usize,
    player_index: usize,
    tile_claimed: Option<RoundTileClaimed>,
    wall_tile_drawn: Option<TileId>,
    wind: Wind,
}
