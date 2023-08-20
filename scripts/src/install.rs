use crate::utils::Shell;

pub fn run_setup_dev_install(shell: &Shell) {
    shell.run_status("cd service && DATABASE_URL=sqlite://mahjong.db diesel setup");
    shell.run_status("cd scripts && cargo run pack_wasm");
    shell.run_status("cd ts_sdk && npm i && npm run sync_all");
    shell.run_status("cd mobile-apps && npm i && npm run setup_images");
}
