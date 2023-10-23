use crate::utils::Shell;

pub fn run_setup_dev_install(shell: &Shell) {
    shell.run_status(&["cd service", "bash scripts/setup_dev_install.sh"].join(" && "));
    shell.run_status(&["cd scripts", "cargo run pack_wasm"].join(" && "));
    shell.run_status(&["cd ts_sdk", "bun i", "bun run sync_sdk"].join(" && "));
    shell.run_status(&["cd mobile_apps", "bun run setup_images"].join(" && "));
}
