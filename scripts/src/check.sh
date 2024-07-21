#!/usr/bin/env bash

set -e

run_clippy() {
  cargo clippy --release --all-features --all-targets -- -D warnings
}

run_fix() {
  cd web_client
  bun run lint:fix
}

run_check() {
  cargo check --workspace --release --all-targets
  cargo build --release
  cargo test --release

  run_clippy

  (cd service && sqlfluff lint --dialect postgres migrations/**/*.sql)

  run_pack_wasm

  cargo run --release --bin mahjong_cli -- simulate -o

  (cd web_client &&
    bun install &&
    bash scripts/format_bindings.sh &&
    bun run lint &&
    bun run test &&
    bun run build)

  echo "All checks passed"
}
