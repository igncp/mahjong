#!/usr/bin/env bash

set -e

run_pack_wasm() {
  rm -rf web_lib/pkg web_client/pkg
  (cd web_lib && wasm-pack build --release)
  mv web_lib/pkg web_client/pkg
}
