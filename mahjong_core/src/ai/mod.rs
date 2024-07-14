use crate::game::{DrawTileResult, InitialDrawError};
use crate::meld::PossibleMeld;
use crate::{Game, GamePhase, PlayerId, TileId};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rustc_hash::FxHashSet;
use strum_macros::EnumIter;

mod best_drops;

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

#[derive(Debug, Eq, PartialEq, EnumIter, Clone)]
pub enum PlayExitLocation {
    AIPlayerTileDrawn,
    AIPlayerTurnPassed,
    AlreadyEnd,
    AutoStoppedDrawMahjong,
    AutoStoppedDrawNormal,
    ClaimedTile,
    CouldNotClaimTile,
    DecidedDealer,
    InitialDraw,
    InitialDrawError(InitialDrawError),
    InitialShuffle,
    MeldCreated,
    NoAIPlayers,
    NoAction,
    NoAutoDrawTile,
    RoundPassed,
    StartGame,
    SuccessMahjong,
    TileDiscarded,
    TileDrawn,
    TurnPassed,
}

#[derive(Debug, Eq, PartialEq)]
pub struct PlayActionResult {
    pub changed: bool,
    pub exit_location: PlayExitLocation,
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
                exit_location: PlayExitLocation::NoAIPlayers,
            };
        }

        match self.game.phase {
            GamePhase::InitialShuffle => {
                self.game.prepare_table();

                return PlayActionResult {
                    changed: true,
                    exit_location: PlayExitLocation::InitialShuffle,
                };
            }
            GamePhase::DecidingDealer => {
                self.game.decide_dealer();
                return PlayActionResult {
                    changed: true,
                    exit_location: PlayExitLocation::DecidedDealer,
                };
            }
            GamePhase::Beginning => {
                self.can_draw_round = true;

                self.game.start();

                return PlayActionResult {
                    changed: true,
                    exit_location: PlayExitLocation::StartGame,
                };
            }
            GamePhase::InitialDraw => match self.game.initial_draw() {
                Ok(_) => {
                    return PlayActionResult {
                        changed: true,
                        exit_location: PlayExitLocation::InitialDraw,
                    };
                }
                Err(e) => {
                    return PlayActionResult {
                        changed: false,
                        exit_location: PlayExitLocation::InitialDrawError(e),
                    };
                }
            },
            GamePhase::End => {
                return PlayActionResult {
                    changed: false,
                    exit_location: PlayExitLocation::AlreadyEnd,
                };
            }
            GamePhase::Playing => {}
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

                    if mahjong_success.is_ok() {
                        return PlayActionResult {
                            changed: true,
                            exit_location: PlayExitLocation::SuccessMahjong,
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
                                    exit_location: PlayExitLocation::ClaimedTile,
                                };
                            } else {
                                // Unexpected state
                                return PlayActionResult {
                                    changed: false,
                                    exit_location: PlayExitLocation::CouldNotClaimTile,
                                };
                            }
                        }
                    }
                }

                if meld.discard_tile.is_some() {
                    continue;
                }

                let meld_created = self.game.create_meld(&meld.player_id, &meld.tiles);

                if meld_created.is_ok() {
                    return PlayActionResult {
                        changed: true,
                        exit_location: PlayExitLocation::MeldCreated,
                    };
                }
            }
        }

        let current_player = self.game.get_current_player();

        if self.ai_players.contains(&current_player) {
            let is_tile_claimed = self.game.round.tile_claimed.is_some();

            if !is_tile_claimed {
                let tile_drawn = self.game.draw_tile_from_wall();

                match tile_drawn {
                    DrawTileResult::Bonus(_) | DrawTileResult::Normal(_) => {
                        if let DrawTileResult::Normal(_) = tile_drawn {
                            if self.sort_on_draw {
                                self.game.table.hands.sort_player_hand(&current_player);
                            }
                        }

                        return PlayActionResult {
                            changed: true,
                            exit_location: PlayExitLocation::AIPlayerTileDrawn,
                        };
                    }
                    DrawTileResult::AlreadyDrawn | DrawTileResult::WallExhausted => {}
                };
            }

            let player_hand = self.game.table.hands.0.get(&current_player).unwrap();
            if player_hand.len() == self.game.style.tiles_after_claim() {
                let mut tiles_without_meld = player_hand
                    .list
                    .iter()
                    .filter(|tile| tile.set_id.is_none())
                    .map(|tile| tile.id)
                    .collect::<Vec<TileId>>();

                if !tiles_without_meld.is_empty() {
                    tiles_without_meld.shuffle(&mut thread_rng());
                    let tile_to_discard = tiles_without_meld[0];

                    let discarded = self.game.discard_tile_to_board(&tile_to_discard);

                    if discarded.is_ok() {
                        return PlayActionResult {
                            changed: true,
                            exit_location: PlayExitLocation::TileDiscarded,
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
                                exit_location: PlayExitLocation::AutoStoppedDrawMahjong,
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
                                exit_location: PlayExitLocation::AutoStoppedDrawNormal,
                            };
                        }
                    }
                }
                let success = self.game.round.next_turn(&self.game.table.hands);

                if success.is_ok() {
                    return PlayActionResult {
                        changed: true,
                        exit_location: PlayExitLocation::AIPlayerTurnPassed,
                    };
                }
            };
        } else {
            let is_tile_claimed = self.game.round.tile_claimed.is_some();

            if !is_tile_claimed {
                if !self.draw_tile_for_real_player {
                    return PlayActionResult {
                        changed: false,
                        exit_location: PlayExitLocation::NoAutoDrawTile,
                    };
                }

                let tile_drawn = self.game.draw_tile_from_wall();

                match tile_drawn {
                    DrawTileResult::Bonus(_) | DrawTileResult::Normal(_) => {
                        if let DrawTileResult::Normal(_) = tile_drawn {
                            if self.sort_on_draw {
                                self.game.table.hands.sort_player_hand(&current_player);
                            }
                        }

                        return PlayActionResult {
                            changed: true,
                            exit_location: PlayExitLocation::TileDrawn,
                        };
                    }
                    DrawTileResult::AlreadyDrawn | DrawTileResult::WallExhausted => {}
                };
            } else if self.can_pass_turn {
                let player_hand = self.game.table.hands.0.get(&current_player).unwrap();
                if player_hand.len() < self.game.style.tiles_after_claim() {
                    let success = self.game.round.next_turn(&self.game.table.hands);

                    if success.is_ok() {
                        return PlayActionResult {
                            changed: true,
                            exit_location: PlayExitLocation::TurnPassed,
                        };
                    }
                }
            }
        }

        if self.game.table.draw_wall.0.is_empty() && self.can_draw_round {
            let round_passed = self.game.pass_null_round();

            if round_passed.is_ok() {
                return PlayActionResult {
                    changed: true,
                    exit_location: PlayExitLocation::RoundPassed,
                };
            }
        }

        PlayActionResult {
            changed: false,
            exit_location: PlayExitLocation::NoAction,
        }
    }

    pub fn get_is_after_discard(&self) -> bool {
        let current_player = self.game.get_current_player();
        let current_hand = self.game.table.hands.get(&current_player);

        current_hand.len() < self.game.style.tiles_after_claim()
            && self.game.round.tile_claimed.is_some()
    }
}
