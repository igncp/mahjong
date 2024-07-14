pub use self::creation::GameNewOpts;
pub use self::definition::{DrawTileResult, Game, GameId, GamePhase, GameStyle, GameVersion};
pub use self::errors::{
    BreakMeldError, CreateMeldError, DiscardTileError, InitialDrawError, PassNullRoundError,
};
pub use self::players::{PlayerId, Players, PlayersVec};
use crate::{
    deck::DEFAULT_DECK,
    hand::CanSayMahjongError,
    meld::{
        get_is_chow, get_is_kong, get_is_pung, get_tile_claimed_id_for_user, PlayerDiff,
        PossibleMeld, SetCheckOpts,
    },
    round::{Round, RoundTileClaimed},
    Hand, HandTile, TileId,
};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rustc_hash::FxHashSet;
use uuid::Uuid;

mod creation;
mod definition;
mod errors;
mod players;

impl Game {
    pub fn set_players(&mut self, players: &Players) {
        if players.len() != self.players.len() {
            return;
        }

        let current_players = self.players.clone();

        players.clone_into(&mut self.players);

        for (index, player_id) in self.players.iter().enumerate() {
            let current_player = current_players.get(index).unwrap();
            let player_hand = self.table.hands.remove(current_player);
            let score = self.score.remove(current_player);

            self.table.hands.insert(player_id, player_hand);
            self.score.insert(player_id, score);
        }
    }

    pub fn say_mahjong(&mut self, player_id: &PlayerId) -> Result<(), CanSayMahjongError> {
        let hand = self.table.hands.get(player_id);

        hand.can_say_mahjong()?;

        self.calculate_hand_score(player_id);
        let player_index = self.players.iter().position(|p| p == player_id).unwrap();

        self.round.move_after_win(&mut self.phase, player_index);

        if self.phase != GamePhase::End {
            self.phase = GamePhase::InitialShuffle;
        }

        Ok(())
    }

    pub fn pass_null_round(&mut self) -> Result<(), PassNullRoundError> {
        if !self.table.draw_wall.0.is_empty() || self.round.tile_claimed.is_some() {
            for hand in self.table.hands.0.values() {
                if hand.can_drop_tile() {
                    return Err(PassNullRoundError::HandCanDropTile);
                }

                if hand.can_say_mahjong().is_ok() {
                    return Err(PassNullRoundError::HandCanSayMahjong);
                }
            }
        }

        self.round.move_after_draw(&mut self.phase);

        if self.phase != GamePhase::End {
            self.phase = GamePhase::InitialShuffle;
        }

        Ok(())
    }

    pub fn start(&mut self) {
        self.phase = GamePhase::DecidingDealer;
    }

    pub fn decide_dealer(&mut self) {
        self.round.dealer_player_index = 0;
        self.round.east_player_index = 0;
        self.round.player_index = 0;

        self.phase = GamePhase::InitialShuffle;
    }

    pub fn prepare_table(&mut self) {
        self.table = DEFAULT_DECK.create_table(&self.players);
        self.table.draw_wall.0.shuffle(&mut thread_rng());
        self.phase = GamePhase::InitialDraw;
    }

    pub fn initial_draw(&mut self) -> Result<(), InitialDrawError> {
        let tiles_after_claim = self.style.tiles_after_claim();

        for hand in self.table.hands.0.values_mut() {
            for _ in 0..(tiles_after_claim - 1) {
                let tile_id = self.table.draw_wall.0.pop();

                if tile_id.is_none() {
                    return Err(InitialDrawError::NotEnoughTiles);
                }

                let tile = HandTile {
                    id: tile_id.unwrap(),
                    concealed: true,
                    set_id: None,
                };

                hand.push(tile);
            }
        }

        self.phase = GamePhase::Playing;

        Ok(())
    }

