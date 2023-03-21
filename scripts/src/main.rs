// This can be run from the root:
// cargo run --bin scripts check

use std::env;

use clap::Command;

fn run_bash_cmd(cmd: &str, current_dir: &str) {
    let prefix = if current_dir == "scripts" {
        "cd .. && "
    } else {
        ""
    };

    let status = std::process::Command::new("bash")
        .arg("-c")
        .arg(format!("{prefix}{cmd}"))
        .status()
        .unwrap();

    if !status.success() {
        std::process::exit(1);
    }
}

fn check() {
    let current_dir_path = env::current_dir().unwrap();
    let current_dir = current_dir_path
        .to_str()
        .unwrap()
        .split('/')
        .last()
        .unwrap();

    run_bash_cmd("cargo check", current_dir);
    run_bash_cmd("cargo test", current_dir);
    run_bash_cmd("cargo clippy -- -D warnings", current_dir);
    run_bash_cmd("cargo fmt --all -- --check", current_dir);
    run_bash_cmd("cargo build --release", current_dir);
}

fn main() {
    let mut cmd = Command::new("scripts")
        .about("Run various scripts")
        .subcommand(Command::new("check").about("Run all checks"));

    match cmd.clone().get_matches().subcommand() {
        Some(("check", _)) => check(),
        _ => {
            cmd.print_long_help().unwrap();
            std::process::exit(1);
        }
    }
}
