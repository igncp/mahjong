use serde::{Deserialize, Serialize};

use crate::{
    meld::{
        get_board_tile_player_diff, get_is_pair, get_possible_melds, get_tile_claimed_id_for_user,
        GetBoardTilePlayerDiff, GetPossibleMelds,
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

pub struct PossibleMeld {
    pub player_id: PlayerId,
    pub tiles: Vec<TileId>,
    pub discard_tile: Option<TileId>,
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

            let opts = GetBoardTilePlayerDiff {
                hand: &player_hand,
                players: &self.players,
                player_id: &player.id,
                round: &round,
            };

            let board_tile_player_diff = get_board_tile_player_diff(&opts);
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
            let game_copy = self.clone();

            let mut opts = DiscardTileToBoardOpts {
                board: game_copy.table.board.clone(),
                hands: game_copy.table.hands.clone(),
                player_id: player_id.clone(),
                tile_id: hand_tile.id,
                round: game_copy.round.clone(),
            };

            discard_tile_to_board(&mut opts);

            let new_melds = game_copy.get_possible_melds();

            new_melds
                .iter()
                .filter(|m| m.player_id != player_id)
                .for_each(|meld| {
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
}

pub struct DiscardTileToBoardOpts {
    pub board: Board,
    pub hands: Hands,
    pub player_id: PlayerId,
    pub tile_id: TileId,
    pub round: Round,
}

pub fn discard_tile_to_board(opts: &mut DiscardTileToBoardOpts) -> Option<TileId> {
    let player_hand = opts.hands.get_mut(&opts.player_id).unwrap();

    if player_hand.0.len() != 14 {
        return None;
    }

    let tile_index = player_hand.0.iter().position(|t| t.id == opts.tile_id);

    tile_index?;

    let tile_index = tile_index.unwrap();
    let tile = player_hand.0.get(tile_index).unwrap().clone();

    if !tile.concealed {
        return None;
    }

    if opts.round.tile_claimed.is_some() {
        let tile_claimed = opts.round.tile_claimed.clone().unwrap();
        if tile_claimed.by.is_some()
            && tile_claimed.by.unwrap() == opts.player_id
            && tile.id != tile_claimed.id
            && player_hand
                .0
                .iter()
                .find(|t| t.id == tile_claimed.id)
                .unwrap()
                .set_id
                .is_none()
        {
            return None;
        }
    }

    player_hand.0.remove(tile_index);

    opts.board.push(tile.id);

    opts.round.tile_claimed = Some(RoundTileClaimed {
        from: opts.player_id.clone(),
        id: tile.id,
        by: None,
    });

    Some(tile.id)
}
