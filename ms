#!/usr/bin/env bash

set -e

# ms - Mahjong Scripts
# This script is sourced from the flake

(cd ./scripts && bash src/main.sh "$@")
