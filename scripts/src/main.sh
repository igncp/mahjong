#!/usr/bin/env bash

set -e

. ./src/check.sh
. ./src/docker.sh
. ./src/gh.sh
. ./src/install.sh
. ./src/tests_summaries_fix.sh
. ./src/wasm.sh

SCRIPTPATH="$(
  cd -- "$(dirname "$0")" >/dev/null 2>&1
  pwd -P
)"
cd "$SCRIPTPATH/../.."

USAGE="bash src/main.sh <command>
Run various scripts for the Mahjong project
  - check: Run all checks
  - check_docker: Run all checks inside docker
  - clippy: Run only clippy checks
  - dev_install: Install some dependencies for development (alias: install_dev)
  - docker: Build docker images
  - fix: Run linters in fix mode
  - gh_checks: Triggers a manual check in GitHub Actions
  - gh_deploy: Triggers a manual deployment in GitHub Actions
  - list: List root files to be used in a pipe
  - pack_wasm: Pack the wasm files
  - tests_summaries_fix: Convert the tests summaries to chinese chars"

# This is specially convenient for maintaining the clippy rules, which need to
# be in each crate
list() {
  FILES=(
    "../mahjong_core/src/lib.rs"
    "../service/src/main.rs"
    "../service_contracts/src/lib.rs"
    "../tui_client/src/main.rs"
    "../web_lib/src/lib.rs"
  )
}

main() {
  case "$1" in
  check)
    run_check
    ;;
  check_docker)
    run_check_docker
    ;;
  clippy)
    run_clippy
    ;;
  docker)
    run_docker
    ;;
  fix)
    run_fix
    ;;
  list)
    list
    ;;
  pack_wasm)
    run_pack_wasm
    ;;
  gh_deploy)
    gh_deploy
    ;;
  gh_checks)
    gh_checks
    ;;
  dev_install | install_dev)
    run_setup_dev_install
    ;;
  tests_summaries_fix)
    tests_summaries_fix
    ;;
  *)
    echo "$USAGE"
    exit 1
    ;;
  esac
}

main "$@"
