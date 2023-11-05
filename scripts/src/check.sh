#!/usr/bin/env bash

set -e

run_clippy() {
  cargo clippy --all-targets --all-features -- -D warnings
}

run_fix() {
  cd web_client
  bun run lint:fix
}

run_check() {
  cargo build --release
  cargo check --workspace --release
  cargo test --release

  run_clippy

  (cd service && sqlfluff fix --dialect postgres migrations/**/*.sql)

  run_pack_wasm

  (cd ts_sdk && bun run sync_sdk && bun run lint)

  (cd web_client &&
    bun run lint &&
    bun run test &&
    bun run build)

  (cd mobile_apps &&
    bun run typecheck &&
    bun run lint &&
    bun run test)
}
