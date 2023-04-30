#![deny(clippy::use_self, clippy::shadow_unrelated)]

// This can be run from the root:

use check::{run_check, run_clippy, run_fix};
use clap::Command;
use docker::run_docker;
use std::env;
use sync_prod::run_sync_prod;
use utils::Shell;
use wasm::run_pack_wasm;

mod check;
mod docker;
mod sync_prod;
mod utils;
mod wasm;

// This is specially convenient for maintaining the clippy rules, which need to be in each crate
fn list(current_dir: &str) {
    let prefix = if current_dir == "scripts" { "../" } else { "" };
    let list_str = vec![
        "mahjong_core/src/lib.rs",
        "scripts/src/main.rs",
        "service/src/main.rs",
        "service_contracts/src/lib.rs",
        "tui_client/src/main.rs",
        "web_lib/src/lib.rs",
    ]
    .iter()
    .map(|path| format!("{}{}", prefix, path))
    .collect::<Vec<String>>()
    .join("\n");

    println!("{list_str}");
}

fn main() {
    let mut cmd = Command::new("scripts")
        .about("Run various scripts for the Mahjong project")
        .subcommand(Command::new("check").about("Run all checks"))
        .subcommand(Command::new("clippy").about("Run only clippy checks"))
        .subcommand(Command::new("docker").about("Build docker images"))
        .subcommand(Command::new("fix").about("Run linters in fix mode"))
        .subcommand(Command::new("list").about("List root files to be used in a pipe"))
        .subcommand(Command::new("pack_wasm").about("Pack the wasm files"))
        .subcommand(Command::new("sync_prod").about("Deploy a clean production DB"))
        .subcommand(Command::new("web").about("Build the web client"));

    let current_dir_path = env::current_dir().unwrap();
    let current_dir = current_dir_path
        .to_str()
        .unwrap()
        .split('/')
        .last()
        .unwrap();

    let mut shell = Shell::new(current_dir);

    match cmd.clone().get_matches().subcommand() {
        Some(("check", _)) => run_check(&shell),
        Some(("clippy", _)) => run_clippy(&shell),
        Some(("docker", _)) => run_docker(&shell),
        Some(("fix", _)) => run_fix(&shell),
        Some(("list", _)) => list(current_dir),
        Some(("pack_wasm", _)) => run_pack_wasm(&shell),
        Some(("sync_prod", _)) => run_sync_prod(&mut shell),
        _ => {
            cmd.print_long_help().unwrap();
            std::process::exit(1);
        }
    }
}
