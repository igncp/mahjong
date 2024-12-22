use crate::{DragonTile, FlowerTile, SeasonTile, SuitTile, WindTile};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub type TileId = usize;

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
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
            Self::Suit(tile) => tile.id,
            Self::Dragon(tile) => tile.id,
            Self::Wind(tile) => tile.id,
            Self::Season(tile) => tile.id,
            Self::Flower(tile) => tile.id,
        }
    }

    pub fn is_same_type(&self, tile_b: &Self) -> bool {
        match self {
            Self::Suit(_) => matches!(tile_b, Self::Suit(_)),
            Self::Dragon(_) => matches!(tile_b, Self::Dragon(_)),
            Self::Wind(_) => matches!(tile_b, Self::Wind(_)),
            Self::Season(_) => matches!(tile_b, Self::Season(_)),
            Self::Flower(_) => matches!(tile_b, Self::Flower(_)),
        }
    }

    pub fn is_same_content(&self, tile_b: &Self) -> bool {
        match self {
            Self::Suit(tile_a) => match tile_b {
                Self::Suit(tile_b) => tile_a.suit == tile_b.suit && tile_a.value == tile_b.value,
                _ => false,
            },
            Self::Dragon(tile_a) => match tile_b {
                Self::Dragon(tile_b) => tile_a.value == tile_b.value,
                _ => false,
            },
            Self::Wind(tile_a) => match tile_b {
                Self::Wind(tile_b) => tile_a.value == tile_b.value,
                _ => false,
            },
            Self::Season(tile_a) => match tile_b {
                Self::Season(tile_b) => tile_a.value == tile_b.value,
                _ => false,
            },
            Self::Flower(tile_a) => match tile_b {
                Self::Flower(tile_b) => tile_a.value == tile_b.value,
                _ => false,
            },
        }
    }

    fn cmp_custom_order(tile: &Self) -> u32 {
        match tile {
            Self::Suit(_) => 0,
            Self::Dragon(_) => 1,
            Self::Wind(_) => 2,
            Self::Season(_) => 3,
            Self::Flower(_) => 4,
        }
    }

    pub fn cmp_custom(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Self::Suit(tile_a) => {
                if let Self::Suit(tile_b) = other {
                    if tile_a.suit != tile_b.suit {
                        return tile_a.suit.cmp(&tile_b.suit);
                    }

                    return tile_a.value.cmp(&tile_b.value);
                }
            }
            Self::Dragon(tile_a) => {
                if let Self::Dragon(tile_b) = other {
                    return tile_a.value.cmp(&tile_b.value);
                }
            }
            Self::Wind(tile_a) => {
                if let Self::Wind(tile_b) = other {
                    return tile_a.value.cmp(&tile_b.value);
                }
            }
            Self::Season(tile_a) => {
                if let Self::Season(tile_b) = other {
                    return tile_a.value.cmp(&tile_b.value);
                }
            }
            Self::Flower(tile_a) => {
                if let Self::Flower(tile_b) = other {
                    return tile_a.value.cmp(&tile_b.value);
                }
            }
        };

        Self::cmp_custom_order(self).cmp(&Self::cmp_custom_order(other))
    }

    pub fn is_bonus(&self) -> bool {
        matches!(self, Self::Flower(_) | Self::Season(_))
    }
}

impl Tile {
    pub fn set_id(&mut self, id: TileId) {
        match self {
            Self::Suit(tile) => tile.id = id,
            Self::Dragon(tile) => tile.id = id,
            Self::Wind(tile) => tile.id = id,
            Self::Season(tile) => tile.id = id,
            Self::Flower(tile) => tile.id = id,
        }
    }
}
