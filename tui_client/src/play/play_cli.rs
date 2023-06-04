use clap::{arg, value_parser, ArgMatches, Command};

use crate::base::{App, AppCommand};
use std::path::PathBuf;

use super::PlayMode;

pub fn get_play_command() -> Command {
    Command::new("play").about("Play a game [deprecated]")
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
}

pub async fn parse_play_args(app: &mut App, matches: &ArgMatches) {
    app.command = Some(AppCommand::Play);

    app.play.user_id = match matches.get_one::<PathBuf>("id") {
        Some(user_id) => {
            let user_id_str = user_id.to_str().unwrap();
            Some(user_id_str.to_string())
        }
        None => None,
    };

    if let Some(game_mode) = matches.get_one::<PathBuf>("mode") {
        let game_mode = game_mode.to_str().unwrap();
        if game_mode == "admin" {
            app.play.mode = Some(PlayMode::Admin);
        } else if game_mode == "player" {
            app.play.mode = Some(PlayMode::User);
            if app.play.user_id.is_none() {
                println!("Error: 'id' option is required for 'user' mode");
                std::process::exit(1);
            }
        }
    } else {
        app.play.mode = match app.play.user_id {
            Some(_) => Some(PlayMode::User),
            None => Some(PlayMode::Admin),
        };
    }

    if let Some(game_id) = matches.get_one::<PathBuf>("game-id") {
        let game_id_str = game_id.to_str().unwrap();

        if app.play.mode == Some(PlayMode::User) {
            let player_id = app.play.user_id.clone().unwrap();
            let response = app.play.user_load_game(game_id_str, &player_id).await;
            if response.is_err() {
                println!("{}", response.err().unwrap());
                println!("Exiting");
                std::process::exit(1);
            }
        } else {
            let game = app.play.admin_load_game(game_id_str).await;

            if game.is_err() {
                println!("{}", game.err().unwrap());
                println!("Exiting");
                std::process::exit(1);
            }
        }
    }
}
