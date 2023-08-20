use crate::{
    utils::{doc, Shell},
    wasm::run_pack_wasm,
};

pub fn run_check(shell: &Shell) {
    shell.run_status("cargo build --release");
    shell.run_status("cargo check --workspace --release");
    shell.run_status("cargo test --release");

    run_clippy(shell);

    run_pack_wasm(shell);
    shell.run_status("cd ts_sdk && npm i && npm run lint && npm pack");

    shell.run_status("cd web_client && npm uninstall mahjong_sdk");
    shell.run_status("cd mobile_apps && npm uninstall mahjong_sdk");

    shell.run_status("cd web_client && npm i && npm run sync_sdk && npm run lint && npm run test && npm run build");
    shell.run_status(
        "cd mobile_apps && npm i && npm run sync_sdk && npm run typecheck && npm run lint && npm run test",
    );

    doc(shell);
}

pub fn run_clippy(shell: &Shell) {
    shell.run_status("cargo clippy --all-targets --all-features -- -D warnings");
}

pub fn run_fix(shell: &Shell) {
    shell.run_status("cd web_client && npm run lint:fix");
}
