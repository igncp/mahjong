use mahjong_core::{ui::format_to_emoji, Tile};
use service_contracts::ServiceGame;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn format_tile(tile: JsValue) -> String {
    let tile: Tile = serde_wasm_bindgen::from_value(tile).unwrap();

    format_to_emoji(&tile)
}

#[wasm_bindgen]
pub fn get_possible_melds(game: String) -> JsValue {
    let service_game: ServiceGame = serde_json::from_str(&game).unwrap();
    let possible_melds = service_game.game.get_possible_melds();

    serde_wasm_bindgen::to_value(&possible_melds).unwrap()
}
