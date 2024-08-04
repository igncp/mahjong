use mahjong_core::{
    deck::DEFAULT_DECK,
    meld::{get_is_chow, get_is_kong, get_is_pung, PossibleMeld, SetCheckOpts},
    TileId,
};
use serde::{Deserialize, Serialize};
use service_contracts::ServiceGame;
use ts_rs::TS;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(TS, Serialize, Deserialize)]
#[ts(export)]
struct LibGetIsMeldParam(Vec<TileId>);

macro_rules! get_meld_opts {
    ($val:expr) => {{
        let sub_hand: LibGetIsMeldParam = serde_wasm_bindgen::from_value($val).unwrap();

        SetCheckOpts {
            board_tile_player_diff: None,
            claimed_tile: None,
            sub_hand: &sub_hand
                .0
                .iter()
                .map(|id| &DEFAULT_DECK.0[*id])
                .collect::<Vec<_>>(),
        }
    }};
}

#[wasm_bindgen]
pub fn is_pung(val: JsValue) -> bool {
    let opts = get_meld_opts!(val);

    get_is_pung(&opts)
}

#[wasm_bindgen]
pub fn is_chow(val: JsValue) -> bool {
    let opts = get_meld_opts!(val);

    get_is_chow(&opts)
}

#[wasm_bindgen]
pub fn is_kong(val: JsValue) -> bool {
    let opts = get_meld_opts!(val);

    get_is_kong(&opts)
}

#[derive(TS, Serialize, Deserialize)]
#[ts(export)]
struct LibGetPossibleMeldsParam(ServiceGame);

#[derive(TS, Serialize, Deserialize)]
#[ts(export)]
struct LibGetPossibleMeldsReturn(Vec<PossibleMeld>);

#[wasm_bindgen]
pub fn get_possible_melds(game: JsValue) -> JsValue {
    let service_game: LibGetPossibleMeldsParam = serde_wasm_bindgen::from_value(game).unwrap();
    let possible_melds = service_game.0.game.get_possible_melds(false);

    serde_wasm_bindgen::to_value(&LibGetPossibleMeldsReturn(possible_melds)).unwrap()
}
