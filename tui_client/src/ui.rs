use crate::base::{App, AppEvent, Mode};
use crate::view::draw_view;
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use mahjong_core::{HandTile, PlayerId, TileId};
use std::collections::HashSet;
use std::io;
use std::time::Duration;
use tui::{backend::CrosstermBackend, Terminal};

const PAGE_UP_DOWN_SCROLL: u16 = 20;

#[derive(Debug, PartialEq)]
pub enum UIScreen {
    Game,
    Init,
}

enum UIEvent {
    Input(KeyEvent),
    Message(AppEvent),
}

pub struct UIState {
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

    fn extract_create_meld_args(&mut self, app: &App) -> Option<(PlayerId, HashSet<TileId>)> {
        let args = self.state.input.split_whitespace().collect::<Vec<&str>>();
        if args.len() < 4 {
            return None;
        }
        let player_index = args[1].parse::<usize>();
        if player_index.is_err() || player_index.clone().unwrap() > 3 {
            return None;
        }
        let game = &app.game.clone().unwrap();
        let player_id = game.players.clone()[player_index.unwrap()].id.clone();
        let player_hand = &game.table.hands[&player_id];
        let tiles = args[2..]
            .iter()
            .filter_map(|tile| {
                let tile_index = tile.parse::<usize>();
                if tile_index.is_err() {
                    return None;
                }
                let tile_id = player_hand.0[tile_index.unwrap()].id;
                Some(tile_id)
            })
            .collect::<HashSet<TileId>>();

        if tiles.len() < 2 {
            return None;
        }

        Some((player_id, tiles))
    }

    fn extract_claim_tile_args(&self, app: &App) -> Option<PlayerId> {
        let args = self.state.input.split_whitespace().collect::<Vec<&str>>();
        if args.len() < 2 {
            return None;
        }
        let player_index = args[1].parse::<usize>();
        if player_index.is_err() || player_index.clone().unwrap() > 3 {
            return None;
        }
        let game = &app.game.clone().unwrap();
        let player_id = game.players.clone()[player_index.unwrap()].id.clone();
        Some(player_id)
    }

    fn extract_discard_tile_args(&mut self, app: &App) -> Option<TileId> {
        let args = self.state.input.split_whitespace().collect::<Vec<&str>>();
        if args.len() < 2 {
            return None;
        }
        let tile_index = args[1].parse::<usize>();
        if tile_index.is_err() {
            return None;
        }
        let tile_index = tile_index.unwrap();
        for player in &app.game.as_ref().unwrap().players {
            let player_hand = &app.game.as_ref().unwrap().table.hands[&player.id];
            if player_hand.0.iter().len() == 14 {
                let filtered_hand = player_hand
                    .0
                    .iter()
                    .filter(|tile| tile.set_id.is_none())
                    .collect::<Vec<&HandTile>>();
                let tile = filtered_hand.get(tile_index);

                if let Some(tile) = tile {
                    return Some(tile.id);
                }
            }
        }

        None
    }

    pub async fn run(&mut self, app: &mut App) {
        self.prepare();

        if (app.mode == Some(Mode::Admin) && app.game.is_some())
            || (app.mode == Some(Mode::User) && app.game_summary.is_some())
        {
            self.state.screen = UIScreen::Game;
            self.state.display_hand = true;
        }

        loop {
            self.terminal
                .draw(|f| {
                    draw_view(f, app, &mut self.state);
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
                            if ch == 'c' && key.modifiers.contains(KeyModifiers::CONTROL) {
                                break;
                            }
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

                            let input_fragment =
                                self.state.input.split_whitespace().next().unwrap_or("");

                            match input_fragment {
                                "h" => {
                                    self.state.display_help = !self.state.display_help;
                                }
                                "q" => {
                                    break;
                                }
                                _ => match self.state.screen {
                                    UIScreen::Init => match input_fragment {
                                        "ss" => {
                                            self.create_game(app).await;
                                        }
                                        "gg" => {
                                            app.get_games().await;
                                            self.state.display_games = true;
                                        }
                                        _ => {}
                                    },
                                    UIScreen::Game => match input_fragment {
                                        "claim" => {
                                            let parsed_input = self.extract_claim_tile_args(app);
                                            if let Some(player_id) = parsed_input {
                                                app.admin_claim_tile(&player_id).await;
                                            }
                                            self.state.display_hand = true;
                                        }
                                        "cm" => {
                                            let parsed_input = self.extract_create_meld_args(app);
                                            if let Some((player_id, tiles)) = parsed_input {
                                                let new_hand =
                                                    app.admin_create_meld(&player_id, &tiles).await;
                                                app.game
                                                    .as_mut()
                                                    .unwrap()
                                                    .table
                                                    .hands
                                                    .insert(player_id, new_hand);
                                                self.state.display_hand = true;
                                            }
                                        }
                                        "di" => {
                                            if app.is_current_player() {
                                                let parsed_input =
                                                    self.extract_discard_tile_args(app);
                                                if let Some(tile_id) = parsed_input {
                                                    if app.mode == Some(Mode::Admin) {
                                                        app.admin_discard_tile(&tile_id).await;
                                                    } else {
                                                        app.user_discard_tile(&tile_id).await;
                                                    }
                                                }
                                            }
                                            self.state.display_hand = true;
                                        }
                                        "draw" => {
                                            app.admin_draw_tile().await;
                                            self.state.display_hand = true;
                                        }
                                        "hd" => {
                                            self.state.display_hand = true;
                                        }
                                        "n" => {
                                            app.admin_move_player_combined().await;
                                            self.state.display_hand = true;
                                        }
                                        "next" => {
                                            app.admin_move_player().await;
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
