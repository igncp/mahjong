use crate::meld::PossibleMeld;
use crate::{Game, PlayerId, TileId};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rustc_hash::FxHashSet;

mod best_drops;
mod test_ai;

// Naive AI as a placeholder which can be extended later
pub struct StandardAI<'a> {
    ai_players: FxHashSet<PlayerId>,
    pub auto_stop_claim_meld: FxHashSet<PlayerId>,
    pub can_draw_round: bool,
    pub can_pass_turn: bool,
    pub draw_tile_for_real_player: bool,
    pub game: &'a mut Game,
    pub sort_on_draw: bool,
}

pub struct PlayActionResult {
    pub changed: bool,
    pub tile_discarded: Option<bool>,
}

pub fn sort_by_is_mahjong(a: &PossibleMeld, b: &PossibleMeld) -> std::cmp::Ordering {
    if a.is_mahjong && !b.is_mahjong {
        std::cmp::Ordering::Less
    } else if !a.is_mahjong && b.is_mahjong {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}

impl<'a> StandardAI<'a> {
    pub fn new(
        game: &'a mut Game,
        ai_players: FxHashSet<PlayerId>,
        auto_stop_claim_meld: FxHashSet<PlayerId>,
    ) -> Self {
        Self {
            ai_players,
            auto_stop_claim_meld,
            can_draw_round: false,
            can_pass_turn: true,
            draw_tile_for_real_player: true,
            game,
            sort_on_draw: false,
        }
    }

    pub fn play_action(&mut self) -> PlayActionResult {
        if self.ai_players.is_empty() {
            return PlayActionResult {
                changed: false,
                tile_discarded: None,
            };
        }

        // Check if any meld can be created with existing cards
        let mut melds = self.game.get_possible_melds(true);

        // Suffle melds
        let mut rng = thread_rng();
        melds.shuffle(&mut rng);
        melds.sort_by(sort_by_is_mahjong);

        for meld in melds {
            if self.ai_players.contains(&meld.player_id) {
                if meld.is_mahjong {
                    let mahjong_success = self.game.say_mahjong(&meld.player_id);

                    if mahjong_success {
                        return PlayActionResult {
                            changed: true,
                            tile_discarded: None,
                        };
                    }
                }

                let player_hand = self.game.table.hands.0.get(&meld.player_id).unwrap();
                let missing_tile = meld
                    .tiles
                    .iter()
                    .find(|tile| !player_hand.get_has_tile(tile));

                if let Some(missing_tile) = missing_tile {
                    if let Some(claimable_type) =
                        self.game.round.get_claimable_tile(&meld.player_id)
                    {
                        if claimable_type == *missing_tile {
                            let was_tile_claimed = self.game.claim_tile(&meld.player_id);

                            if was_tile_claimed {
                                return PlayActionResult {
                                    changed: true,
                                    tile_discarded: None,
                                };
                            }
                        }
                    }
                }

                if meld.discard_tile.is_some() {
                    continue;
                }

                let tiles = meld.tiles.iter().cloned().collect::<FxHashSet<TileId>>();
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
                    if self.sort_on_draw {
                        let mut hand = self
                            .game
                            .table
                            .hands
                            .0
                            .get(&current_player)
                            .unwrap()
                            .clone();
                        hand.sort_default();
                        self.game.table.hands.0.insert(current_player, hand);
                    }

                    return PlayActionResult {
                        changed: true,
                        tile_discarded: Some(false),
                    };
                }
            }

            let player_hand = self.game.table.hands.0.get(&current_player).unwrap();
            if player_hand.len() == self.game.style.tiles_after_claim() {
                let mut tiles_without_meld = player_hand
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
                let auto_stop_claim_meld = self.auto_stop_claim_meld.clone();
                if !auto_stop_claim_meld.is_empty() {
                    for player in auto_stop_claim_meld {
                        if player.is_empty() {
                            continue;
                        }
                        let (can_claim_tile, tile_claimed, _) =
                            self.game.get_can_claim_tile(&player);

                        if !can_claim_tile {
                            continue;
                        }

                        let tile_claimed = tile_claimed.unwrap();

                        let melds_mahjong = self.game.get_possible_melds_for_player(&player, true);
                        let melds_with_draw_mahjong = melds_mahjong
                            .iter()
                            .filter(|meld| meld.tiles.contains(&tile_claimed))
                            .collect::<Vec<&PossibleMeld>>();

                        if !melds_with_draw_mahjong.is_empty() {
                            return PlayActionResult {
                                changed: false,
                                tile_discarded: None,
                            };
                        }

                        let melds_normal = self.game.get_possible_melds_for_player(&player, false);
                        let melds_with_draw_normal = melds_normal
                            .iter()
                            .filter(|meld| meld.tiles.contains(&tile_claimed))
                            .collect::<Vec<&PossibleMeld>>();

                        if !melds_with_draw_normal.is_empty() {
                            return PlayActionResult {
                                changed: false,
                                tile_discarded: None,
                            };
                        }
                    }
                }
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
                if !self.draw_tile_for_real_player {
                    return PlayActionResult {
                        changed: false,
                        tile_discarded: None,
                    };
                }

                let tile_drawn = self.game.draw_tile_from_wall();

                if tile_drawn.is_some() {
                    if self.sort_on_draw {
                        let mut hand = self
                            .game
                            .table
                            .hands
                            .0
                            .get(&current_player)
                            .unwrap()
                            .clone();
                        hand.sort_default();
                        self.game.table.hands.0.insert(current_player, hand);
                    }

                    return PlayActionResult {
                        changed: false,
                        tile_discarded: Some(false),
                    };
                }
            } else if self.can_pass_turn {
                let player_hand = self.game.table.hands.0.get(&current_player).unwrap();
                if player_hand.len() < self.game.style.tiles_after_claim() {
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

        if self.game.table.draw_wall.0.is_empty() && self.can_draw_round {
            let round_passed = self.game.pass_null_round();

            if round_passed {
                return PlayActionResult {
                    changed: true,
                    tile_discarded: None,
                };
            }
        }

        PlayActionResult {
            changed: false,
            tile_discarded: None,
        }
    }

    pub fn get_is_after_discard(&self) -> bool {
        let current_player = self.game.get_current_player();
        let current_hand = self.game.table.hands.get(&current_player);

        current_hand.len() < self.game.style.tiles_after_claim()
            && self.game.round.tile_claimed.is_some()
    }
}
