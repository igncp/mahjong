use js_sys::Date;
use mahjong_core::{
    deck::DEFAULT_DECK, score::ScoringRule, Game, Hand, HandTile, PlayerId, Players, TileId,
};
use offscreen_player::{OffscreenPlayer, OffscreenPlayers};
use selecting_hand::{SelectingHand, SelectingHandTile};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
pub use wrappers::{ScoringRuleWasm, WindWasm};

mod offscreen_player;
mod round_validation;
mod selecting_hand;
mod wrappers;

#[wasm_bindgen]
pub struct ScoreResult {
    rules: Vec<ScoringRule>,
    pub score: u32,
}

#[wasm_bindgen]
impl ScoreResult {
    #[wasm_bindgen(getter)]
    pub fn rules(&self) -> Vec<ScoringRuleWasm> {
        self.rules.iter().map(|r| r.clone().into()).collect()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct OffscreenGame {
    pub date_created: u64,
    game: Game,
    players: OffscreenPlayers,
}

#[wasm_bindgen]
impl OffscreenGame {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut game = Game::new(None);
        game.start(false);

        let players: OffscreenPlayers = [1, 2, 3, 4]
            .iter()
            .map(|num| {
                let mut player = OffscreenPlayer {
                    id: Players::new_player(),
                    ..Default::default()
                };
                player.name = format!("Player {}", num);
                (player.id.clone(), player)
            })
            .collect();

        players.iter().for_each(|(player_id, _)| {
            game.players.push(player_id.clone());
        });

        game.complete_players(false).unwrap();
        let date_created = Date::now() as u64;

        Self {
            date_created,
            game,
            players,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn players(&self) -> Vec<OffscreenPlayer> {
        self.game
            .players
            .0
            .iter()
            .map(|p| self.players.get(p).unwrap().clone())
            .collect()
    }

    pub fn set_player(&mut self, id: PlayerId, player: OffscreenPlayer) {
        if self.players.contains_key(&id) {
            self.players.insert(id, player);
        }
    }

    pub fn update_player_score(&mut self, player_id: PlayerId, score: u32) {
        self.game.score.0.insert(player_id, score);
    }

    pub fn get_player_score(&self, player_id: PlayerId) -> u32 {
        if !self.game.score.0.contains_key(&player_id) {
            return 0;
        }
        *self.game.score.0.get(&player_id).unwrap()
    }

    pub fn is_dealer(&self, player_id: PlayerId) -> bool {
        Some(self.game.round.dealer_player_index)
            == self.game.players.0.iter().position(|p| p == &player_id)
    }

    pub fn get_wind(&self, player_id: PlayerId) -> WindWasm {
        let wind = self
            .game
            .round
            .get_player_wind(&self.game.players.0, &player_id);

        wind.into()
    }

    pub fn set_dealer(&mut self, player_id: PlayerId) {
        if let Some(index) = self.game.players.0.iter().position(|p| p == &player_id) {
            self.game.round.dealer_player_index = index;
        }
    }

    #[wasm_bindgen(getter)]
    pub fn available_tiles_for_round(&self) -> Vec<TileId> {
        let tiles_ids: Vec<TileId> = DEFAULT_DECK
            .0
            .iter()
            .enumerate()
            .map(|(i, _)| i as TileId)
            .filter(|i| {
                !self.game.players.0.iter().any(|p| {
                    self.game
                        .table
                        .hands
                        .get(p)
                        .unwrap()
                        .list
                        .iter()
                        .any(|t| t.id == *i)
                })
            })
            .collect();
        let hand_list = tiles_ids
            .iter()
            .map(|id| HandTile {
                concealed: true,
                id: *id,
                set_id: None,
            })
            .collect();
        let mut hand = Hand::new(hand_list);
        hand.sort_default();
        hand.list.iter().map(|t| t.id).collect()
    }

    pub fn selecting_hand(&self, player_id: PlayerId) -> SelectingHand {
        let mut hand = self.game.table.hands.get(&player_id).unwrap().clone();
        hand.sort_default();
        let tiles = hand
            .list
            .iter()
            .map(|t| SelectingHandTile {
                concealed: t.concealed,
                id: t.id,
                set_id: t.set_id.clone(),
            })
            .collect::<Vec<SelectingHandTile>>();

        SelectingHand { tiles }
    }

    pub fn select_tile_for_round(&mut self, player_id: PlayerId, tile_id: TileId) {
        let hand = self.game.table.hands.0.get_mut(&player_id).unwrap();
        let has_tile = hand.list.iter().any(|t| t.id == tile_id);

        if has_tile {
            hand.list.retain(|t| t.id != tile_id);
        } else {
            let hand_tile = HandTile {
                concealed: true,
                id: tile_id,
                set_id: None,
            };

            hand.list.push(hand_tile);
        }
    }

    pub fn update_score(&mut self, player_id: PlayerId) -> ScoreResult {
        let (rules, score) = self.game.calculate_hand_score(&player_id);

        ScoreResult { rules, score }
    }

    pub fn set_wind(&mut self, player_id: PlayerId, wind: WindWasm) {
        self.game.set_wind_for_player(&player_id, &wind.into());
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn deserialize(data: String) -> Self {
        serde_json::from_str(&data).unwrap()
    }
}
