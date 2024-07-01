use clap::{Arg, ArgAction, Command};

#[derive(Debug, Clone, PartialEq)]
pub struct SimulateOpts {
    pub once: bool,
}

pub fn get_simulate_command() -> Command {
    Command::new("simulate").about("Simulates games").arg(
        Arg::new("once")
            .short('o')
            .help("Only run one simulation")
            .action(ArgAction::SetTrue),
    )
}

pub fn get_simulate_opts(matches: &clap::ArgMatches) -> SimulateOpts {
    let once: Option<&bool> = matches.get_one("once");

    SimulateOpts {
        once: once == Some(&true),
    }
}
