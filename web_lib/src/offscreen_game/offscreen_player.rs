use mahjong_core::PlayerId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Default, Serialize, Deserialize, Clone)]
#[wasm_bindgen]
pub struct OffscreenPlayer {
    pub(super) id: PlayerId,
    #[wasm_bindgen(getter_with_clone)]
    pub name: String,
}

#[wasm_bindgen]
impl OffscreenPlayer {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }
}

pub type OffscreenPlayers = HashMap<PlayerId, OffscreenPlayer>;
