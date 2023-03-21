use base::App;
use cli::parse_args;
use ui::UI;

mod base;
mod cli;
mod log;
mod service_http_client;
mod ui;
mod view;

#[tokio::main]
async fn main() {
    let mut app = App::new().await;

    parse_args(&mut app).await;

    let mut ui = UI::new();

    ui.run(&mut app).await;
}
