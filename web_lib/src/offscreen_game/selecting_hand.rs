use std::collections::HashSet;

use mahjong_core::TileId;
use wasm_bindgen::prelude::wasm_bindgen;

use super::OffscreenGame;

#[derive(Clone)]
#[wasm_bindgen]
pub struct SelectingHandTile {
    pub(super) concealed: bool,
    pub(super) id: TileId,
    pub(super) set_id: Option<String>,
}

#[wasm_bindgen]
impl SelectingHandTile {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> TileId {
        self.id
    }

    #[wasm_bindgen(getter)]
    pub fn concealed(&self) -> bool {
        self.concealed
    }

    #[wasm_bindgen(getter)]
    pub fn set_id(&self) -> Option<String> {
        self.set_id.clone()
    }

    pub fn set_concealed(&mut self, concealed: bool, off_game: &mut OffscreenGame) {
        off_game
            .game
            .table
            .hands
            .0
            .iter_mut()
            .for_each(|(_, hand)| {
                hand.list.iter_mut().for_each(|t| {
                    if t.id == self.id {
                        t.concealed = concealed;
                    }
                });
            });
    }

    pub fn set_set_id(&mut self, set_id: Option<String>, off_game: &mut OffscreenGame) {
        off_game
            .game
            .table
            .hands
            .0
            .iter_mut()
            .for_each(|(_, hand)| {
                hand.list.iter_mut().for_each(|t| {
                    if t.id == self.id {
                        t.set_id = set_id.clone();
                    }
                });
            });
    }
}

#[wasm_bindgen]
pub struct SelectingHand {
    pub(super) tiles: Vec<SelectingHandTile>,
}

#[wasm_bindgen]
impl SelectingHand {
    #[wasm_bindgen(getter)]
    pub fn tiles(&self) -> Vec<SelectingHandTile> {
        self.tiles.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn sets_ids(&self) -> Vec<String> {
        let ids: HashSet<String> = self.tiles.iter().filter_map(|t| t.set_id.clone()).collect();

        ids.into_iter().collect()
    }
}
