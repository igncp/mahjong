use crate::{
    base::{App, AppCommand},
    simulate::{get_simulate_command, get_simulate_opts},
};
use clap::command;

pub async fn parse_args(app: &mut App) {
    let simulate_command = get_simulate_command();

    let matches = command!().subcommand(simulate_command).get_matches();

    match matches.subcommand() {
        Some(("simulate", args_matches)) => {
            let opts = get_simulate_opts(args_matches);
            app.command = Some(AppCommand::Simulate(opts));
        }
        _ => {
            println!("Error: no command specified");
            std::process::exit(1);
        }
    }
}
