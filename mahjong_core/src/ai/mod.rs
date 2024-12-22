use crate::game::{DrawError, DrawTileResult};
use crate::meld::PossibleMeld;
use crate::{Game, GamePhase, PlayerId, TileId, Wind, WINDS_ROUND_ORDER};
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
    pub dealer_order_deterministic: Option<bool>,
    pub draw_tile_for_real_player: bool,
    pub game: &'a mut Game,
    pub shuffle_players: bool,
    pub sort_on_draw: bool,
    pub sort_on_initial_draw: bool,
    pub with_dead_wall: bool,
}

#[derive(Debug, Eq, PartialEq, EnumIter, Clone)]
pub enum PlayExitLocation {
    AIPlayerTileDrawn,
    AIPlayerTurnPassed,
    AlreadyEnd,
    AutoStoppedDrawMahjong,
    AutoStoppedDrawNormal,
    ClaimedTile,
    CompletedPlayers,
    CouldNotClaimTile,
    DecidedDealer,
    FinishedCharleston,
    InitialDraw,
    InitialDrawError(DrawError),
    InitialShuffle,
    MeldCreated,
    NewRoundFromMeld,
    NoAction,
    NoAutoDrawTile,
    RoundPassed,
    StartGame,
    SuccessMahjong,
    TileDiscarded,
    TileDrawn,
    TurnPassed,
    WaitingDealerOrder,
    WaitingPlayers,
}

