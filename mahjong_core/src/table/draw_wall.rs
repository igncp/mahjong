use crate::{TileId, Wind, WINDS_ROUND_ORDER};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};
use ts_rs::TS;

pub enum DrawWallPlace {
    Segment(Wind),
    DeadWall,
    Unordered,
}

impl Display for DrawWallPlace {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Segment(wind) => write!(f, "Segment({})", wind),
            Self::DeadWall => write!(f, "DeadWall"),
            Self::Unordered => write!(f, "Unordered"),
        }
    }
}

impl FromStr for DrawWallPlace {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Segment(東)" => Ok(Self::Segment(Wind::East)),
            "Segment(南)" => Ok(Self::Segment(Wind::South)),
            "Segment(西)" => Ok(Self::Segment(Wind::West)),
            "Segment(北)" => Ok(Self::Segment(Wind::North)),
            "DeadWall" => Ok(Self::DeadWall),
            "Unordered" => Ok(Self::Unordered),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, TS)]
#[ts(export)]
pub struct WallSegment(Vec<TileId>);

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, TS)]
#[ts(export)]
pub struct DrawWall {
    segments: FxHashMap<Wind, WallSegment>,
    dead_wall: WallSegment,
    unordered: Vec<TileId>,
}

pub struct PositionTilesOpts {
    pub shuffle: Option<bool>,
    pub dead_wall: Option<bool>,
}

impl DrawWall {
    pub fn new(tiles: Vec<TileId>) -> Self {
        Self {
            segments: FxHashMap::default(),
            dead_wall: WallSegment::default(),
            unordered: tiles,
        }
    }

    pub fn new_full(tiles: Vec<(TileId, DrawWallPlace)>) -> Self {
        let mut wall = Self::default();

        for (tile, place) in tiles {
            match place {
                DrawWallPlace::Segment(wind) => {
                    let segment = wall
                        .segments
                        .entry(wind)
                        .or_insert_with(WallSegment::default);
                    segment.0.push(tile);
                }
                DrawWallPlace::DeadWall => {
                    wall.dead_wall.0.push(tile);
                }
                DrawWallPlace::Unordered => {
                    wall.unordered.push(tile);
                }
            }
        }

        wall
    }

    pub fn can_draw(&self) -> bool {
        for segment in self.segments.values() {
            if !segment.0.is_empty() {
                return true;
            }
        }

        false
    }

    pub fn len(&self) -> usize {
        self.segments
            .values()
            .map(|segment| segment.0.len())
            .sum::<usize>()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter_all<'a>(
        &'a self,
        vec: &'a mut Vec<(TileId, DrawWallPlace)>,
    ) -> impl DoubleEndedIterator<Item = &'a (TileId, DrawWallPlace)> {
        let mut wall_copy = self.clone();

        for (wind, segment) in wall_copy.segments.iter_mut() {
            for tile in segment.0.iter() {
                vec.push((*tile, DrawWallPlace::Segment(wind.clone())));
            }
        }

        let mut dead_wall = wall_copy.dead_wall.clone();
        while let Some(tile) = dead_wall.0.pop() {
            vec.push((tile, DrawWallPlace::DeadWall));
        }

        let mut unordered = wall_copy.unordered.clone();
        while let Some(tile) = unordered.pop() {
            vec.push((tile, DrawWallPlace::Unordered));
        }

        vec.iter()
    }

    pub fn get_next(&self, wind: &Wind) -> Option<&TileId> {
        let mut wind_index = WINDS_ROUND_ORDER.iter().position(|w| w == wind).unwrap();
        for _ in 0..WINDS_ROUND_ORDER.len() {
            let current_wind = WINDS_ROUND_ORDER.get(wind_index).unwrap();
            let segment = self.segments.get(current_wind);
            if let Some(segment) = segment {
                if !segment.0.is_empty() {
                    return segment.0.last();
                }
            }
            wind_index = (wind_index + 1) % WINDS_ROUND_ORDER.len();
        }

        None
    }
}

impl DrawWall {
    pub fn position_tiles(&mut self, opts: Option<PositionTilesOpts>) {
        let mut use_dead_wall = false;
        if let Some(opts) = opts {
            if let Some(shuffle) = opts.shuffle {
                if shuffle {
                    self.unordered.shuffle(&mut thread_rng());
                }
            }
            if let Some(dead_wall) = opts.dead_wall {
                if dead_wall {
                    use_dead_wall = true;
                }
            }
        }

        let mut current_wind_index = 0;
        let remaining_tiles = if use_dead_wall { 14 } else { 0 };
        while self.unordered.len() > remaining_tiles {
            let tile = self.unordered.pop().unwrap();
            let wind = WINDS_ROUND_ORDER.get(current_wind_index).unwrap().clone();

            let segment = self.segments.entry(wind).or_default();

            segment.0.push(tile);
            current_wind_index = (current_wind_index + 1) % WINDS_ROUND_ORDER.len();
        }

        if use_dead_wall {
            self.dead_wall.0.clone_from(&self.unordered);
        }

        self.unordered.sort();
    }

    pub fn pop_for_wind(&mut self, wind: &Wind) -> Option<TileId> {
        let mut wind_index = WINDS_ROUND_ORDER.iter().position(|w| w == wind)?;

        for _ in 0..WINDS_ROUND_ORDER.len() {
            let loop_wind = WINDS_ROUND_ORDER.get(wind_index)?;
            let segment = self.segments.get_mut(loop_wind)?;
            if !segment.0.is_empty() {
                return segment.0.pop();
            }
            wind_index = (wind_index + 1) % WINDS_ROUND_ORDER.len();
        }

        None
    }

    pub fn clear(&mut self) {
        self.segments.clear();
        self.dead_wall.0.clear();
        self.unordered.clear();
    }

    pub fn replace_tail(&mut self, wind: &Wind, tile: &TileId) {
        let mut wind_index = WINDS_ROUND_ORDER.iter().position(|w| w == wind).unwrap();
        for _ in 0..WINDS_ROUND_ORDER.len() {
            let current_wind = WINDS_ROUND_ORDER.get(wind_index).unwrap();
            let segment = self.segments.get_mut(current_wind).unwrap();
            if segment.0.is_empty() {
                wind_index = (wind_index + 1) % WINDS_ROUND_ORDER.len();
            } else {
                segment.0.pop();
                segment.0.push(*tile);
                break;
            }
        }
    }
}