    // If `check_for_mahjong` is true, then it will only check for mahjong, if is false, then it
    // will check for melds that are not mahjong (they are exclusive)
    pub fn get_possible_melds_for_player(
        &self,
        player: &PlayerId,
        check_for_mahjong: bool,
    ) -> Vec<PossibleMeld> {
        let mut melds: Vec<PossibleMeld> = vec![];

        let (can_claim_tile, tile_claimed, player_hand) = self.get_can_claim_tile(player);

        let hand = if can_claim_tile {
            let mut hand = player_hand.unwrap().clone();
            hand.push(HandTile {
                concealed: true,
                id: tile_claimed.unwrap(),
                set_id: None,
            });
            hand
        } else {
            player_hand.unwrap().clone()
        };

        let mut round = self.round.clone();
        if can_claim_tile {
            round.tile_claimed = Some(RoundTileClaimed {
                by: Some(player.clone()),
                from: round.tile_claimed.unwrap().from,
                id: tile_claimed.unwrap(),
            });
        }

        let board_tile_player_diff =
            self.get_board_tile_player_diff(Some(&round), Some(&hand), player);
        let claimed_tile = get_tile_claimed_id_for_user(player, &round.tile_claimed);

        let possible_melds =
            hand.get_possible_melds(board_tile_player_diff, claimed_tile, check_for_mahjong);

        for meld in possible_melds {
            melds.push(PossibleMeld {
                discard_tile: None,
                is_mahjong: meld.is_mahjong,
                player_id: player.clone(),
                tiles: meld.tiles,
            });
        }

        melds
    }

    pub fn get_can_claim_tile(&self, player: &PlayerId) -> (bool, Option<TileId>, Option<&Hand>) {
        let tile_claimed = self.round.get_claimable_tile(player);
        let player_hand = self.table.hands.0.get(player);

        if player_hand.is_none() {
            return (false, None, None);
        }

        let player_hand = player_hand.unwrap();
        let can_claim_tile =
            tile_claimed.is_some() && player_hand.len() < self.style.tiles_after_claim();

        (can_claim_tile, tile_claimed, Some(player_hand))
    }

    pub fn get_possible_melds(&self, early_return: bool) -> Vec<PossibleMeld> {
        let mut melds: Vec<PossibleMeld> = vec![];
        let mut players = self.players.clone();

        if early_return {
            players.shuffle();
        }

        for player in &players.0 {
            let mut player_melds = self.get_possible_melds_for_player(player, true);

            if early_return && !player_melds.is_empty() {
                return player_melds;
            }

            melds.append(&mut player_melds);
        }

        for player in &players.0 {
            let mut player_melds = self.get_possible_melds_for_player(player, false);

            if early_return && !player_melds.is_empty() {
                return player_melds;
            }

            melds.append(&mut player_melds);
        }

        melds
    }

    pub fn get_possible_melds_by_discard(&self) -> Vec<PossibleMeld> {
        let mut melds = self.get_possible_melds(false);

        let player_index = self
            .players
            .iter()
            .position(|p| self.table.hands.get(p).len() == self.style.tiles_after_claim());

        if player_index.is_none() {
            return melds;
        }

        let player_index = player_index.unwrap();
        let player_id = self.players.get(player_index).unwrap().clone();

        let player_hand: Vec<HandTile> = self
            .table
            .hands
            .get(&player_id)
            .clone()
            .list
            .into_iter()
            .filter(|t| t.set_id.is_none())
            .collect();

        player_hand.iter().for_each(|hand_tile| {
            let mut game_copy = self.clone();

            game_copy
                .discard_tile_to_board(&hand_tile.id)
                .unwrap_or_default();

            let new_melds = game_copy.get_possible_melds(false);

            new_melds
                .iter()
                .filter(|m| m.player_id != player_id)
                .for_each(|meld| {
                    if !meld.tiles.iter().any(|t| t == &hand_tile.id) {
                        return;
                    }

                    melds.push(PossibleMeld {
                        discard_tile: Some(hand_tile.id),
                        is_mahjong: meld.is_mahjong,
                        player_id: meld.player_id.clone(),
                        tiles: meld.tiles.clone(),
                    });
                });
        });

        melds
    }

    pub fn get_current_player(&self) -> PlayerId {
        self.players.get(self.round.player_index).unwrap().clone()
    }

