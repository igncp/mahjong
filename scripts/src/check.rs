use crate::{
    utils::{doc, Shell},
    wasm::run_pack_wasm,
};

pub fn run_check(shell: &Shell) {
    shell.run_status("cargo build --release");
    shell.run_status("cargo check --workspace --release");
    shell.run_status("cargo test");

    run_clippy(shell);

    shell.run_status("cargo fmt --all -- --check");
    run_pack_wasm(shell);
    shell.run_status("cd ts_sdk && npm i && npm pack");

    shell.run_status("cd web_client && npm uninstall mahjong_sdk");
    shell.run_status("cd mobile_apps && npm uninstall mahjong_sdk");

    shell.run_status("cd web_client && npm i && npm run sync_sdk && npm run lint && npm run build");
    shell.run_status(
        "cd mobile_apps && npm i && npm run sync_sdk && npm run typecheck && npm run lint",
    );

    doc(shell);
}

pub fn run_clippy(shell: &Shell) {
    shell.run_status("cargo clippy --all-targets --all-features -- -D warnings");
}

pub fn run_fix(shell: &Shell) {
    shell.run_status("cd web_client && npm run lint:fix");
}
