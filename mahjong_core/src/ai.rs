use crate::{Game, PlayerId, TileId};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

// Naive AI as a placeholder which can be extended later
pub struct StandardAI<'a> {
    game: &'a mut Game,
    ai_players: &'a HashSet<PlayerId>,
    pub draw: bool,
}

impl<'a> StandardAI<'a> {
    pub fn new(game: &'a mut Game, ai_players: &'a HashSet<PlayerId>) -> Self {
        Self {
            ai_players,
            draw: true,
            game,
        }
    }

    pub fn play_action(&mut self) -> bool {
        if self.ai_players.is_empty() {
            return false;
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
                    return true;
                }
            }
        }

        let current_player = self.game.get_current_player();
        if self.ai_players.contains(&current_player) {
            let is_tile_claimed = self.game.round.tile_claimed.is_some();

            if !is_tile_claimed {
                let tile_drawn = self.game.draw_tile_from_wall();

                if tile_drawn.is_some() {
                    return true;
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
                        return true;
                    }
                }
            } else {
                let success = self.game.round.next(&self.game.table.hands);

                if success {
                    return true;
                }
            }
        } else {
            let is_tile_claimed = self.game.round.tile_claimed.is_some();

            if !is_tile_claimed {
                if !self.draw {
                    return false;
                }

                let tile_drawn = self.game.draw_tile_from_wall();

                if tile_drawn.is_some() {
                    return true;
                }
            } else {
                let player_tiles = self.game.table.hands.get(&current_player).unwrap();
                if player_tiles.0.len() == 13 {
                    let success = self.game.round.next(&self.game.table.hands);

                    if success {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn get_is_after_discard(&self) -> bool {
        let current_player = self.game.get_current_player();
        let current_hand = self.game.table.hands.get(&current_player).unwrap();

        current_hand.0.len() == 13 && self.game.round.tile_claimed.is_some()
    }
}
