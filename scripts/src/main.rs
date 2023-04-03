// This can be run from the root:

use std::env;

use clap::Command;

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
    run_bash_cmd("cargo clippy -- -D warnings", current_dir);
    run_bash_cmd("cargo fmt --all -- --check", current_dir);
    run_bash_cmd("cd web_lib && bash ./scripts/pack.sh", current_dir);
    run_bash_cmd(
        "cd web_client && npm i && npm run lint && npm run build",
        current_dir,
    );
}

fn fix(current_dir: &str) {
    run_bash_cmd("cd web_client && npm run lint:fix", current_dir);
}

fn docker(current_dir: &str) {
    let service_cmd = vec![
        "docker build",
        "-t 'mahjong_service_build'",
        "-f scripts/Dockerfile.service-build",
        "--progress=plain",
        ".",
    ]
    .join(" ");
    run_bash_cmd(&service_cmd, current_dir);

    let service_cmd = vec!["rm -rf dist && mkdir -p dist"].join(" ");
    run_bash_cmd(&service_cmd, current_dir);

    // TODO: Should revisit this
    let service_cmd = vec!["whoami && id -u && ls -lah dist && chmod -R 777 dist"].join(" ");
    run_bash_cmd(&service_cmd, current_dir);

    let service_cmd = vec![
        "docker run",
        "--rm",
        "-v $(pwd)/dist:/mount",
        "mahjong_service_build",
        "cp /app/target/x86_64-unknown-linux-musl/release/mahjong_service /mount/",
    ]
    .join(" ");
    run_bash_cmd(&service_cmd, current_dir);

    let service_cmd = vec![
        "docker buildx build",
        "-t 'igncp/mahjong_service'",
        "-f scripts/Dockerfile.service",
        "--platform linux/amd64,linux/arm64",
        "--push",
        "--progress=plain",
        ".",
    ]
    .join(" ");
    run_bash_cmd(&service_cmd, current_dir);
}

fn web(current_dir: &str) {
    let service_cmd = vec!["cd web_lib", "bash ./scripts/pack.sh"].join(";");
    run_bash_cmd(&service_cmd, current_dir);

    let service_cmd = vec!["cd web_client", "npm i"].join(";");
    run_bash_cmd(&service_cmd, current_dir);

    let service_cmd = vec!["cd web_client", "npm run build"].join(";");
    run_bash_cmd(&service_cmd, current_dir);
}

fn main() {
    let mut cmd = Command::new("scripts")
        .about("Run various scripts")
        .subcommand(Command::new("check").about("Run all checks"))
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
        Some(("docker", _)) => docker(current_dir),
        Some(("web", _)) => web(current_dir),
        Some(("fix", _)) => fix(current_dir),
        _ => {
            cmd.print_long_help().unwrap();
            std::process::exit(1);
        }
    }
}
