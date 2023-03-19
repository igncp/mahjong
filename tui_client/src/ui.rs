use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io;
use std::time::Duration;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::Frame;
use tui::{backend::CrosstermBackend, Terminal};

use crate::base::{App, AppEvent, Mode};
use crate::formatter::{get_draw_wall, get_hand_str};

const PAGE_UP_DOWN_SCROLL: u16 = 20;

#[derive(Debug, PartialEq)]
enum UIScreen {
    Game,
    Init,
}

enum UIEvent {
    Input(KeyEvent),
    Message(AppEvent),
}

struct UIState {
    pub display_games: bool,
    pub display_hand: bool,
    pub display_help: bool,
    pub input: String,
    pub messages_count: u16,
    pub screen: UIScreen,
    pub scroll: u16,
}

impl UIState {
    pub fn new() -> Self {
        UIState {
            display_games: false,
            display_hand: false,
            display_help: false,
            input: String::new(),
            messages_count: 0,
            screen: UIScreen::Init,
            scroll: 0,
        }
    }
}

pub struct UI {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    state: UIState,
}

fn get_help_text(app: &App, ui_state: &UIState) -> String {
    let mut help_list = vec!["h: Display or hide this help", "q: Quit"];

    if ui_state.screen == UIScreen::Game {
        if app.mode == Some(Mode::Admin) {
            help_list.push("- hd: [Admin] show hands");
            help_list.push("- draw: [Admin] draw a tile for the player from the tiles wall");
            help_list.push("- sh: [Admin] sort hands");
        }
    } else if app.mode == Some(Mode::Admin) {
        help_list.push("ss: [Admin] start a game");
        help_list.push("gg: [Admin] get games");
    }

    help_list.join("\n")
}

fn draw<B: Backend>(f: &mut Frame<B>, app: &App, ui_state: &mut UIState) {
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
            let game = app.game.as_ref().unwrap();
            let current_player = game.get_current_player();
            let draw_wall_str = get_draw_wall(game);

            let paragraph_text = vec![
                Spans::from(format!("- Input: {}", ui_state.input)),
                Spans::from(format!(
                    "- Game ID: {} ({}) {}",
                    game.id,
                    match app.mode {
                        Some(Mode::User) => "user",
                        Some(Mode::Admin) => "admin",
                        _ => panic!("Invalid mode"),
                    },
                    ui_state.messages_count
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
                let mut secondary_strs = vec![
                    format!("- Current player: {}", current_player.name),
                    format!("- Draw wall ({}):", game.table.draw_wall.len(),),
                    draw_wall_str,
                ];

                if ui_state.display_hand {
                    get_hand_str(game)
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

impl UI {
    pub fn new() -> Self {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();
        let state = UIState::new();

        UI { terminal, state }
    }

    pub fn prepare(&self) {
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    }

    pub fn teardown(&mut self) {
        disable_raw_mode().unwrap();
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();

        self.terminal.show_cursor().unwrap();
    }

    async fn create_game(&mut self, app: &mut App) {
        if self.state.screen != UIScreen::Init {
            return;
        }

        app.admin_start_game().await;

        self.state.screen = UIScreen::Game;
    }

    async fn wait_for_key_event() -> Option<KeyEvent> {
        let tick_rate = Duration::from_millis(250);

        if crossterm::event::poll(tick_rate).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                return Some(key);
            }
        }

        None
    }

    async fn get_games(&mut self, app: &mut App) {
        app.admin_get_games().await;
        self.state.display_games = true;
    }

    async fn wait_for_message_event(&mut self, app: &mut App) -> Option<AppEvent> {
        let response = app.wait_for_message().await;

        if response.is_err() {
            return None;
        }

        self.state.messages_count += 1;
        let message = response.unwrap();

        Some(message)
    }

    async fn wait_for_event(&mut self, app: &mut App) -> Option<UIEvent> {
        if self.state.screen == UIScreen::Init {
            let key = Self::wait_for_key_event().await;

            key?;

            return Some(UIEvent::Input(key.unwrap()));
        }

        tokio::select! {
            key = Self::wait_for_key_event() => {
                key?;

                Some(UIEvent::Input(key.unwrap()))
            }
            message = self.wait_for_message_event(app) => {
                message.clone()?;

                Some(UIEvent::Message(message.unwrap()))
            }
        }
    }

    pub async fn run(&mut self, app: &mut App) {
        if app.mode == Some(Mode::User) {
            println!("'user' mode is not yet supported");
            std::process::exit(1);
        }

        self.prepare();

        if app.game.is_some() {
            self.state.screen = UIScreen::Game;
        }

        loop {
            self.terminal
                .draw(|f| {
                    draw(f, app, &mut self.state);
                })
                .unwrap();

            let event = self.wait_for_event(app).await;
            if event.is_none() {
                continue;
            }

            let event = event.unwrap();

            match event {
                UIEvent::Input(key) => {
                    if app.waiting {
                        continue;
                    }

                    match key.code {
                        KeyCode::Char(ch) => {
                            self.state.input = format!("{}{}", self.state.input, ch);
                        }
                        KeyCode::Down => {
                            self.state.scroll += 1;
                        }
                        KeyCode::PageDown => {
                            self.state.scroll += PAGE_UP_DOWN_SCROLL;
                        }
                        KeyCode::Up => {
                            if self.state.scroll > 0 {
                                self.state.scroll -= 1;
                            }
                        }
                        KeyCode::PageUp => {
                            if self.state.scroll > PAGE_UP_DOWN_SCROLL {
                                self.state.scroll -= PAGE_UP_DOWN_SCROLL;
                            } else {
                                self.state.scroll = 0;
                            }
                        }
                        KeyCode::Enter => {
                            self.state.display_hand = false;
                            self.state.display_games = false;

                            self.state.scroll = 0;
                            if self.state.display_help && self.state.input != "h" {
                                self.state.display_help = false;
                            }

                            match self.state.input.as_str() {
                                "h" => {
                                    self.state.display_help = !self.state.display_help;
                                }
                                "q" => {
                                    break;
                                }
                                _ => match self.state.screen {
                                    UIScreen::Init => match self.state.input.as_str() {
                                        "ss" => {
                                            self.create_game(app).await;
                                        }
                                        "gg" => {
                                            self.get_games(app).await;
                                        }
                                        _ => {}
                                    },
                                    UIScreen::Game => match self.state.input.as_str() {
                                        "draw" => {
                                            app.admin_draw_tile().await;
                                            self.state.display_hand = true;
                                        }
                                        "hd" => {
                                            self.state.display_hand = true;
                                        }
                                        "sh" => {
                                            app.admin_sort_hands().await;
                                            self.state.display_hand = true;
                                        }
                                        "oo" => {
                                            app.admin_send_foo().await;
                                        }
                                        _ => {}
                                    },
                                },
                            };
                            self.state.input = "".to_string();
                        }
                        KeyCode::Backspace => {
                            self.state.input.pop();
                        }
                        _ => {}
                    };
                }
                UIEvent::Message(_) => {}
            };
        }

        self.teardown();
    }
}
