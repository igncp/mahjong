#!/usr/bin/env bash

set -e

run_setup_dev_install() {
  (cd service && bash scripts/setup_dev_install.sh)
  (cd scripts && bash src/main.sh pack_wasm)
  (cd ts_sdk && bun i && bun run sync_sdk)
  (cd mobile_apps && bun run setup_images)

  echo "Setup dev install done"
}
