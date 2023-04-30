use crate::game_summary::GameSummary;
use crate::{Game, PlayerId, TileId};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

// Naive AI as a placeholder which can be extended later
pub struct StandardAI<'a> {
    game: &'a mut Game,
    ai_players: HashSet<PlayerId>,
    pub can_pass_turn: bool,
    pub draw: bool,
}

pub struct PlayActionResult {
    pub changed: bool,
    pub tile_discarded: Option<bool>,
}

impl<'a> StandardAI<'a> {
    pub fn new(game: &'a mut Game, ai_players: HashSet<PlayerId>) -> Self {
        Self {
            ai_players,
            can_pass_turn: true,
            draw: true,
            game,
        }
    }

    pub fn play_action(&mut self) -> PlayActionResult {
        if self.ai_players.is_empty() {
            return PlayActionResult {
                changed: false,
                tile_discarded: None,
            };
        }

        // TODO: Check if any player can claim a tile that would produce a meld
        // - Stop if a non-AI player could claim the tile for meld
        // let possible_melds_with_discard = self.game.get_possible_melds_by_discard();

        // Check if any meld can be created with existing cards
        let melds = self.game.get_possible_melds();
        for meld in melds {
            if self.ai_players.contains(&meld.player_id) && meld.discard_tile.is_none() {
                let tiles = meld.tiles.iter().cloned().collect::<HashSet<TileId>>();
                let meld_created = self.game.create_meld(&meld.player_id, &tiles);

                if meld_created {
                    return PlayActionResult {
                        changed: true,
                        tile_discarded: Some(false),
                    };
                }
            }
        }

        let current_player = self.game.get_current_player();
        if self.ai_players.contains(&current_player) {
            let is_tile_claimed = self.game.round.tile_claimed.is_some();

            if !is_tile_claimed {
                let tile_drawn = self.game.draw_tile_from_wall();

                if tile_drawn.is_some() {
                    return PlayActionResult {
                        changed: true,
                        tile_discarded: Some(false),
                    };
                }
            }

            let player_tiles = self.game.table.hands.get(&current_player).unwrap();
            if player_tiles.0.len() == 14 {
                let mut tiles_without_meld = player_tiles
                    .0
                    .iter()
                    .filter(|tile| tile.set_id.is_none())
                    .map(|tile| tile.id)
                    .collect::<Vec<TileId>>();

                if !tiles_without_meld.is_empty() {
                    tiles_without_meld.shuffle(&mut thread_rng());
                    let tile_to_discard = tiles_without_meld[0];

                    let discarded = self.game.discard_tile_to_board(&tile_to_discard);

                    if discarded {
                        return PlayActionResult {
                            changed: true,
                            tile_discarded: Some(true),
                        };
                    }
                }
            } else if self.can_pass_turn {
                let success = self.game.round.next(&self.game.table.hands);

                if success {
                    return PlayActionResult {
                        changed: true,
                        tile_discarded: Some(false),
                    };
                }
            }
        } else {
            let is_tile_claimed = self.game.round.tile_claimed.is_some();

            if !is_tile_claimed {
                if !self.draw {
                    return PlayActionResult {
                        changed: false,
                        tile_discarded: None,
                    };
                }

                let tile_drawn = self.game.draw_tile_from_wall();

                if tile_drawn.is_some() {
                    return PlayActionResult {
                        changed: false,
                        tile_discarded: Some(false),
                    };
                }
            } else if self.can_pass_turn {
                let player_tiles = self.game.table.hands.get(&current_player).unwrap();
                if player_tiles.0.len() == 13 {
                    let success = self.game.round.next(&self.game.table.hands);

                    if success {
                        return PlayActionResult {
                            changed: false,
                            tile_discarded: Some(false),
                        };
                    }
                }
            }
        }

        PlayActionResult {
            changed: false,
            tile_discarded: None,
        }
    }

    pub fn get_is_after_discard(&self) -> bool {
        let current_player = self.game.get_current_player();
        let current_hand = self.game.table.hands.get(&current_player).unwrap();

        current_hand.0.len() == 13 && self.game.round.tile_claimed.is_some()
    }

    // This can become complex if it takes into account different scoring rules.
    // For now it should only take into account the possibility to create a meld.
    // In future it should review which tiles have been claimed by other players.
    // Should have some unit tests.
    // TODO: finalise
    pub fn get_best_drops(&self, player_id: &PlayerId) -> Option<Vec<TileId>> {
        let game_clone = self.game.clone();
        let game_summary = GameSummary::from_game(&game_clone, player_id)?;

        if game_summary.hand.0.len() != 14 {
            return None;
        }

        struct TileDrop {
            id: TileId,
            score: usize,
        }

        let mut drops: Vec<TileDrop> = vec![];

        for tile in game_summary.hand.0.iter() {
            if tile.set_id.is_some() {
                drops.push(TileDrop {
                    id: tile.id,
                    score: 0,
                });
            }

            // - Check how possible is it to build a meld with this tile (score can also be one if
            // one tile left)
        }

        // Best drops sorted from left to right
        drops.sort_by(|a, b| a.score.cmp(&b.score));

        let best_drops = drops.iter().map(|drop| drop.id).collect::<Vec<TileId>>();

        Some(best_drops)
    }
}
