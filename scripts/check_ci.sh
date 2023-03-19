#!/usr/bin/env bash

set -e

cargo check
cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo test
cargo build --release
