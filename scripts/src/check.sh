#!/usr/bin/env bash

set -e

run_clippy() {
  cargo clippy --release --all-features --all-targets -- -D warnings
}

run_fix() {
  cd web_client
  sed -i 's|key: |key in |' bindings/DrawWall.ts
  bun run lint:fix
}

run_check() {
  cargo check --workspace --release --all-targets
  cargo build --release
  cargo test --release

  run_clippy

  cargo doc --release --no-deps

  (cd service && sqlfluff lint --dialect postgres migrations/**/*.sql)

  run_pack_wasm

  cargo run --release --bin mahjong_cli -- simulate -o

  (cd web_client &&
    bun install &&
    run_fix &&
    bun run test &&
    bun run build)

  echo "All checks passed"
}

count_lines() {
  scc \
    service/src \
    service/migrations \
    service_contracts/src \
    cli/src \
    web_client/src \
    web_lib/src \
    scripts/src \
    mahjong_core/src
}

run_test() {
  rm -rf web_client/bindings
  RESULT=$(cargo test --all-targets || echo "error")
  run_fix >/dev/null 2>&1
  if [[ "$RESULT" = "error" ]]; then
    exit 1
  fi
}