    pub fn draw_tile_from_wall(&mut self) -> DrawTileResult {
        if self.table.draw_wall.is_empty() {
            return DrawTileResult::WallExhausted;
        }

        if self.round.wall_tile_drawn.is_some() {
            return DrawTileResult::AlreadyDrawn;
        }

        let tile_id = self.table.draw_wall.pop().unwrap();
        let tile = DEFAULT_DECK.0.get(&tile_id).unwrap();

        if tile.is_bonus() {
            let bonus_tiles = self
                .table
                .bonus_tiles
                .get_or_create(&self.get_current_player());

            bonus_tiles.push(tile_id);

            return DrawTileResult::Bonus(tile_id);
        }

        let wall_tile_drawn = Some(tile_id);
        self.round.wall_tile_drawn = wall_tile_drawn;
        let player_id = self.get_current_player();

        let hand = self.table.hands.0.get_mut(&player_id).unwrap();
        hand.push(HandTile::from_id(tile_id));

        DrawTileResult::Normal(tile_id)
    }

    pub fn discard_tile_to_board(&mut self, tile_id: &TileId) -> Result<(), DiscardTileError> {
        let player_with_max_tiles = self
            .players
            .iter()
            .find(|p| self.table.hands.get(p).len() == self.style.tiles_after_claim());

        if player_with_max_tiles.is_none() {
            return Err(DiscardTileError::NoPlayerCanDiscard);
        }

        let player_id = player_with_max_tiles.unwrap().clone();
        let player_hand = self.table.hands.0.get_mut(&player_id).unwrap();
        let tile_index = player_hand.list.iter().position(|t| &t.id == tile_id);

        if tile_index.is_none() {
            return Err(DiscardTileError::PlayerHasNoTile);
        }

        let tile_index = tile_index.unwrap();
        let tile = player_hand.get(tile_index).clone();

        if !tile.concealed {
            return Err(DiscardTileError::TileIsExposed);
        }

        if tile.set_id.is_some() {
            return Err(DiscardTileError::TileIsPartOfMeld);
        }

        if self.round.tile_claimed.is_some() {
            let tile_claimed = self.round.tile_claimed.clone().unwrap();
            if tile_claimed.by.is_some()
                && tile_claimed.by.unwrap() == player_id
                && tile.id != tile_claimed.id
                && player_hand
                    .list
                    .iter()
                    .find(|t| t.id == tile_claimed.id)
                    .unwrap()
                    .set_id
                    .is_none()
            {
                return Err(DiscardTileError::ClaimedAnotherTile);
            }
        }

        player_hand.list.remove(tile_index);

        self.table.board.0.push(tile.id);

        self.round.tile_claimed = Some(RoundTileClaimed {
            from: player_id.clone(),
            id: tile.id,
            by: None,
        });

        Ok(())
    }

    pub fn create_meld(
        &mut self,
        player_id: &PlayerId,
        tiles: &[TileId],
    ) -> Result<(), CreateMeldError> {
        let tiles_set = tiles.iter().cloned().collect::<FxHashSet<TileId>>();
        let hand = self.table.hands.get(player_id);
        let sub_hand_tiles = hand
            .list
            .iter()
            .filter(|t| tiles_set.contains(&t.id))
            .cloned()
            .collect::<Vec<HandTile>>();

        if sub_hand_tiles
            .iter()
            .any(|t| t.set_id.is_some() || !t.concealed)
        {
            return Err(CreateMeldError::TileIsPartOfMeld);
        }

        let sub_hand = Hand::new(sub_hand_tiles);

        let board_tile_player_diff =
            self.get_board_tile_player_diff(None, Some(&sub_hand), player_id);

        let opts_claimed_tile = get_tile_claimed_id_for_user(player_id, &self.round.tile_claimed);

        let opts = SetCheckOpts {
            board_tile_player_diff,
            claimed_tile: opts_claimed_tile,
            sub_hand: &tiles.to_vec(),
        };

        if get_is_pung(&opts) || get_is_chow(&opts) || get_is_kong(&opts) {
            let set_id = Uuid::new_v4().to_string();
            let concealed = board_tile_player_diff.is_none();
            let player_hand = self.table.hands.0.get_mut(player_id).unwrap();

            player_hand
                .list
                .iter_mut()
                .filter(|t| tiles.contains(&t.id))
                .for_each(|tile| {
                    tile.concealed = concealed;
                    tile.set_id = Some(set_id.clone());
                });

            return Ok(());
        }

        Err(CreateMeldError::NotMeld)
    }