// This is used for debugging unexpected "NoAction" results
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Metadata {}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PlayActionResult {
    pub changed: bool,
    pub exit_location: PlayExitLocation,
    pub metadata: Option<Metadata>,
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

impl StandardAI<'_> {
    pub fn get_is_after_discard(&self) -> bool {
        let current_player = self.game.get_current_player();
        if current_player.is_none() {
            return false;
        }
        let current_hand = self.game.table.hands.get(&current_player.unwrap());

        current_hand.unwrap().len() < self.game.style.tiles_after_claim()
            && self.game.round.tile_claimed.is_some()
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
            dealer_order_deterministic: None,
            draw_tile_for_real_player: true,
            game,
            shuffle_players: false,
            sort_on_draw: false,
            sort_on_initial_draw: false,
            with_dead_wall: false,
        }
    }

    pub fn play_action(&mut self, with_metadata: bool) -> PlayActionResult {
        let mut metadata: Option<Metadata> = None;

        if with_metadata {
            metadata = Some(Metadata {});
        }

        match self.game.phase {
            GamePhase::Charleston => {
                let finished_charleston = self.game.move_charleston();

                if finished_charleston.is_ok() {
                    return PlayActionResult {
                        changed: true,
                        exit_location: PlayExitLocation::FinishedCharleston,
                        metadata,
                    };
                }

                return PlayActionResult {
                    changed: false,
                    exit_location: PlayExitLocation::NoAction,
                    metadata,
                };
            }
            GamePhase::WaitingPlayers => {
                return match self.game.complete_players(self.shuffle_players) {
                    Ok(_) => PlayActionResult {
                        changed: true,
                        exit_location: PlayExitLocation::CompletedPlayers,
                        metadata,
                    },
                    Err(_) => PlayActionResult {
                        changed: false,
                        exit_location: PlayExitLocation::WaitingPlayers,
                        metadata,
                    },
                };
            }
            GamePhase::InitialShuffle => {
                self.game.prepare_table(self.with_dead_wall);

                return PlayActionResult {
                    changed: true,
                    exit_location: PlayExitLocation::InitialShuffle,
                    metadata,
                };
            }
            GamePhase::DecidingDealer => {
                if self.dealer_order_deterministic.is_some() {
                    let dealer_order_deterministic = self.dealer_order_deterministic.unwrap();

                    self.game
                        .round
                        .set_initial_winds(Some(if dealer_order_deterministic {
                            WINDS_ROUND_ORDER.clone()
                        } else {
                            let mut winds: [Wind; 4] = WINDS_ROUND_ORDER.clone();
                            winds.shuffle(&mut thread_rng());
                            winds
                        }))
                        .unwrap();
                } else if self.game.round.initial_winds.is_none() {
                    return PlayActionResult {
                        metadata,
                        changed: false,
                        exit_location: PlayExitLocation::WaitingDealerOrder,
                    };
                }

                self.game.decide_dealer().unwrap();

                return PlayActionResult {
                    changed: true,
                    metadata,
                    exit_location: PlayExitLocation::DecidedDealer,
                };
            }
            GamePhase::Beginning => {
                self.game.start(self.shuffle_players);

                return PlayActionResult {
                    changed: true,
                    metadata,
                    exit_location: PlayExitLocation::StartGame,
                };
            }
            GamePhase::InitialDraw => match self.game.initial_draw() {
                Ok(_) => {
                    if self.sort_on_initial_draw {
                        for player in self.game.table.hands.0.clone().keys() {
                            self.game.table.hands.sort_player_hand(player);
                        }
                    }

                    return PlayActionResult {
                        changed: true,
                        exit_location: PlayExitLocation::InitialDraw,
                        metadata,
                    };
                }
                Err(e) => {
                    return PlayActionResult {
                        metadata,
                        changed: false,
                        exit_location: PlayExitLocation::InitialDrawError(e),
                    };
                }
            },
            GamePhase::End => {
                return PlayActionResult {
                    changed: false,
                    exit_location: PlayExitLocation::AlreadyEnd,
                    metadata,
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
                            metadata,
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
                                    metadata,
                                };
                            } else {
                                // Unexpected state
                                return PlayActionResult {
                                    changed: false,
                                    exit_location: PlayExitLocation::CouldNotClaimTile,
                                    metadata,
                                };
                            }
                        }
                    }
                }

                if meld.discard_tile.is_some() {
                    continue;
                }

                let phase_before = self.game.phase;

                let meld_created = self.game.create_meld(
                    &meld.player_id,
                    &meld.tiles,
                    meld.is_upgrade,
                    meld.is_concealed,
                );

                if phase_before == GamePhase::Playing && self.game.phase != GamePhase::Playing {
                    return PlayActionResult {
                        changed: true,
                        exit_location: PlayExitLocation::NewRoundFromMeld,
                        metadata,
                    };
                }

                if meld_created.is_ok() {
                    return PlayActionResult {
                        changed: true,
                        exit_location: PlayExitLocation::MeldCreated,
                        metadata,
                    };
                }
            }
        }

        let current_player = self.game.get_current_player().unwrap();

        if self.ai_players.contains(&current_player) {
            let is_tile_claimed = self.game.round.tile_claimed.is_some();

            if !is_tile_claimed {
                let tile_drawn = self.game.draw_tile_from_wall();

                match tile_drawn {
                    DrawTileResult::Bonus(_) | DrawTileResult::Normal(_) => {
                        if let DrawTileResult::Normal(_) = tile_drawn {
                            if self.sort_on_initial_draw {
                                self.game.table.hands.sort_player_hand(&current_player);
                            }
                        }

                        return PlayActionResult {
                            changed: true,
                            exit_location: PlayExitLocation::AIPlayerTileDrawn,
                            metadata,
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
                    let tile_to_discard = 'a: {
                        if let Some(tile_claimed) = self.game.round.tile_claimed.clone() {
                            for tile in tiles_without_meld.iter() {
                                if tile_claimed.id == *tile {
                                    break 'a tile_claimed.id;
                                }
                            }
                        }

                        tiles_without_meld.shuffle(&mut thread_rng());
                        tiles_without_meld[0]
                    };

                    let discarded = self.game.discard_tile_to_board(&tile_to_discard);

                    if discarded.is_ok() {
                        return PlayActionResult {
                            changed: true,
                            exit_location: PlayExitLocation::TileDiscarded,
                            metadata,
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
                                metadata,
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
                                metadata,
                            };
                        }
                    }
                }
                let success = self.game.round.next_turn(&self.game.table.hands);

                if success.is_ok() {
                    return PlayActionResult {
                        changed: true,
                        exit_location: PlayExitLocation::AIPlayerTurnPassed,
                        metadata,
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
                        metadata,
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
                            metadata,
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
                            metadata,
                        };
                    }
                }
            }
        }

        if self.game.table.draw_wall.is_empty() && self.can_draw_round {
            let round_passed = self.game.pass_null_round();

            if round_passed.is_ok() {
                return PlayActionResult {
                    changed: true,
                    exit_location: PlayExitLocation::RoundPassed,
                    metadata,
                };
            }
        }

        PlayActionResult {
            changed: false,
            exit_location: PlayExitLocation::NoAction,
            metadata,
        }
    }
}
