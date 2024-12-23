pub use self::creation::GameNewOpts;
pub use self::definition::{DrawTileResult, Game, GameId, GamePhase, GameStyle, GameVersion};
use self::errors::DecideDealerError;
pub use self::errors::{
    BreakMeldError, CreateMeldError, DiscardTileError, DrawError, PassNullRoundError,
};
pub use self::players::{PlayerId, Players, PlayersVec};
use crate::hand::KongTile;
use crate::table::PositionTilesOpts;
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
use crate::{Tile, Wind, WINDS_ROUND_ORDER};
use rustc_hash::FxHashSet;
use uuid::Uuid;

mod charleston;
mod creation;
mod definition;
mod errors;
mod players;

impl Game {
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
                is_concealed: meld.is_concealed,
                is_mahjong: meld.is_mahjong,
                is_upgrade: meld.is_upgrade,
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
            .position(|p| self.table.hands.get(p).unwrap().len() == self.style.tiles_after_claim());

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
            .unwrap()
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
                        is_concealed: meld.is_concealed,
                        is_mahjong: meld.is_mahjong,
                        is_upgrade: meld.is_upgrade,
                        player_id: meld.player_id.clone(),
                        tiles: meld.tiles.clone(),
                    });
                });
        });

        melds
    }

    pub fn get_current_player(&self) -> Option<PlayerId> {
        self.players.get(self.round.player_index).cloned()
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

            let hand = match hand {
                Some(hand) => hand,
                None => self.table.hands.0.get(player_id).unwrap(),
            };
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

    pub fn get_dealer(&self) -> Option<&PlayerId> {
        self.players.get(self.round.dealer_player_index)
    }

    pub fn get_player_wind(&self) -> Wind {
        let current_player = self.get_current_player().unwrap();

        self.round.get_player_wind(&self.players.0, &current_player)
    }
}

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
        let hand = self.table.hands.get(player_id).unwrap();

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
        if self.table.draw_wall.can_draw() {
            return Err(PassNullRoundError::WallNotEmpty);
        }

        if self.round.tile_claimed.is_some() {
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

    pub fn start(&mut self, shuffle_players: bool) {
        if self.phase != GamePhase::Beginning {
            return;
        }

        self.phase = GamePhase::WaitingPlayers;

        self.complete_players(shuffle_players).unwrap_or_default();
    }

    pub fn decide_dealer(&mut self) -> Result<(), DecideDealerError> {
        let mut new_players = self.players.clone();
        let winds = self.round.get_initial_winds_slice();

        winds.iter(|(player_index, player_wind)| {
            let wind_index = WINDS_ROUND_ORDER
                .iter()
                .position(|w| w == player_wind)
                .unwrap();

            new_players.0[wind_index].clone_from(&self.players.0[player_index]);
        });

        self.round.dealer_player_index = 0;
        self.round.east_player_index = 0;
        self.round.player_index = 0;
        self.round.tile_claimed = None;

        self.phase = GamePhase::InitialShuffle;

        Ok(())
    }

    pub fn prepare_table(&mut self, with_dead_wall: bool) {
        self.table = DEFAULT_DECK.create_table(&self.players);
        self.table.draw_wall.position_tiles(Some(PositionTilesOpts {
            shuffle: Some(true),
            dead_wall: Some(with_dead_wall),
        }));
        self.phase = GamePhase::InitialDraw;
    }

    fn draw_tile_for_player(&mut self, player_id: &PlayerId) -> Result<(), DrawError> {
        let player_wind = self.round.get_player_wind(&self.players.0, player_id);

        loop {
            let tile_id = self.table.draw_wall.pop_for_wind(&player_wind);

            if tile_id.is_none() {
                return Err(DrawError::NotEnoughTiles);
            }

            let tile_id = tile_id.unwrap();
            let is_bonus = DEFAULT_DECK.0[tile_id].is_bonus();

            if is_bonus {
                let bonus_tiles = self.table.bonus_tiles.get_or_create(player_id);

                bonus_tiles.push(tile_id);
                continue;
            }

            let hand = self.table.hands.0.get_mut(player_id).unwrap();
            hand.push(HandTile::from_id(tile_id));
            return Ok(());
        }
    }

    pub fn initial_draw(&mut self) -> Result<(), DrawError> {
        let tiles_after_claim = self.style.tiles_after_claim();

        for player_id in self.players.0.clone() {
            'loop_label: loop {
                let hand = self.table.hands.0.get(&player_id).unwrap();
                if hand.len() == tiles_after_claim - 1 {
                    break 'loop_label;
                }

                self.draw_tile_for_player(&player_id)?;
            }
        }

        self.phase = GamePhase::Playing;

        Ok(())
    }

    pub fn draw_tile_from_wall(&mut self) -> DrawTileResult {
        if self.table.draw_wall.is_empty() {
            return DrawTileResult::WallExhausted;
        }

        if self.round.wall_tile_drawn.is_some() {
            return DrawTileResult::AlreadyDrawn;
        }

        let player_wind = self.get_player_wind();
        let tile_id = self.table.draw_wall.pop_for_wind(&player_wind);

        if tile_id.is_none() {
            return DrawTileResult::WallExhausted;
        }

        let tile_id = tile_id.unwrap();

        let tile = &DEFAULT_DECK.0[tile_id];

        if tile.is_bonus() {
            let bonus_tiles = self
                .table
                .bonus_tiles
                .get_or_create(&self.get_current_player().unwrap());

            bonus_tiles.push(tile_id);

            return DrawTileResult::Bonus(tile_id);
        }

        let wall_tile_drawn = Some(tile_id);
        self.round.wall_tile_drawn = wall_tile_drawn;
        let player_id = self.get_current_player().unwrap();

        let hand = self.table.hands.0.get_mut(&player_id).unwrap();
        hand.push(HandTile::from_id(tile_id));

        DrawTileResult::Normal(tile_id)
    }

    pub fn discard_tile_to_board(&mut self, tile_id: &TileId) -> Result<(), DiscardTileError> {
        let player_with_max_tiles = self
            .players
            .iter()
            .find(|p| self.table.hands.get(p).unwrap().len() == self.style.tiles_after_claim());

        if player_with_max_tiles.is_none() {
            return Err(DiscardTileError::NoPlayerCanDiscard);
        }

        let player_id = player_with_max_tiles.unwrap().clone();
        let player_hand = self.table.hands.0.get_mut(&player_id).unwrap();
        let tiles_with_id = player_hand
            .list
            .iter()
            .filter(|t| t.id == *tile_id)
            .collect::<Vec<_>>();
        let tile_index = player_hand
            .list
            .iter()
            .position(|t| &t.id == tile_id && (tiles_with_id.len() == 1 || t.set_id.is_none()));

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

        if let Some(tile_claimed) = self.round.tile_claimed.clone() {
            if let Some(by) = tile_claimed.by {
                if by == player_id
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
        is_upgrade: bool,
        is_concealed: bool,
    ) -> Result<(), CreateMeldError> {
        let tiles_set = tiles.iter().cloned().collect::<FxHashSet<TileId>>();
        let hand = self.table.hands.get(player_id);
        let sub_hand_tiles = hand
            .unwrap()
            .list
            .iter()
            .filter(|t| tiles_set.contains(&t.id))
            .cloned()
            .collect::<Vec<HandTile>>();

        if !is_upgrade
            && sub_hand_tiles
                .iter()
                .any(|t| t.set_id.is_some() || !t.concealed)
        {
            return Err(CreateMeldError::TileIsPartOfMeld);
        }

        let sub_hand = Hand::new(sub_hand_tiles);

        let board_tile_player_diff =
            self.get_board_tile_player_diff(None, Some(&sub_hand), player_id);

        let opts_claimed_tile = get_tile_claimed_id_for_user(player_id, &self.round.tile_claimed);
        let tiles_full: Vec<&Tile> = tiles.iter().map(|t| &DEFAULT_DECK.0[*t]).collect();

        let opts = SetCheckOpts {
            board_tile_player_diff,
            claimed_tile: opts_claimed_tile,
            sub_hand: &tiles_full,
        };

        let mut is_kong = false;

        if get_is_pung(&opts) || get_is_chow(&opts) || {
            is_kong = get_is_kong(&opts);
            is_kong
        } {
            if (is_upgrade && !is_kong) || (is_concealed && opts_claimed_tile.is_some()) {
                return Err(CreateMeldError::NotMeld);
            }

            let set_id = Uuid::new_v4().to_string();
            let player_hand = self.table.hands.0.get_mut(player_id).unwrap();

            for tile in tiles.iter() {
                let tile = player_hand.list.iter().find(|t| t.id == *tile);

                if tile.is_none() {
                    return Err(CreateMeldError::NotMeld);
                }
            }

            player_hand
                .list
                .iter_mut()
                .filter(|t| tiles.contains(&t.id))
                .for_each(|tile| {
                    tile.concealed = is_concealed;
                    tile.set_id = Some(set_id.clone());
                });

            if is_kong {
                let moved_tile = player_hand
                    .list
                    .iter()
                    .find(|t| t.set_id == Some(set_id.clone()))
                    .unwrap()
                    .clone();

                self.draw_tile_for_player(player_id)
                    .map_err(|_| match self.pass_null_round() {
                        Ok(_) => CreateMeldError::EndRound,
                        Err(_) => CreateMeldError::NotMeld,
                    })?;

                let next_player_hand = self.table.hands.0.get_mut(player_id).unwrap();

                let position = next_player_hand
                    .list
                    .iter()
                    .position(|t| t.id == moved_tile.id)
                    .unwrap();
                next_player_hand.list.remove(position);
                next_player_hand.kong_tiles.insert(KongTile {
                    set_id: set_id.clone(),
                    concealed: is_concealed,
                    id: moved_tile.id,
                });
            }

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

        if hand.kong_tiles.iter().any(|t| t.set_id == set_id.clone()) {
            return Err(BreakMeldError::MeldIsKong);
        }

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

    pub fn update_version(&mut self) {
        self.version = Uuid::new_v4().to_string();
    }

    pub fn update_id(&mut self, id: Option<&str>) {
        self.id = id.map_or_else(
            || Uuid::new_v4().to_string(),
            |inner_id| inner_id.to_string(),
        );
    }

    pub fn complete_players(&mut self, shuffle_players: bool) -> Result<(), &'static str> {
        if self.phase != GamePhase::WaitingPlayers {
            return Err("Game is not waiting for players");
        }

        let players_num = Self::get_players_num(&self.style);

        if self.players.len() != players_num {
            return Err("Not enough players");
        }

        if shuffle_players {
            self.players.shuffle();
        }

        for player_id in self.players.0.clone() {
            self.table.hands.insert(player_id.clone(), Hand::default());
            self.score.insert(player_id, 0);
        }

        self.phase = GamePhase::DecidingDealer;

        Ok(())
    }

    pub fn set_wind_for_player(&mut self, player_id: &PlayerId, wind: &Wind) {
        let current_player_with_wind = self
            .players
            .0
            .iter()
            .find(|p| self.round.get_player_wind(&self.players.0, p) == *wind)
            .unwrap()
            .clone();

        if &current_player_with_wind == player_id {
            return;
        }

        self.players.swap(player_id, &current_player_with_wind);
    }
}
