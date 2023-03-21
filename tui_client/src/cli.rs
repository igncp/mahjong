use crate::base::{App, Mode};
use clap::{arg, command, value_parser};
use std::path::PathBuf;

pub async fn parse_args(app: &mut App) {
    let matches = command!()
        .arg(
            arg!(--"game-id" <GAME_ID> "Loads a game id")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(--"mode" <MODE> "Plays as an 'admin' or as a 'user'. It defaults to 'admin' unless the 'id' option is passed")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(--"id" <MODE> "User id (only for 'user' mode))")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    app.user_id = match matches.get_one::<PathBuf>("id") {
        Some(user_id) => {
            let user_id_str = user_id.to_str().unwrap();
            Some(user_id_str.to_string())
        }
        None => None,
    };

    if let Some(game_mode) = matches.get_one::<PathBuf>("mode") {
        let game_mode = game_mode.to_str().unwrap();
        if game_mode == "admin" {
            app.mode = Some(Mode::Admin);
        } else if game_mode == "player" {
            app.mode = Some(Mode::User);
            if app.user_id.is_none() {
                println!("Error: 'id' option is required for 'user' mode");
                std::process::exit(1);
            }
        }
    } else {
        app.mode = match app.user_id {
            Some(_) => Some(Mode::User),
            None => Some(Mode::Admin),
        };
    }

    if let Some(game_id) = matches.get_one::<PathBuf>("game-id") {
        let game_id_str = game_id.to_str().unwrap();

        if app.mode == Some(Mode::User) {
            let player_id = app.user_id.clone().unwrap();
            let response = app.user_load_game(game_id_str, &player_id).await;
            if response.is_err() {
                println!("{}", response.err().unwrap());
                println!("Exiting");
                std::process::exit(1);
            }
        } else {
            let game = app.admin_load_game(game_id_str).await;

            if game.is_err() {
                println!("{}", game.err().unwrap());
                println!("Exiting");
                std::process::exit(1);
            }
        }
    }
}
