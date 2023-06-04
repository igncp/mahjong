use clap::Command;

pub fn get_simulate_command() -> Command {
    Command::new("simulate").about("Simulates games")
}
