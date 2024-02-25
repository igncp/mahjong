#!/usr/bin/env bash

set -e

run_clippy() {
  cargo clippy --release --all-features -- -D warnings
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

  (cd service && sqlfluff lint --dialect postgres migrations/**/*.sql)

  run_pack_wasm

  (cd web_client &&
    bun install &&
    bun run lint &&
    bun run test &&
    bun run build)

  echo "All checks passed"
}
