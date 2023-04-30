use crate::{
    utils::{doc, Shell},
    wasm::run_pack_wasm,
};

#[cfg(target_arch = "x86_64")]
const DOCKER_IMAGE_TAG: &str = "x86_64";
#[cfg(target_arch = "aarch64")]
const DOCKER_IMAGE_TAG: &str = "aarch64";
#[cfg(target_arch = "arm")]
const DOCKER_IMAGE_TAG: &str = "arm";

fn web(shell: &Shell) {
    run_pack_wasm(shell);

    shell.run_status("cd web_client && npm uninstall mahjong_sdk");

    shell.run_status(&vec!["cd web_client", "npm i"].join(";"));

    shell.run_status(&vec!["cd web_client", "npm run sync_sdk"].join(";"));

    shell.run_status(&vec!["cd web_client", "npm run build"].join(";"));

    doc(shell);
}

fn docker_service(shell: &Shell) {
    shell.run_status(
        // This could use buildx but that cross-compiling is not working with sqlite3
        &vec![
            "docker build",
            format!("-t 'igncp/mahjong_service:{DOCKER_IMAGE_TAG}'").as_str(),
            "-f scripts/Dockerfile.service",
            "--progress=plain",
            ".",
        ]
        .join(" "),
    );

    shell.run_status(
        &vec![
            "docker image push",
            format!("'igncp/mahjong_service:{DOCKER_IMAGE_TAG}'").as_str(),
        ]
        .join(" "),
    );
}

fn docker_front(shell: &Shell) {
    shell.run_status(
        &vec![
            "docker build",
            format!("-t 'igncp/mahjong_front:{DOCKER_IMAGE_TAG}'").as_str(),
            "-f scripts/Dockerfile.front",
            "--progress=plain",
            ".",
        ]
        .join(" "),
    );

    shell.run_status(
        &vec![
            "docker image push",
            format!("'igncp/mahjong_front:{DOCKER_IMAGE_TAG}'").as_str(),
        ]
        .join(" "),
    );
}

pub fn run_docker(shell: &Shell) {
    web(shell);
    docker_front(shell);
    docker_service(shell);
}
