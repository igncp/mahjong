use std::collections::HashMap;

pub use deck::Deck;
pub use game::{Game, GamePhase};
use serde::{Deserialize, Serialize};
pub use tile::{Tile, TileId};

pub mod deck;
pub mod game;
pub mod meld;
pub mod round;
pub mod score;
mod test_deck;
mod test_game;
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HandTile {
    concealed: bool,
    id: TileId,
    set_id: SetId,
}

impl HandTile {
    pub fn from_id(id: TileId) -> Self {
        Self {
            id,
            set_id: None,
            concealed: true,
        }
    }
}

pub type Hand = Vec<HandTile>;
pub type Board = Vec<TileId>;
pub type Hands = HashMap<PlayerId, Hand>;
pub type DrawWall = Vec<TileId>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    board: Board,
    draw_wall: DrawWall,
    hands: Hands,
}

pub type Score = HashMap<PlayerId, u32>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RoundTileClaimed {
    by: Option<PlayerId>,
    from: PlayerId,
    id: TileId,
}

pub type TileClaimed = Option<RoundTileClaimed>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Round {
    dealer_player_index: usize,
    player_index: usize,
    tile_claimed: TileClaimed,
    wall_tile_drawn: Option<TileId>,
    wind: Wind,
}
