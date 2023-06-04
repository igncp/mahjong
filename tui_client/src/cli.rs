use crate::{
    base::{App, AppCommand},
    play::{get_play_command, parse_play_args},
    simulate::get_simulate_command,
};
use clap::command;

pub async fn parse_args(app: &mut App) {
    let play_command = get_play_command();
    let simulate_command = get_simulate_command();

    let matches = command!()
        .subcommand(simulate_command)
        .subcommand(play_command)
        .get_matches();

    match matches.subcommand() {
        Some(("play", subcommand_matches)) => {
            parse_play_args(app, subcommand_matches).await;
        }
        Some(("simulate", _)) => {
            app.command = Some(AppCommand::Simulate);
        }
        _ => {
            println!("Error: no command specified");
            std::process::exit(1);
        }
    }
}
