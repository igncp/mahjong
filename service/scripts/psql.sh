#!/usr/bin/env bash

set -e

docker exec \
  -it \
  -e PAGER=cat \
  mahjong_db \
  psql -U postgres -d mahjong "$@"
