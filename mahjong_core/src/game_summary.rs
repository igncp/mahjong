use crate::{
    deck::DEFAULT_DECK,
    game::{GameStyle, GameVersion, Players},
    meld::{PlayerDiff, PossibleMeld},
    table::BonusTiles,
    Board, Game, GameId, GamePhase, Hand, HandTile, Hands, PlayerId, Score, TileId, Wind,
    WINDS_ROUND_ORDER,
};
use rustc_hash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct RoundSummary {
    consecutive_same_seats: usize,
    pub dealer_player_index: usize,
    east_player_index: usize,
    pub discarded_tile: Option<TileId>,
    pub player_index: usize,
    wind: Wind,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct VisibleMeld {
    set_id: String,
    tiles: Vec<TileId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct OtherPlayerHand {
    pub tiles: usize,
    pub visible: Hand,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HandTileStat {
    in_other_melds: usize,
    in_board: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct OtherPlayerHands(pub FxHashMap<PlayerId, OtherPlayerHand>);

impl OtherPlayerHands {
    pub fn from_hands(hands: &Hands, player_id: &PlayerId) -> Self {
        let mut other_hands = FxHashMap::default();

        for (id, hand) in hands.0.iter() {
            if id != player_id {
                let visible_tiles: Vec<HandTile> =
                    hand.list.iter().filter(|t| !t.concealed).cloned().collect();
                other_hands.insert(
                    id.clone(),
                    OtherPlayerHand {
                        tiles: hand.len(),
                        visible: Hand::new(visible_tiles),
                    },
                );
            }
        }

        Self(other_hands)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GameSummary {
    pub board: Board,
    pub bonus_tiles: BonusTiles,
    pub draw_wall_count: usize,
    pub hand: Option<Hand>,
    pub id: GameId,
    pub other_hands: OtherPlayerHands,
    pub phase: GamePhase,
    pub player_id: PlayerId,
    pub players: Players,
    pub round: RoundSummary,
    pub score: Score,
    pub style: GameStyle,
    pub version: GameVersion,
}

impl GameSummary {
    pub fn from_game(game: &Game, player_id: &PlayerId) -> Option<Self> {
        let discarded_tile = if let Some(tile_claimed) = game.round.tile_claimed.clone() {
            Some(tile_claimed.id)
        } else {
            None
        };

        let round = RoundSummary {
            dealer_player_index: game.round.dealer_player_index,
            east_player_index: game.round.east_player_index,
            discarded_tile,
            consecutive_same_seats: game.round.consecutive_same_seats,
            player_index: game.round.player_index,
            wind: game.round.wind.clone(),
        };

        let draw_wall_count = game.table.draw_wall.len();
        let other_hands = OtherPlayerHands::from_hands(&game.table.hands, player_id);

        Some(Self {
            board: game.table.board.clone(),
            bonus_tiles: game.table.bonus_tiles.clone(),
            draw_wall_count,
            hand: game.table.hands.get(player_id),
            id: game.id.clone(),
            other_hands,
            phase: game.phase,
            player_id: player_id.clone(),
            players: game.players.clone(),
            round,
            score: game.score.clone(),
            style: game.style.clone(),
            version: game.version.clone(),
        })
    }

    pub fn get_current_player(&self) -> &PlayerId {
        &self.players.0[self.round.player_index]
    }

    pub fn get_can_claim_tile(&self) -> bool {
        if self.hand.is_none() {
            return false;
        }

        let tiles_after_claim = self.style.tiles_after_claim();
        self.hand.clone().unwrap().len() < tiles_after_claim
            && self
                .other_hands
                .0
                .iter()
                .all(|(_, hand)| hand.tiles < tiles_after_claim)
            && self.round.discarded_tile.is_some()
    }

    pub fn get_can_pass_turn(&self) -> bool {
        self.phase == GamePhase::Playing
            && self.hand.is_some()
            && self.hand.as_ref().unwrap().len() == self.style.tiles_after_claim() - 1
            && self.get_current_player() == &self.player_id
    }

    pub fn get_can_discard_tile(&self) -> bool {
        self.hand.is_some() && self.hand.clone().unwrap().len() == self.style.tiles_after_claim()
    }

    pub fn get_possible_melds(&self) -> Vec<PossibleMeld> {
        let tested_hand = self.hand.clone();
        if tested_hand.is_none() {
            return vec![];
        }

        let mut tested_hand = tested_hand.unwrap();

        let mut possible_melds: Vec<PossibleMeld> = vec![];
        let can_claim_tile = self.get_can_claim_tile();

        let claimed_tile: Option<TileId> = self.round.discarded_tile;
        let mut player_diff: PlayerDiff = None;
        let player_index = self
            .players
            .iter()
            .position(|p| p == &self.player_id)
            .unwrap();
        let current_player_index = self.round.player_index;

        if can_claim_tile {
            let tile = HandTile {
                concealed: true,
                id: claimed_tile.unwrap(),
                set_id: None,
            };

            tested_hand.push(tile);
            player_diff = Some(match player_index as i32 - current_player_index as i32 {
                -3 => 1,
                val => val,
            });
        }

        let mut raw_melds = tested_hand.get_possible_melds(player_diff, claimed_tile, true);

        tested_hand
            .get_possible_melds(player_diff, claimed_tile, false)
            .iter()
            .for_each(|m| {
                raw_melds.push(m.clone());
            });

        for raw_meld in raw_melds {
            let possible_meld = PossibleMeld {
                discard_tile: None,
                is_concealed: raw_meld.is_concealed,
                is_mahjong: raw_meld.is_mahjong,
                is_upgrade: raw_meld.is_upgrade,
                player_id: self.player_id.clone(),
                tiles: raw_meld.tiles.clone(),
            };

            possible_melds.push(possible_meld);
        }

        possible_melds
    }

    pub fn get_players_winds(&self) -> FxHashMap<PlayerId, Wind> {
        let mut winds = FxHashMap::default();

        let east_index = WINDS_ROUND_ORDER
            .iter()
            .position(|w| w == &Wind::East)
            .unwrap();

        for (index, player_id) in self.players.iter().enumerate() {
            let wind_index = (east_index + index) % WINDS_ROUND_ORDER.len();
            let wind = WINDS_ROUND_ORDER[wind_index].clone();
            winds.insert(player_id.clone(), wind);
        }

        winds
    }

    pub fn get_players_visible_melds(&self) -> FxHashMap<PlayerId, Vec<VisibleMeld>> {
        let mut visible_melds_set = FxHashMap::default();

        fn get_visible_melds(player_hand: &Hand) -> Vec<VisibleMeld> {
            let mut visible_melds = vec![];
            let player_melds = player_hand
                .list
                .iter()
                .filter(|t| !t.concealed)
                .filter_map(|t| t.set_id.clone())
                .collect::<FxHashSet<_>>();

            for meld_id in player_melds {
                let mut tiles = player_hand
                    .list
                    .iter()
                    .filter(|t| t.set_id.as_ref() == Some(&meld_id))
                    .map(|t| t.id)
                    .collect::<Vec<_>>();

                let kong_tile = player_hand
                    .kong_tiles
                    .iter()
                    .find(|t| t.set_id.as_ref() == meld_id);

                if let Some(kong_tile) = kong_tile {
                    tiles.push(kong_tile.id);
                }

                visible_melds.push(VisibleMeld {
                    set_id: meld_id.clone(),
                    tiles,
                })
            }
            visible_melds
        }

        if self.hand.is_none() {
            return visible_melds_set;
        }

        visible_melds_set.insert(
            self.player_id.clone(),
            get_visible_melds(&self.hand.clone().unwrap()),
        );

        for (player_id, other_player_hand) in self.other_hands.0.iter() {
            let hand = &other_player_hand.visible;

            visible_melds_set.insert(player_id.clone(), get_visible_melds(hand));
        }

        visible_melds_set
    }

    pub fn get_can_pass_round(&self) -> bool {
        let tiles_after_claim = self.style.tiles_after_claim();

        self.phase == GamePhase::Playing
            && self.hand.is_some()
            && self.draw_wall_count == 0
            && self.hand.as_ref().unwrap().len() < tiles_after_claim
            && self
                .other_hands
                .0
                .iter()
                .all(|(_, hand)| hand.tiles < tiles_after_claim)
    }

    pub fn get_can_draw_tile(&self) -> bool {
        self.phase == GamePhase::Playing
            && self.hand.is_some()
            && self.hand.as_ref().unwrap().len() < self.style.tiles_after_claim()
            && self.draw_wall_count > 0
            && self.get_current_player() == &self.player_id
    }

    pub fn get_can_say_mahjong(&self) -> bool {
        self.phase == GamePhase::Playing
            && self.hand.is_some()
            && self.hand.as_ref().unwrap().can_say_mahjong().is_ok()
    }

    pub fn get_hand_stats(&self) -> FxHashMap<TileId, HandTileStat> {
        let mut hand_stats = FxHashMap::default();

        if self.hand.is_none() {
            return hand_stats;
        }

        let hand = self.hand.as_ref().unwrap();

        let mut own_meld_tiles = hand
            .list
            .iter()
            .filter(|t| t.set_id.is_some())
            .map(|t| t.id)
            .collect::<Vec<_>>();

        for kong_tile in hand.kong_tiles.iter() {
            own_meld_tiles.push(kong_tile.id);
        }

        for hand_tile in hand.list.iter() {
            if hand_tile.set_id.is_some() {
                continue;
            }

            let mut stat = HandTileStat {
                in_other_melds: 0,
                in_board: 0,
            };

            let hand_tile_full = &DEFAULT_DECK.0[hand_tile.id];

            for own_meld_tile in own_meld_tiles.iter() {
                let tile = &DEFAULT_DECK.0[*own_meld_tile];

                if tile.is_same_content(hand_tile_full) {
                    stat.in_other_melds += 1;
                }
            }

            for (_, other_hand) in self.other_hands.0.iter() {
                other_hand.visible.list.iter().for_each(|t| {
                    let tile = &DEFAULT_DECK.0[t.id];

                    if tile.is_same_content(hand_tile_full) {
                        stat.in_other_melds += 1;
                    }
                })
            }

            for board_tile in self.board.0.iter() {
                let tile = &DEFAULT_DECK.0[*board_tile];

                if tile.is_same_content(hand_tile_full) {
                    stat.in_board += 1;
                }
            }

            hand_stats.insert(hand_tile.id, stat);
        }

        hand_stats
    }
}
