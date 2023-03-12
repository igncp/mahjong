use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod deck;
mod test_deck;

type PlayerId = String;
type TileId = u32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub id: PlayerId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GamePhase {
    Beginning,
    End,
    Playing,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Wind {
    East,
    North,
    South,
    West,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WindTile {
    id: TileId,
    value: Wind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Tile {
    Dragon(DragonTile),
    Suit(SuitTile),
    Wind(WindTile),
    Flower(FlowerTile),
    Season(SeasonTile),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HandTile {
    concealed: bool,
    id: TileId,
    set_id: Option<String>,
}

pub type Hand = Vec<HandTile>;
pub type Hands = HashMap<PlayerId, Hand>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    board: Vec<TileId>,
    draw_wall: Vec<TileId>,
    hands: Hands,
}

pub type Score = HashMap<PlayerId, u32>;
pub type Deck = HashMap<TileId, Tile>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    pub deck: Deck,
    pub id: String,
    pub name: String,
    pub phase: GamePhase,
    pub players: Vec<Player>,
    pub score: Score,
    pub table: Table,
}
