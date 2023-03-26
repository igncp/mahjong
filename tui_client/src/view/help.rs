use crate::{
    base::{App, Mode},
    ui::{UIScreen, UIState},
};

pub fn get_help_text(app: &App, ui_state: &UIState) -> String {
    let mut help_list = vec!["h: Display or hide this help", "q: Quit (or Ctrl+C)"];

    if ui_state.screen == UIScreen::Game {
        if app.mode == Some(Mode::Admin) {
            help_list.push("- ai: [Admin] continue game until the turn of a non-ai player");
            help_list.push("- cm <player_index> <tile_index>..: [Admin] create meld");
            help_list.push("- di <tile_index>: [Admin] discard tile for the player with 14 tiles");
            help_list.push("- draw: [Admin] draw a tile for the player from the tiles wall");
            help_list.push("- dwi: [Admin] show index of the draw wall tiles");
            help_list.push("- hd: [Admin] show hands");
            help_list.push("- n: [Admin] move to next player, draw and sort");
            help_list.push("- next: [Admin] move to next player");
            help_list.push("- pmd: [Admin] get possible melds by discard");
            help_list.push("- sh: [Admin] sort hands");
            help_list.push("- sm <player_index>: [Admin] say mahjong");
            help_list.push(
                "- sw <index_1> [<index_2>]: [Admin] swap tiles in the draw wall (default to last)",
            );
        }
    } else if app.mode == Some(Mode::Admin) {
        help_list.push("ss: [Admin] start a game");
        help_list.push("gg: [Admin] get games");
    } else {
        help_list.push("gg: Get games for the user");
    }

    help_list.join("\n")
}
