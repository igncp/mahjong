#!/usr/bin/env bash

set -e

SCRIPTPATH="$(
  cd -- "$(dirname "$0")" >/dev/null 2>&1
  pwd -P
)"
cd "$SCRIPTPATH/.."

. ./src/check.sh
. ./src/dev.sh
. ./src/docker.sh
. ./src/gh.sh
. ./src/profile.sh
. ./src/service_watch.sh
. ./src/tests_summaries_fix.sh
. ./src/wasm.sh

cd "$SCRIPTPATH/../.."

USAGE="bash src/main.sh <command>
Run various scripts for the Mahjong project
  - check: Run all checks
  - check_docker: Run all checks inside docker
  - clippy: Run only clippy checks
  - count_lines: Count the lines of code
  - dev_be: Run the backend in dev mode
  - dev_fe: Run the frontend in dev mode
  - dev_install: Install some dependencies for development (alias: install_dev)
  - docker: Build docker images
  - docker_build: Script to build the code, is run inside docker
  - docker_prod: Build docker images for production, locally
  - fix: Run linters in fix mode
  - pack_wasm: Pack the wasm files (alias: wasm)
  - profile_instruments: Create a trace file to be inspected by Instruments
  - test: Runs tests plus formatting
  - tests_summaries_fix: Convert the tests summaries to chinese chars"

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
  docker_build)
    run_docker_build
    ;;
  docker_prod)
    run_docker_prod
    ;;
  fix)
    run_fix
    ;;
  wasm | pack_wasm)
    run_pack_wasm
    ;;
  dev_install | install_dev)
    run_setup_dev_install "$2"
    ;;
  dev_fe)
    run_dev_fe
    ;;
  dev_be)
    run_dev_be
    ;;
  tests_summaries_fix)
    tests_summaries_fix
    ;;
  profile_instruments)
    profile_instruments
    ;;
  service_watch)
    service_watch
    ;;
  test)
    run_test
    ;;
  count_lines)
    count_lines
    ;;
  *)
    echo "$USAGE"
    exit 1
    ;;
  esac
}

main "$@"
