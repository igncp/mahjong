#!/usr/bin/env bash

set -e

run_setup_dev_install() {
  (cd scripts && bash src/main.sh pack_wasm)

  cd service

  if [ ! -f .env ]; then
    echo "Missing .env file in the service, copying it"
    cp .env.sample .env
  fi

  if [ -n "$(docker ps | grep mahjong_redis)" ]; then
    docker kill mahjong_redis || true
  fi

  docker run \
    --rm \
    --name mahjong_redis \
    -p 6379:6379 \
    -d \
    redis:7

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

  echo "Waiting for the database to start..."
  sleep 4

  if [ "$1" == "--reset" ]; then
    echo "Resetting database"
    docker exec -it mahjong_db psql -U postgres \
      -c "DROP DATABASE mahjong;" \
      -c "CREATE DATABASE mahjong;"
  fi

  DATABASE_URL=$(grep DATABASE_URL .env | cut -d '=' -f2- | sed 's/"//g')

  echo "DATABASE_URL: $DATABASE_URL"

  DATABASE_URL=$DATABASE_URL diesel setup

  DATABASE_URL=$DATABASE_URL diesel migration run

  cd ../web_client
  bun i

  echo "Setup dev install done"
}
