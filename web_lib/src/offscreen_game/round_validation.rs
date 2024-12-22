use mahjong_core::PlayerId;
use serde::Serialize;
use ts_rs::TS;
use wasm_bindgen::prelude::wasm_bindgen;

use super::OffscreenGame;

#[derive(Clone, Serialize, TS)]
#[ts(export)]
pub enum RoundValidationError {
    NoHandMahjong,
}

#[wasm_bindgen]
pub struct IsValidRoundResult {
    pub is_valid: bool,
    error_message: Option<RoundValidationError>,
    #[wasm_bindgen(getter_with_clone)]
    pub winner_player: Option<PlayerId>,
}

#[wasm_bindgen]
impl IsValidRoundResult {
    pub fn error_message_data(&self) -> String {
        serde_json::to_string(&self.error_message).unwrap()
    }
}

#[wasm_bindgen]
impl OffscreenGame {
    pub fn get_is_valid_round(&self) -> IsValidRoundResult {
        let mut hand_with_mahjong: Option<PlayerId> = None;

        for player in self.game.players.iter() {
            let hand = self.game.table.hands.get(player).unwrap();

            if hand.can_say_mahjong().map_or(false, |_| true) {
                hand_with_mahjong = Some(player.clone());
                break;
            }
        }

        if hand_with_mahjong.is_some() {
            return IsValidRoundResult {
                error_message: None,
                is_valid: true,
                winner_player: hand_with_mahjong,
            };
        }

        IsValidRoundResult {
            error_message: Some(RoundValidationError::NoHandMahjong),
            is_valid: false,
            winner_player: None,
        }
    }
}
