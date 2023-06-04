#!/usr/bin/env bash

set -e

rm -rf flamegraph.svg
rm -rf perf.data

flamegraph --freq 10 -- cargo run -- simulate
