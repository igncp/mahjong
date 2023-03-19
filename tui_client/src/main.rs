use base::{App, Mode};
use clap::{arg, command, value_parser};
use std::path::PathBuf;
use ui::UI;

mod base;
mod formatter;
mod log;
mod service_http_client;
mod ui;

#[tokio::main]
async fn main() {
    let matches = command!()
        .arg(
            arg!(--"game-id" <GAME_ID> "Loads a game id")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(--"mode" <GAME_ID> "Plays as an admin or as a user")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let mut app = App::new().await;

    if let Some(game_id) = matches.get_one::<PathBuf>("game-id") {
        let game_id_str = game_id.to_str().unwrap();
        let game = app.admin_load_game(game_id_str).await;

        if game.is_err() {
            println!("{}", game.err().unwrap());
            println!("Exiting");
            std::process::exit(1);
        }
    }

    if let Some(game_mode) = matches.get_one::<PathBuf>("mode") {
        let game_mode = game_mode.to_str().unwrap();
        if game_mode == "admin" {
            app.mode = Some(Mode::Admin);
        } else if game_mode == "user" {
            app.mode = Some(Mode::User);
        }
    } else {
        app.mode = Some(Mode::Admin);
    }

    let mut ui = UI::new();

    ui.run(&mut app).await;
}
