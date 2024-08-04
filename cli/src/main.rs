#![deny(clippy::use_self)]
use base::{App, AppCommand};
use cli::parse_args;
use print_game::print_game;
use simulate::run_simulation;

mod base;
mod cli;
mod log;
mod print_game;
mod simulate;

#[tokio::main]
async fn main() {
    let mut app = App::new();

    parse_args(&mut app).await;

    let command = app.command.clone().unwrap();

    match command {
        AppCommand::Simulate(opts) => {
            run_simulation(opts).await;
        }
        AppCommand::PrintGame(opts) => {
            print_game(opts).await.unwrap_or_else(|e| {
                println!("Error: {:?}", e);
            });
        }
    }
}
