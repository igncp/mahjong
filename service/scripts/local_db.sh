#!/usr/bin/env bash

set -e

if [ -n "$(docker ps | grep mahjong_db)" ]; then
  docker kill mahjong_db || true
fi

docker run \
  --rm \
  --name mahjong_db \
  -d \
  -v $(pwd)/mahjong_db:/var/lib/postgresql/data \
  -p 5432:5432 \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=mahjong \
  postgres:16
