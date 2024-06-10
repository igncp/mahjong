use crate::base::App;
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use mahjong_core::{HandTile, PlayerId, TileId};
use rustc_hash::FxHashSet;
use std::io;
use std::time::Duration;
use tui::{backend::CrosstermBackend, Terminal};

use super::view::draw_view;
use super::{PlayEvent, PlayMode};

const PAGE_UP_DOWN_SCROLL: u16 = 20;

#[derive(Debug, PartialEq)]
pub enum UIScreen {
    Game,
    Init,
}

enum UIEvent {
    Input(KeyEvent),
    Message,
}

pub struct UIState {
    pub display_draw_wall_index: bool,
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
        Self {
            display_draw_wall_index: false,
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

pub struct PlayUI {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    state: UIState,
}

impl PlayUI {
    pub fn new() -> Self {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();
        let state = UIState::new();

        Self { terminal, state }
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

        app.play.admin_start_game().await;

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

    async fn wait_for_message_event(&mut self, app: &mut App) -> Option<PlayEvent> {
        let response = app.play.wait_for_message().await;

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

                Some(UIEvent::Message)
            }
        }
    }

    fn extract_create_meld_args(&mut self, app: &App) -> Option<(PlayerId, FxHashSet<TileId>)> {
        let args = self.state.input.split_whitespace().collect::<Vec<&str>>();
        if args.len() < 4 {
            return None;
        }
        let player_index = args[1].parse::<usize>();
        if player_index.is_err() || player_index.clone().unwrap() > 3 {
            return None;
        }
        let game = &app.play.service_game.clone().unwrap().game;
        let player_id = game.players.0.clone()[player_index.unwrap()].clone();
        let player_hand = &game.table.hands.0[&player_id];
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
            .collect::<FxHashSet<TileId>>();

        if tiles.len() < 2 {
            return None;
        }

        Some((player_id, tiles))
    }

    fn extract_say_mahjong_args(&mut self, app: &App) -> Option<PlayerId> {
        let args = self.state.input.split_whitespace().collect::<Vec<&str>>();
        if args.len() != 2 {
            return None;
        }
        let player_index = args[1].parse::<usize>();
        if player_index.is_err() {
            return None;
        }

        let player_index = player_index.unwrap();
        let player = app
            .play
            .service_game
            .as_ref()
            .unwrap()
            .game
            .players
            .get(player_index);

        player?;

        Some(player.unwrap().clone())
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
        let game = &app.play.service_game.clone().unwrap().game;
        let player_id = game.players.0.clone()[player_index.unwrap()].clone();
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
        for player in &app.play.service_game.as_ref().unwrap().game.players.0 {
            let player_hand = &app.play.service_game.as_ref().unwrap().game.table.hands.0[player];
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

    fn extract_swap_tiles_args(&mut self, app: &App) -> Option<(TileId, TileId)> {
        let args = self.state.input.split_whitespace().collect::<Vec<&str>>();
        if args.len() < 2 {
            return None;
        }
        let tile_index_a = args[1].parse::<usize>();
        let tile_index_b = args.get(2).unwrap_or(&"_").parse::<usize>();
        if tile_index_a.is_err() {
            return None;
        }
        let draw_wall_len = app
            .play
            .service_game
            .as_ref()
            .unwrap()
            .game
            .table
            .draw_wall
            .0
            .len();
        let tile_index_a = tile_index_a.unwrap();
        let tile_index_b = tile_index_b.unwrap_or(draw_wall_len - 1);

        let tile_id_a = app
            .play
            .service_game
            .as_ref()
            .unwrap()
            .game
            .table
            .draw_wall
            .0
            .get(tile_index_a);
        let tile_id_b = app
            .play
            .service_game
            .as_ref()
            .unwrap()
            .game
            .table
            .draw_wall
            .0
            .get(tile_index_b);

        if tile_id_a.is_none() || tile_id_b.is_none() {
            return None;
        }

        Some((*tile_id_a.unwrap(), *tile_id_b.unwrap()))
    }

    pub async fn run_play(&mut self, app: &mut App) {
        let health = app.play.service_client.check_health().await;

        if health.is_err() {
            println!("Error: {}", health.err().unwrap());
            std::process::exit(1);
        }

        self.prepare();

        if (app.play.mode == Some(PlayMode::Admin) && app.play.service_game.is_some())
            || (app.play.mode == Some(PlayMode::User) && app.play.service_game_summary.is_some())
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
                    if app.play.waiting {
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
                            self.state.display_draw_wall_index = false;

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
                                            app.play.get_games().await;
                                            self.state.display_games = true;
                                        }
                                        _ => {}
                                    },
                                    UIScreen::Game => match input_fragment {
                                        "ai" => {
                                            app.play.admin_ai_continue().await;
                                            self.state.display_hand = true;
                                        }
                                        "claim" => {
                                            let parsed_input = self.extract_claim_tile_args(app);
                                            if let Some(player_id) = parsed_input {
                                                app.play.admin_claim_tile(&player_id).await;
                                            }
                                            self.state.display_hand = true;
                                        }
                                        "cm" => {
                                            let parsed_input = self.extract_create_meld_args(app);
                                            if let Some((player_id, tiles)) = parsed_input {
                                                let new_hand = app
                                                    .play
                                                    .admin_create_meld(&player_id, &tiles)
                                                    .await;
                                                app.play
                                                    .service_game
                                                    .as_mut()
                                                    .unwrap()
                                                    .game
                                                    .table
                                                    .hands
                                                    .0
                                                    .insert(player_id, new_hand);
                                                self.state.display_hand = true;
                                            }
                                        }
                                        "di" => {
                                            if app.play.is_current_player() {
                                                let parsed_input =
                                                    self.extract_discard_tile_args(app);
                                                if let Some(tile_id) = parsed_input {
                                                    if app.play.mode == Some(PlayMode::Admin) {
                                                        app.play.admin_discard_tile(&tile_id).await;
                                                    } else {
                                                        app.play.user_discard_tile(&tile_id).await;
                                                    }
                                                }
                                            }
                                            self.state.display_hand = true;
                                        }
                                        "dwi" => {
                                            self.state.display_hand = true;
                                            self.state.display_draw_wall_index = true;
                                        }
                                        "draw" => {
                                            app.play.admin_draw_tile().await;
                                            self.state.display_hand = true;
                                        }
                                        "hd" => {
                                            self.state.display_hand = true;
                                        }
                                        "n" => {
                                            app.play.admin_move_player_combined().await;
                                            self.state.display_hand = true;
                                        }
                                        "next" => {
                                            app.play.admin_move_player().await;
                                            self.state.display_hand = true;
                                        }
                                        "sm" => {
                                            let parsed_input = self.extract_say_mahjong_args(app);
                                            if let Some(player_id) = parsed_input {
                                                app.play.admin_say_mahjong(&player_id).await;
                                            }
                                            self.state.display_hand = true;
                                        }
                                        "sh" => {
                                            app.play.admin_sort_hands().await;
                                            self.state.display_hand = true;
                                        }
                                        "sw" => {
                                            let parsed_input = self.extract_swap_tiles_args(app);
                                            if let Some((tile_id_a, tile_id_b)) = parsed_input {
                                                app.play
                                                    .admin_swap_wall_tiles(tile_id_a, tile_id_b)
                                                    .await;
                                            }
                                            self.state.display_hand = true;
                                        }
                                        "oo" => {
                                            app.play.admin_send_foo().await;
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
                UIEvent::Message => {}
            };
        }

        self.teardown();
    }
}
