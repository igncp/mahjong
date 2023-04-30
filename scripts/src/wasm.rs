use crate::utils::Shell;

pub fn run_pack_wasm(shell: &Shell) {
    // mv pkg ../web_client/pkg
    shell.run_status("rm -rf web_lib/pkg web_client/pkg");
    shell.run_status("cd web_lib && wasm-pack build --release");
    shell.run_status("mv web_lib/pkg web_client/pkg");
}
