#![deny(clippy::use_self, clippy::shadow_unrelated)]

// This can be run from the root:

use clap::Command;
use std::env;

#[cfg(target_arch = "x86_64")]
const DOCKER_IMAGE_TAG: &str = "x86_64";
#[cfg(target_arch = "aarch64")]
const DOCKER_IMAGE_TAG: &str = "aarch64";
#[cfg(target_arch = "arm")]
const DOCKER_IMAGE_TAG: &str = "arm";

fn run_bash_cmd(cmd: &str, current_dir: &str) {
    let prefix = if current_dir == "scripts" {
        "cd .. && "
    } else {
        ""
    };

    println!();
    println!("Running: {}{}", prefix, cmd);
    let status = std::process::Command::new("bash")
        .arg("-c")
        .arg(format!("{prefix}{cmd}"))
        .status()
        .unwrap();

    if !status.success() {
        std::process::exit(1);
    }
}

fn check(current_dir: &str) {
    run_bash_cmd("cargo build --release", current_dir);
    run_bash_cmd("cargo check --workspace --release", current_dir);
    run_bash_cmd("cargo test", current_dir);

    clippy(current_dir);

    run_bash_cmd("cargo fmt --all -- --check", current_dir);
    run_bash_cmd("cd web_lib && bash ./scripts/pack.sh", current_dir);
    run_bash_cmd(
        "cd web_client && npm i && npm run lint && npm run build",
        current_dir,
    );

    doc(current_dir);
}

fn doc(_current_dir: &str) {
    // This is failing currently
    // run_bash_cmd("cargo doc --release", current_dir);
}

fn clippy(current_dir: &str) {
    run_bash_cmd(
        "cargo clippy --all-targets --all-features -- -D warnings",
        current_dir,
    );
}

fn fix(current_dir: &str) {
    run_bash_cmd("cd web_client && npm run lint:fix", current_dir);
}

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

fn docker_service(current_dir: &str) {
    run_bash_cmd(
        // This could use buildx but that cross-compiling is not working with sqlite3
        &vec![
            "docker build",
            format!("-t 'igncp/mahjong_service:{DOCKER_IMAGE_TAG}'").as_str(),
            "-f scripts/Dockerfile.service",
            "--progress=plain",
            ".",
        ]
        .join(" "),
        current_dir,
    );

    run_bash_cmd(
        &vec![
            "docker image push",
            format!("'igncp/mahjong_service:{DOCKER_IMAGE_TAG}'").as_str(),
        ]
        .join(" "),
        current_dir,
    );
}

fn docker_front(current_dir: &str) {
    run_bash_cmd(
        &vec![
            "docker build",
            format!("-t 'igncp/mahjong_front:{DOCKER_IMAGE_TAG}'").as_str(),
            "-f scripts/Dockerfile.front",
            "--progress=plain",
            ".",
        ]
        .join(" "),
        current_dir,
    );

    run_bash_cmd(
        &vec![
            "docker image push",
            format!("'igncp/mahjong_front:{DOCKER_IMAGE_TAG}'").as_str(),
        ]
        .join(" "),
        current_dir,
    );
}

fn docker(current_dir: &str) {
    web(current_dir);
    docker_front(current_dir);
    docker_service(current_dir);
}

fn web(current_dir: &str) {
    run_bash_cmd(
        &vec!["cd web_lib", "bash ./scripts/pack.sh"].join(";"),
        current_dir,
    );

    run_bash_cmd(&vec!["cd web_client", "npm i"].join(";"), current_dir);

    run_bash_cmd(
        &vec!["cd web_client", "npm run build"].join(";"),
        current_dir,
    );

    doc(current_dir);
}

fn main() {
    let mut cmd = Command::new("scripts")
        .about("Run various scripts")
        .subcommand(Command::new("check").about("Run all checks"))
        .subcommand(Command::new("clippy").about("Run only clippy checks"))
        .subcommand(Command::new("list").about("List root files to be used in a pipe"))
        .subcommand(Command::new("fix").about("Run linters in fix mode"))
        .subcommand(Command::new("docker").about("Build docker images"))
        .subcommand(Command::new("web").about("Build the web client"));

    let current_dir_path = env::current_dir().unwrap();
    let current_dir = current_dir_path
        .to_str()
        .unwrap()
        .split('/')
        .last()
        .unwrap();

    match cmd.clone().get_matches().subcommand() {
        Some(("check", _)) => check(current_dir),
        Some(("clippy", _)) => clippy(current_dir),
        Some(("docker", _)) => docker(current_dir),
        Some(("list", _)) => list(current_dir),
        Some(("fix", _)) => fix(current_dir),
        _ => {
            cmd.print_long_help().unwrap();
            std::process::exit(1);
        }
    }
}
