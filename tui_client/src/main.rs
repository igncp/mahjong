use base::App;
use clap::{arg, command, value_parser};
use std::path::PathBuf;
use ui::UI;

mod base;
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
        .get_matches();

    let mut app = App::new().await;

    if let Some(game_id) = matches.get_one::<PathBuf>("game-id") {
        let game_id_str = game_id.to_str().unwrap();
        let game = app.load_game(game_id_str).await;

        if game.is_err() {
            println!("{}", game.err().unwrap());
            println!("Exiting");
            std::process::exit(1);
        }
    }

    let mut ui = UI::new();

    ui.run(&mut app).await;
}
