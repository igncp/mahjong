use clap::{Arg, ArgAction, Command};

#[derive(Debug, Clone, PartialEq)]
pub struct SimulateOpts {
    pub once: bool,
    pub debug: bool,
}

pub fn get_simulate_command() -> Command {
    Command::new("simulate")
        .about("Simulates games")
        .arg(
            Arg::new("once")
                .short('o')
                .help("Only run one simulation")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .help("Store debugging information to troubleshoot issues")
                .action(ArgAction::SetTrue),
        )
}

pub fn get_simulate_opts(matches: &clap::ArgMatches) -> SimulateOpts {
    let once: Option<&bool> = matches.get_one("once");
    let debug: Option<&bool> = matches.get_one("debug");

    SimulateOpts {
        once: once == Some(&true),
        debug: debug == Some(&true),
    }
}
