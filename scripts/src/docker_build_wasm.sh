#!/usr/bin/env bash

set -e

mkdir -p /target

export CARGO_TARGET_DIR=/target

if ! type -P wasm-pack >/dev/null 2>&1; then
  apt update
  apt install -y libc6

  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

cd web_lib

wasm-pack build
