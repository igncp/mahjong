use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    meld::{
        get_is_chow, get_is_kong, get_is_pair, get_is_pung, get_possible_melds,
        get_tile_claimed_id_for_user, GetPossibleMelds, PlayerDiff, PossibleMeld, SetCheckOpts,
    },
    Board, Deck, Hand, HandTile, Hands, Player, PlayerId, Round, RoundTileClaimed, Score, Table,
    TileId,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum GamePhase {
    Beginning,
    End,
    Playing,
}

pub type GameId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    pub deck: Deck,
    pub id: GameId,
    pub name: String,
    pub phase: GamePhase,
    pub players: Vec<Player>,
    pub round: Round,
    pub score: Score,
    pub table: Table,
}

impl Default for Game {
    fn default() -> Self {
        let deck = Deck::default();
        let mut players = vec![];

        for player_id in 0..4 {
            players.push(Player {
                id: player_id.to_string(),
                name: format!("Player {}", player_id),
            });
        }

        let table = deck.create_table(&players);
        let mut score = Score::default();

        for player in &players {
            score.insert(player.id.clone(), 0);
        }

        Game {
            deck,
            id: "game_id".to_string(),
            name: "game_name".to_string(),
            phase: GamePhase::Beginning,
            players,
            round: Round::default(),
            score,
            table,
        }
    }
}

pub struct ClaimTile {
    board: Board,
    hands: Hands,
    player_id: PlayerId,
    players: Vec<Player>,
    round: Round,
}

pub fn claim_tile(opts: &mut ClaimTile) -> bool {
    let player_hand = opts.hands.get_mut(&opts.player_id);
    if player_hand.is_none() {
        return false;
    }
    let player_hand = player_hand.unwrap();

    if player_hand.0.len() != 13 || opts.round.tile_claimed.is_none() || opts.board.is_empty() {
        return false;
    }

    let tile = opts.board.pop().unwrap();

    let mut tile_claimed = opts.round.tile_claimed.clone().unwrap();
    tile_claimed.by = Some(opts.player_id.clone());

    opts.round.tile_claimed = Some(tile_claimed);
    opts.round.player_index = opts
        .players
        .iter()
        .position(|p| p.id == opts.player_id)
        .unwrap();

    player_hand.0.push(HandTile {
        concealed: true,
        id: tile,
        set_id: None,
    });

    true
}

impl Game {
    pub fn set_players(&mut self, players: &Vec<Player>) {
        if players.len() != self.players.len() {
            return;
        }

        let current_players = self.players.clone();

        self.players = players.clone();

        for (index, player) in self.players.iter().enumerate() {
            let current_player = current_players.get(index).unwrap();
            let player_hand = self.table.hands.remove(&current_player.id).unwrap();
            let score = self.score.remove(&current_player.id).unwrap();

            self.table.hands.insert(player.id.clone(), player_hand);
            self.score.insert(player.id.clone(), score);
        }
    }

    pub fn say_mahjong(&mut self, player_id: PlayerId) -> bool {
        if !self.can_say_mahjong(self.table.hands.get(&player_id).unwrap()) {
            return false;
        }

        self.calculate_hand_score(player_id);

        self.round.move_after_win(&mut self.phase);

        self.table = self.deck.create_table(&self.players);

        true
    }

    pub fn start_game(&mut self) {
        self.phase = GamePhase::Playing;
    }

    pub fn can_say_mahjong(&self, player_hand: &Hand) -> bool {
        if player_hand.0.len() != 14 {
            return false;
        }

        let tiles_without_meld = player_hand
            .0
            .iter()
            .filter(|t| t.set_id.is_none())
            .map(|t| self.deck.0.get(&t.id).unwrap())
            .collect();

        get_is_pair(&tiles_without_meld)
    }

