#!/usr/bin/env bash

set -e

run_setup_dev_install() {
  (cd service && bash scripts/setup_dev_install.sh)
  (cd scripts && bash src/main.sh pack_wasm)

  echo "Setup dev install done"
}
