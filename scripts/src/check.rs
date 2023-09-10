use crate::{
    utils::{doc, Shell},
    wasm::run_pack_wasm,
};

pub fn run_check(shell: &Shell) {
    shell.run_status("cargo build --release");
    shell.run_status("cargo check --workspace --release");
    shell.run_status("cargo test --release");

    run_clippy(shell);

    shell.run_status("cd service && sqlfluff fix --dialect sqlite migrations/**/*.sql");

    run_pack_wasm(shell);

    shell.run_status(&["cd ts_sdk", "bun run sync_sdk", "bun run lint"].join(" && "));

    shell.run_status(
        &[
            "cd web_client",
            "bun run lint",
            "bun run test",
            "bun run build",
        ]
        .join(" && "),
    );

    shell.run_status(
        &[
            "cd mobile_apps",
            "bun run typecheck",
            "bun run lint",
            "bun run test",
        ]
        .join(" && "),
    );

    doc(shell);
}

pub fn run_clippy(shell: &Shell) {
    shell.run_status("cargo clippy --all-targets --all-features -- -D warnings");
}

pub fn run_fix(shell: &Shell) {
    shell.run_status(&["cd web_client", "bun run lint:fix"].join(" && "));
}
