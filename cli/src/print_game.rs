use std::io::{Error, ErrorKind};

use clap::{Arg, Command};
use mahjong_service::db_storage::DBStorage;

#[derive(Debug, Clone, PartialEq)]
pub struct PrintGameOpts {
    pub game_id: String,
}

pub async fn print_game(opts: PrintGameOpts) -> Result<(), Error> {
    let storage = DBStorage::new_dyn();

    let game = storage
        .get_game(&opts.game_id, false)
        .await
        .unwrap()
        .ok_or_else(|| {
            Error::new(
                ErrorKind::Other,
                format!("Game with ID {} not found", opts.game_id),
            )
        })?;

    println!("Game:\n{}", game.game.get_summary_sorted());

    Ok(())
}

pub fn get_print_game_command() -> Command {
    Command::new("print-game")
        .about("Print the game summary")
        .arg(
            Arg::new("game-id")
                .short('i')
                .help("The ID of the game to print"),
        )
}

pub fn get_print_game_opts(matches: &clap::ArgMatches) -> PrintGameOpts {
    let game_id: &String = matches.get_one("game-id").unwrap();

    PrintGameOpts {
        game_id: game_id.clone(),
    }
}
