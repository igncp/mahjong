#!/bin/bash

set -e

profile_instruments() {
  cd cli
  cargo instruments -t time --release -- simulate -o
}