    pub fn get_possible_melds(&self) -> Vec<PossibleMeld> {
        let mut melds: Vec<PossibleMeld> = vec![];

        for player in &self.players {
            let tile_claimed = &self.round.tile_claimed.clone();
            let player_hand = self.table.hands.get(&player.id).unwrap().clone();
            let can_claim_tile = tile_claimed.is_some()
                && tile_claimed.clone().unwrap().by.is_none()
                && tile_claimed.clone().unwrap().from != player.id
                && player_hand.0.len() == 13;

            let hand = if can_claim_tile {
                let mut hand = player_hand.clone();
                hand.0.push(HandTile {
                    concealed: true,
                    id: tile_claimed.clone().unwrap().id,
                    set_id: None,
                });
                hand
            } else {
                player_hand.clone()
            };

            let mut round = self.round.clone();
            if can_claim_tile {
                round.tile_claimed = Some(RoundTileClaimed {
                    by: Some(player.id.clone()),
                    from: tile_claimed.clone().unwrap().from,
                    id: tile_claimed.clone().unwrap().id,
                });
            }

            let board_tile_player_diff =
                self.get_board_tile_player_diff(Some(&round), Some(&hand), &player.id);
            let claimed_tile = get_tile_claimed_id_for_user(&player.id, &round.tile_claimed);

            let opts = GetPossibleMelds {
                board_tile_player_diff,
                claimed_tile,
                deck: &self.deck,
                hand: &hand,
            };

            let possible_melds = get_possible_melds(&opts);

            if self.can_say_mahjong(&hand) {
                melds.push(PossibleMeld {
                    discard_tile: None,
                    player_id: player.id.clone(),
                    tiles: hand
                        .0
                        .iter()
                        .filter(|t| t.set_id.is_none())
                        .map(|t| t.id)
                        .collect(),
                });
            }

            for meld in possible_melds {
                melds.push(PossibleMeld {
                    discard_tile: None,
                    player_id: player.id.clone(),
                    tiles: meld,
                });
            }
        }

        melds
    }

    pub fn get_possible_melds_by_discard(&self) -> Vec<PossibleMeld> {
        let mut melds = self.get_possible_melds();

        let player_index = self.players.iter().position(|p| {
            self.table
                .hands
                .get(&p.id)
                .unwrap()
                .0
                .iter()
                .filter(|t| t.set_id.is_none())
                .count()
                == 14
        });

        if player_index.is_none() {
            return melds;
        }

        let player_index = player_index.unwrap();
        let player_id = self.players.get(player_index).unwrap().id.clone();

        let player_hand: Vec<HandTile> = self
            .table
            .hands
            .get(&player_id)
            .unwrap()
            .clone()
            .0
            .into_iter()
            .filter(|t| t.set_id.is_none())
            .collect();

        player_hand.iter().for_each(|hand_tile| {
            let mut game_copy = self.clone();

            game_copy.discard_tile_to_board(&hand_tile.id);

            let new_melds = game_copy.get_possible_melds();

            new_melds
                .iter()
                .filter(|m| m.player_id != player_id)
                .for_each(|meld| {
                    if !meld.tiles.iter().any(|t| t == &hand_tile.id) {
                        return;
                    }

                    melds.push(PossibleMeld {
                        discard_tile: Some(hand_tile.id),
                        player_id: meld.player_id.clone(),
                        tiles: meld.tiles.clone(),
                    });
                });
        });

        melds
    }

    pub fn get_current_player(&self) -> &Player {
        &self.players[self.round.player_index]
    }

    pub fn draw_tile_from_wall(&mut self) -> Option<TileId> {
        if self.table.draw_wall.is_empty() || self.round.wall_tile_drawn.is_some() {
            return None;
        }

        let tile_id = self.table.draw_wall.pop().unwrap();

        let wall_tile_drawn = Some(tile_id);
        self.round.wall_tile_drawn = wall_tile_drawn;
        let player_id = self.get_current_player().id.clone();

        let hand = self.table.hands.get_mut(&player_id).unwrap();
        hand.0.push(HandTile::from_id(tile_id));

        wall_tile_drawn
    }

