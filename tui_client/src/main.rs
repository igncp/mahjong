#![deny(clippy::use_self)]
use base::{App, AppCommand};
use cli::parse_args;
use play::PlayUI;
use simulate::run_simulation;

mod base;
mod cli;
mod log;
mod play;
mod service_http_client;
mod simulate;

#[tokio::main]
async fn main() {
    let mut app = App::new();

    parse_args(&mut app).await;

    let command = app.command.clone().unwrap();

    match command {
        AppCommand::Play => {
            let mut play_ui = PlayUI::new();
            play_ui.run_play(&mut app).await;
        }
        AppCommand::Simulate => {
            run_simulation().await;
        }
    }
}
