#![deny(clippy::use_self)]
use base::{App, AppCommand};
use cli::parse_args;
use simulate::run_simulation;

mod base;
mod cli;
mod log;
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
    }
}
