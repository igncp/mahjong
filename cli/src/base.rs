use crate::simulate::SimulateOpts;

#[derive(Debug, Clone, PartialEq)]
pub enum AppCommand {
    Simulate(SimulateOpts),
}

pub struct App {
    pub command: Option<AppCommand>,
}

impl App {
    pub fn new() -> Self {
        Self { command: None }
    }
}