    pub fn break_meld(
        &mut self,
        player_id: &PlayerId,
        set_id: &String,
    ) -> Result<(), BreakMeldError> {
        let hand = self.table.hands.0.get(player_id);

        if hand.is_none() {
            return Err(BreakMeldError::MissingHand);
        }

        let mut hand = hand.unwrap().clone();

        for hand_tile in hand.list.iter_mut() {
            if hand_tile.set_id.is_some() && hand_tile.set_id.clone().unwrap() == *set_id {
                if !hand_tile.concealed {
                    return Err(BreakMeldError::TileIsExposed);
                }

                hand_tile.set_id = None;
            }
        }

        self.table.hands.0.insert(player_id.clone(), hand);

        Ok(())
    }

    pub fn get_board_tile_player_diff(
        &self,
        round: Option<&Round>,
        hand: Option<&Hand>,
        player_id: &PlayerId,
    ) -> PlayerDiff {
        let round = round.unwrap_or(&self.round);
        let tile_claimed = round.tile_claimed.clone();

        if let Some(tile_claimed) = tile_claimed {
            tile_claimed.by?;

            let hand = hand.unwrap_or(self.table.hands.get(player_id));
            if !hand.list.iter().any(|h| h.id == tile_claimed.id) {
                return None;
            }

            let player_index = self.players.iter().position(|p| p == player_id);
            let other_player_index = self.players.iter().position(|p| *p == tile_claimed.from);

            if player_index.is_none() || other_player_index.is_none() {
                return None;
            }

            let player_index = player_index.unwrap();
            let other_player_index = other_player_index.unwrap();
            let raw_diff = player_index as i32 - other_player_index as i32;

            return Some(if raw_diff == -3 { 1 } else { raw_diff });
        }

        None
    }

    pub fn claim_tile(&mut self, player_id: &PlayerId) -> bool {
        let player_hand = self.table.hands.0.get_mut(player_id);
        if player_hand.is_none() {
            return false;
        }
        let player_hand = player_hand.unwrap();

        if player_hand.len() != self.style.tiles_after_claim() - 1
            || self.round.tile_claimed.is_none()
            || self.table.board.0.is_empty()
        {
            return false;
        }

        let tile = self.table.board.0.pop().unwrap();

        let mut tile_claimed = self.round.tile_claimed.clone().unwrap();
        tile_claimed.by = Some(player_id.clone());

        self.round.tile_claimed = Some(tile_claimed);
        self.round.player_index = self.players.iter().position(|p| p == player_id).unwrap();

        player_hand.push(HandTile {
            concealed: true,
            id: tile,
            set_id: None,
        });

        true
    }

    pub fn draw_wall_swap_tiles(&mut self, tile_id_a: &TileId, tile_id_b: &TileId) -> bool {
        let draw_wall = &mut self.table.draw_wall;

        let tile_index_a = draw_wall.0.iter().position(|t| t == tile_id_a);
        let tile_index_b = draw_wall.0.iter().position(|t| t == tile_id_b);

        if tile_index_a.is_none() || tile_index_b.is_none() {
            return false;
        }

        let tile_index_a = tile_index_a.unwrap();
        let tile_index_b = tile_index_b.unwrap();

        draw_wall.0.swap(tile_index_a, tile_index_b);

        true
    }

    pub fn get_dealer(&self) -> Option<&PlayerId> {
        self.players.get(self.round.dealer_player_index)
    }

    pub fn update_version(&mut self) {
        self.version = Uuid::new_v4().to_string();
    }
}
