#!/usr/bin/env bash

set -e

# In some environments can't use `cargo watch`
service_watch() {
  cd service

  find src ../mahjong_core/src ../service_contracts/src \
    -type f |
    entr -r timeout -k 0.5 0 cargo run
}
