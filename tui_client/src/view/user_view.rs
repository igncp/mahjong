use super::formatter::{format_player, get_board, get_user_hand_str};
use super::help::get_help_text;
use crate::base::App;
use crate::ui::{UIScreen, UIState};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::Frame;

pub fn draw_user_view<B: Backend>(f: &mut Frame<B>, app: &App, ui_state: &mut UIState) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Length(10)].as_ref())
        .split(size);

    let wrapper_block = Block::default()
        .borders(Borders::ALL)
        .title("Game Panel (user view)");

    let paragraph_style = Style::default().fg(Color::White).bg(Color::Black);

    match ui_state.screen {
        UIScreen::Init => {
            let init_text = vec![
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
                let games_ids = app.games_ids.clone().unwrap();
                let mut paragraph_text = vec![Spans::from(format!("Games ({}):", games_ids.len()))];

                app.games_ids
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
            let game = app.game_summary.as_ref().unwrap();

            let paragraph_text = vec![
                Spans::from(format!("- Input: {}", ui_state.input)),
                Spans::from(format!(
                    "- Game ID: {} (user) {}",
                    game.id, ui_state.messages_count
                )),
                Spans::from(format!("- Phase: {:?}", game.phase)),
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
                let player = game
                    .players
                    .iter()
                    .find(|p| p.id == game.player_id)
                    .unwrap();

                let current_player = game.players.get(game.round.player_index).unwrap();
                let mut secondary_strs = vec![
                    format!(
                        "- Player: {}",
                        // TODO: Here in future it should know the number of tiles another player
                        // has
                        format_player(player, current_player, Some(&game.hand), &game.score)
                    ),
                    "- Other players:".to_string(),
                ];

                for player in game.players.iter() {
                    if player.id != game.player_id {
                        secondary_strs.push(format!(
                            "-   {}",
                            format_player(player, current_player, None, &game.score)
                        ));
                    }
                }

                if ui_state.display_hand {
                    // TODO: Display other player visible melds
                    get_user_hand_str(game)
                        .iter()
                        .for_each(|s| secondary_strs.push(s.to_string()));
                }

                vec![
                    "".to_string(),
                    format!("- Draw wall tiles left: {}", game.draw_wall_count),
                    "".to_string(),
                    "- Board:".to_string(),
                    get_board(&game.board, &game.deck),
                ]
                .iter()
                .for_each(|s| secondary_strs.push(s.clone()));

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
