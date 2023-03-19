use serde::{Deserialize, Serialize};

use crate::{DragonTile, FlowerTile, SeasonTile, SuitTile, WindTile};

pub type TileId = u32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Tile {
    Dragon(DragonTile),
    Suit(SuitTile),
    Wind(WindTile),
    Flower(FlowerTile),
    Season(SeasonTile),
}

impl Tile {
    pub fn get_id(&self) -> TileId {
        match self {
            Tile::Suit(tile) => tile.id,
            Tile::Dragon(tile) => tile.id,
            Tile::Wind(tile) => tile.id,
            Tile::Season(tile) => tile.id,
            Tile::Flower(tile) => tile.id,
        }
    }

    pub fn set_id(&mut self, id: TileId) {
        match self {
            Tile::Suit(tile) => tile.id = id,
            Tile::Dragon(tile) => tile.id = id,
            Tile::Wind(tile) => tile.id = id,
            Tile::Season(tile) => tile.id = id,
            Tile::Flower(tile) => tile.id = id,
        }
    }

    pub fn is_same_content(&self, tile_b: &Self) -> bool {
        match self {
            Tile::Suit(tile_a) => match tile_b {
                Tile::Suit(tile_b) => tile_a.suit == tile_b.suit && tile_a.value == tile_b.value,
                _ => false,
            },
            Tile::Dragon(tile_a) => match tile_b {
                Tile::Dragon(tile_b) => tile_a.value == tile_b.value,
                _ => false,
            },
            Tile::Wind(tile_a) => match tile_b {
                Tile::Wind(tile_b) => tile_a.value == tile_b.value,
                _ => false,
            },
            Tile::Season(tile_a) => match tile_b {
                Tile::Season(tile_b) => tile_a.value == tile_b.value,
                _ => false,
            },
            Tile::Flower(tile_a) => match tile_b {
                Tile::Flower(tile_b) => tile_a.value == tile_b.value,
                _ => false,
            },
        }
    }

    pub fn cmp_custom(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Tile::Suit(tile_a) => match other {
                Tile::Suit(tile_b) => {
                    if tile_a.suit != tile_b.suit {
                        return tile_a.suit.cmp(&tile_b.suit);
                    }

                    return tile_a.value.cmp(&tile_b.value);
                }
                _ => std::cmp::Ordering::Less,
            },
            Tile::Dragon(tile_a) => match other {
                Tile::Dragon(tile_b) => tile_a.value.cmp(&tile_b.value),
                _ => std::cmp::Ordering::Greater,
            },
            Tile::Wind(tile_a) => match other {
                Tile::Wind(tile_b) => tile_a.value.cmp(&tile_b.value),
                _ => std::cmp::Ordering::Greater,
            },
            Tile::Season(tile_a) => match other {
                Tile::Season(tile_b) => tile_a.value.cmp(&tile_b.value),
                _ => std::cmp::Ordering::Greater,
            },
            Tile::Flower(tile_a) => match other {
                Tile::Flower(tile_b) => tile_a.value.cmp(&tile_b.value),
                _ => std::cmp::Ordering::Greater,
            },
        }
    }
}
