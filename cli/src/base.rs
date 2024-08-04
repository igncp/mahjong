use crate::print_game::PrintGameOpts;
use crate::simulate::SimulateOpts;

#[derive(Debug, Clone, PartialEq)]
pub enum AppCommand {
    Simulate(SimulateOpts),
    PrintGame(PrintGameOpts),
}

pub struct App {
    pub command: Option<AppCommand>,
}

impl App {
    pub fn new() -> Self {
        Self { command: None }
    }
}
