#!/usr/bin/env bash

set -e

SCRIPTPATH="$(
  cd -- "$(dirname "$0")" >/dev/null 2>&1
  pwd -P
)"
cd "$SCRIPTPATH"

# ms - Mahjong Scripts
# This script is sourced from the flake

(cd ./scripts && bash src/main.sh "$@")
