use crate::play::Play;

#[derive(Debug, Clone, PartialEq)]
pub enum AppCommand {
    Play,
    Simulate,
}

pub struct App {
    pub command: Option<AppCommand>,
    pub play: Play,
}

impl App {
    pub fn new() -> Self {
        let play = Play::new();

        Self {
            command: None,
            play,
        }
    }
}