    pub fn discard_tile_to_board(&mut self, tile_id: &TileId) -> bool {
        let player_with_14_tiles = self
            .players
            .iter()
            .find(|p| self.table.hands.get(&p.id).unwrap().0.len() == 14);

        if player_with_14_tiles.is_none() {
            return false;
        }

        let player_id = player_with_14_tiles.unwrap().id.clone();
        let player_hand = self.table.hands.get_mut(&player_id).unwrap();
        let tile_index = player_hand.0.iter().position(|t| &t.id == tile_id);

        if tile_index.is_none() {
            return false;
        }

        let tile_index = tile_index.unwrap();
        let tile = player_hand.0.get(tile_index).unwrap().clone();

        if !tile.concealed || tile.set_id.is_some() {
            return false;
        }

        if self.round.tile_claimed.is_some() {
            let tile_claimed = self.round.tile_claimed.clone().unwrap();
            if tile_claimed.by.is_some()
                && tile_claimed.by.unwrap() == player_id
                && tile.id != tile_claimed.id
                && player_hand
                    .0
                    .iter()
                    .find(|t| t.id == tile_claimed.id)
                    .unwrap()
                    .set_id
                    .is_none()
            {
                return false;
            }
        }

        player_hand.0.remove(tile_index);

        self.table.board.push(tile.id);

        self.round.tile_claimed = Some(RoundTileClaimed {
            from: player_id.clone(),
            id: tile.id,
            by: None,
        });

        true
    }

    pub fn create_meld(&mut self, player_id: &PlayerId, tiles: &HashSet<TileId>) -> bool {
        let hand = self.table.hands.get(player_id).unwrap();
        let sub_hand_tiles = hand
            .0
            .iter()
            .filter(|t| tiles.contains(&t.id))
            .cloned()
            .collect::<Vec<HandTile>>();

        if sub_hand_tiles
            .iter()
            .any(|t| t.set_id.is_some() || !t.concealed)
        {
            return false;
        }

        let sub_hand = Hand(sub_hand_tiles);

        let board_tile_player_diff =
            self.get_board_tile_player_diff(None, Some(&sub_hand), player_id);

        let tiles_ids = tiles.iter().cloned().collect::<Vec<TileId>>();

        let opts_claimed_tile = get_tile_claimed_id_for_user(player_id, &self.round.tile_claimed);

        let opts = SetCheckOpts {
            board_tile_player_diff,
            claimed_tile: opts_claimed_tile,
            deck: &self.deck,
            sub_hand: &tiles_ids,
        };

        if get_is_pung(&opts) || get_is_chow(&opts) || get_is_kong(&opts) {
            let set_id = Uuid::new_v4().to_string();
            let concealed = board_tile_player_diff.is_none();
            let hand = self.table.hands.get_mut(player_id).unwrap();

            hand.0
                .iter_mut()
                .filter(|t| tiles.contains(&t.id))
                .for_each(|tile| {
                    tile.concealed = concealed;
                    tile.set_id = Some(set_id.clone());
                });

            return true;
        }

        false
    }

    pub fn get_board_tile_player_diff(
        &self,
        round: Option<&Round>,
        hand: Option<&Hand>,
        player_id: &PlayerId,
    ) -> PlayerDiff {
        let round = round.unwrap_or(&self.round);
        let hand = hand.unwrap_or(self.table.hands.get(player_id).unwrap());
        let tile_claimed = round.tile_claimed.clone();

        if let Some(tile_claimed) = tile_claimed {
            tile_claimed.by?;

            if !hand.0.iter().any(|h| h.id == tile_claimed.id) {
                return None;
            }

            let player_index = self.players.iter().position(|p| &p.id == player_id);
            let other_player_index = self.players.iter().position(|p| p.id == tile_claimed.from);

            if player_index.is_none() || other_player_index.is_none() {
                return None;
            }

            let player_index = player_index.unwrap();
            let other_player_index = other_player_index.unwrap();

            return Some(player_index as i32 - other_player_index as i32);
        }

        None
    }
}
