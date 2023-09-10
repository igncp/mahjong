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

    shell.run_status(&["cd ts_sdk", "bun run sync_sdk"].join(" && "));

    shell.run_status(&["cd web_client", "bun run build"].join(" && "));

    doc(shell);
}

fn docker_service(shell: &Shell) {
    // Now this can only target x86-64
    shell.run_status(&[
    "cd service",
    "cargo build --release --target-dir target",
    "patchelf --set-interpreter /lib/x86_64-linux-gnu/ld-linux-x86-64.so.2 ./target/release/mahjong_service",
    ].join(" && "));

    shell.run_status(
        &[
            "docker build",
            format!("-t 'igncp/mahjong_service:{DOCKER_IMAGE_TAG}'").as_str(),
            "-f scripts/Dockerfile.service",
            "--progress=plain",
            ".",
        ]
        .join(" "),
    );

    shell.run_status(
        &[
            "docker image push",
            format!("'igncp/mahjong_service:{DOCKER_IMAGE_TAG}'").as_str(),
        ]
        .join(" "),
    );
}

fn docker_front(shell: &Shell) {
    shell.run_status(
        &[
            "docker build",
            format!("-t 'igncp/mahjong_front:{DOCKER_IMAGE_TAG}'").as_str(),
            "-f scripts/Dockerfile.front",
            "--progress=plain",
            ".",
        ]
        .join(" "),
    );

    shell.run_status(
        &[
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
