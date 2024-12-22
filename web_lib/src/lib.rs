#![deny(clippy::use_self, clippy::shadow_unrelated)]
use mahjong_core::{deck::DEFAULT_DECK, ui::format_to_emoji, Tile};
pub use melds::{get_possible_melds, is_chow, is_kong, is_pung};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

mod melds;
mod offscreen_game;
mod service_game_summary;

#[wasm_bindgen]
pub fn format_tile(tile: JsValue) -> String {
    let tile: Tile = serde_wasm_bindgen::from_value(tile).unwrap();

    format_to_emoji(&tile)
}

#[wasm_bindgen]
pub fn get_deck() -> JsValue {
    let deck = DEFAULT_DECK.clone();
    serde_wasm_bindgen::to_value(&deck).unwrap()
}
