#!/usr/bin/env bash

set -e

run_pack_wasm() {
  sudo echo
  rm -rf web_lib/pkg web_client/pkg

  # It is necessary to build with wasm-pack inside Docker in NixOS due to
  # wasm-bindgen not working on NixOS directly, even inside the nix shell
  docker start -ai mahjong-rust ||
    docker run -it \
      -v "$PWD:/data" \
      --name mahjong-rust \
      rust:bookworm \
      bash -c "cd /data && bash scripts/src/docker_build_wasm.sh"

  sudo chown -R $USER web_lib/pkg

  mv web_lib/pkg web_client/pkg
}
