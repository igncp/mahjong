use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
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

use crate::base::{App, AppDisplay};

pub struct UI {
    pub terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let wrapper_block = Block::default()
        .borders(Borders::ALL)
        .title("Game Panel (admin view)");
    let paragraph_style = Style::default().fg(Color::White).bg(Color::Black);

    match app.display {
        AppDisplay::Init => {
            let init_text = vec![
                "Welcome to Mahjong!",
                "Press 's' to start a new game.",
                "Press 'q' to quit.",
            ]
            .join("\n");

            let paragraph = Paragraph::new(init_text)
                .block(wrapper_block)
                .wrap(Wrap { trim: true })
                .style(paragraph_style);

            f.render_widget(paragraph, chunks[0]);
        }
        AppDisplay::Game => {
            let game = app.game.as_ref().unwrap();
            let paragraph_text = vec![
                Spans::from(format!("- ID: {}", game.id)),
                Spans::from(format!("- Phase: {:?}", game.phase)),
            ];

            let paragraph = Paragraph::new(paragraph_text)
                .block(wrapper_block)
                .wrap(Wrap { trim: true })
                .style(paragraph_style);

            f.render_widget(paragraph, chunks[0]);
        }
    }
}

impl UI {
    pub fn new() -> Self {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();

        UI { terminal }
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

    async fn handle_s_key(&mut self, app: &mut App) {
        if app.display != AppDisplay::Init {
            return;
        }

        app.waiting = true;

        app.start_game().await;

        app.display = AppDisplay::Game;
        app.waiting = false;
    }

    pub async fn run(&mut self, app: &mut App) {
        let tick_rate = Duration::from_millis(250);
        self.prepare();

        loop {
            self.terminal
                .draw(|f| {
                    draw(f, app);
                })
                .unwrap();

            if crossterm::event::poll(tick_rate).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    if app.waiting {
                        continue;
                    }

                    match key.code {
                        KeyCode::Char('s') => {
                            self.handle_s_key(app).await;
                        }
                        KeyCode::Char('q') => break,
                        _ => {}
                    }
                }
            }
        }

        self.teardown();
    }
}
