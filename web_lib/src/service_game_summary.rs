use mahjong_core::{
    game_summary::{HandTileStat, VisibleMeld},
    meld::PossibleMeld,
    PlayerId, TileId, Wind,
};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use service_contracts::{ServiceGameSummary, ServicePlayerSummary};
use ts_rs::TS;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(TS, Serialize, Deserialize)]
#[ts(export)]
struct PlayingExtras {
    can_claim_tile: bool,
    can_discard_tile: bool,
    can_draw_tile: bool,
    can_pass_round: bool,
    can_pass_turn: bool,
    can_say_mahjong: bool,
    dealer_player: Option<ServicePlayerSummary>,
    hand_stats: FxHashMap<TileId, HandTileStat>,
    players_visible_melds: FxHashMap<PlayerId, Vec<VisibleMeld>>,
    players_winds: FxHashMap<PlayerId, Wind>,
    playing_player: Option<ServicePlayerSummary>,
    possible_melds: Vec<PossibleMeld>,
    turn_player: Option<ServicePlayerSummary>,
}

#[derive(TS, Serialize, Deserialize)]
#[ts(export)]
struct LibGetGamePlayingExtrasParam(ServiceGameSummary);

#[derive(TS, Serialize, Deserialize)]
#[ts(export)]
struct LibGetGamePlayingExtrasReturn(PlayingExtras);

#[wasm_bindgen]
pub fn get_game_playing_extras(param: JsValue) -> JsValue {
    let parsed_val: LibGetGamePlayingExtrasParam = serde_wasm_bindgen::from_value(param).unwrap();

    let can_draw_tile = parsed_val.0.game_summary.get_can_draw_tile();
    let can_say_mahjong = parsed_val.0.game_summary.get_can_say_mahjong();
    let can_pass_round = parsed_val.0.game_summary.get_can_pass_round();
    let can_claim_tile = parsed_val.0.game_summary.get_can_claim_tile();
    let can_pass_turn = parsed_val.0.game_summary.get_can_pass_turn();
    let can_discard_tile = parsed_val.0.game_summary.get_can_discard_tile();

    let dealer_player = parsed_val.0.get_dealer_player();
    let possible_melds = parsed_val.0.game_summary.get_possible_melds();
    let players_visible_melds = parsed_val.0.game_summary.get_players_visible_melds();
    let players_winds = parsed_val.0.game_summary.get_players_winds();
    let playing_player = parsed_val
        .0
        .players
        .get(&parsed_val.0.game_summary.player_id)
        .cloned();
    let turn_player = parsed_val.0.get_turn_player();
    let hand_stats = parsed_val.0.game_summary.get_hand_stats();

    let rv = LibGetGamePlayingExtrasReturn(PlayingExtras {
        can_claim_tile,
        can_discard_tile,
        can_draw_tile,
        can_pass_round,
        can_pass_turn,
        can_say_mahjong,
        dealer_player,
        hand_stats,
        players_visible_melds,
        players_winds,
        playing_player,
        possible_melds,
        turn_player,
    });

    serde_wasm_bindgen::to_value(&rv).unwrap()
}
