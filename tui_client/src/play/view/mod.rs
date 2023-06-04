mod admin_view;
mod formatter;
mod help;
mod user_view;
use crate::{base::App, play::UIState};
use admin_view::draw_admin_view;
use tui::{backend::Backend, Frame};
use user_view::draw_user_view;

use super::PlayMode;

pub fn draw_view<B: Backend>(f: &mut Frame<B>, app: &App, ui_state: &mut UIState) {
    match app.play.mode.clone().unwrap() {
        PlayMode::User => draw_user_view(f, app, ui_state),
        PlayMode::Admin => draw_admin_view(f, app, ui_state),
    }
}
