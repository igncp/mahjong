use super::formatter::{get_admin_hands_str, get_board, get_draw_wall};
use super::help::get_help_text;
use crate::base::App;
use crate::play::{UIScreen, UIState};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::Frame;

pub fn draw_admin_view<B: Backend>(f: &mut Frame<B>, app: &App, ui_state: &mut UIState) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Length(10)].as_ref())
        .split(size);

    let wrapper_block = Block::default()
        .borders(Borders::ALL)
        .title("Game Panel (admin view)");

    let paragraph_style = Style::default().fg(Color::White).bg(Color::Black);

    match ui_state.screen {
        UIScreen::Init => {
            let init_text = [
                "Welcome to Mahjong!",
                "Input 'h' to display the help",
                &ui_state.input,
            ]
            .join("\n");

            let paragraph = Paragraph::new(init_text)
                .block(wrapper_block.clone())
                .wrap(Wrap { trim: true })
                .style(paragraph_style);

            f.render_widget(paragraph, chunks[0]);

            if ui_state.display_help {
                let help_text = get_help_text(app, ui_state);

                let paragraph = Paragraph::new(help_text)
                    .block(wrapper_block.clone().title("Help"))
                    .wrap(Wrap { trim: true })
                    .style(paragraph_style);

                f.render_widget(paragraph, chunks[1]);
            }

            if ui_state.display_games {
                let games_ids = app.play.games_ids.clone().unwrap();
                let mut paragraph_text = vec![Spans::from(format!("Games ({}):", games_ids.len()))];

                app.play
                    .games_ids
                    .clone()
                    .unwrap()
                    .iter()
                    .for_each(|g| paragraph_text.push(Spans::from(format!("- Game: {g}"))));

                let paragraph = Paragraph::new(paragraph_text)
                    .block(wrapper_block.clone().title("Help"))
                    .wrap(Wrap { trim: true })
                    .style(paragraph_style);

                f.render_widget(paragraph, chunks[1]);
            }
        }
        UIScreen::Game => {
            let service_game = app.play.service_game.as_ref().unwrap();
            let draw_wall_str = get_draw_wall(&service_game.game, ui_state.display_draw_wall_index);
            let board_str = get_board(&service_game.game.table.board);

            let paragraph_text = vec![
                Spans::from(format!("- Input: {}", ui_state.input)),
                Spans::from(format!(
                    "- Game ID: {} (admin) {}",
                    service_game.game.id, ui_state.messages_count
                )),
                Spans::from(format!("- Phase: {:?}", service_game.game.phase)),
            ];

            let paragraph = Paragraph::new(paragraph_text)
                .block(wrapper_block.clone())
                .wrap(Wrap { trim: true })
                .style(paragraph_style);

            f.render_widget(paragraph, chunks[0]);

            if ui_state.display_help {
                let help_text = get_help_text(app, ui_state);

                let paragraph = Paragraph::new(help_text)
                    .block(wrapper_block.clone().title("Help"))
                    .wrap(Wrap { trim: true })
                    .style(paragraph_style);

                f.render_widget(paragraph, chunks[1]);
            } else {
                let dealer = service_game.game.get_dealer();
                let mut secondary_strs = vec![
                    format!("- Dealer: {}", dealer.unwrap()),
                    format!("- Round wind: {:?}", service_game.game.round.wind),
                    format!("- Draw wall ({}):", service_game.game.table.draw_wall.len(),),
                    draw_wall_str,
                    "".to_string(),
                    format!("- Board ({}):", service_game.game.table.board.len(),),
                    board_str,
                ];

                if ui_state.display_hand {
                    get_admin_hands_str(service_game)
                        .iter()
                        .for_each(|s| secondary_strs.push(s.to_string()));
                }

                let secondary_spans = secondary_strs
                    .iter()
                    .map(|s| Spans::from(s.to_string()))
                    .collect::<Vec<_>>();

                let wrapper_block = Block::default().borders(Borders::ALL);

                let paragraph = Paragraph::new(secondary_spans)
                    .scroll((ui_state.scroll, 0))
                    .block(wrapper_block)
                    .wrap(Wrap { trim: true })
                    .style(paragraph_style);

                f.render_widget(paragraph, chunks[1]);
            }
        }
    }
}
