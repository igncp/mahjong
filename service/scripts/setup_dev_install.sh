#!/usr/bin/env bash

set -e

if [ ! -f .env ]; then
  echo "Missing .env file in the service, copying it"
  cp .env.sample .env
fi

if [ -z "$(docker ps | grep mahjong_db || true)" ]; then
  bash scripts/local_env.sh
  sleep 6
fi

if [ "$RESET_DB" == "true" ]; then
  echo "Resetting database"
  docker exec -it mahjong_db psql -U postgres \
    -c "DROP DATABASE mahjong;" \
    -c "CREATE DATABASE mahjong;"
fi

DATABASE_URL=$(grep DATABASE_URL .env | cut -d '=' -f2- | sed 's/"//g')

DATABASE_URL=$DATABASE_URL diesel setup

DATABASE_URL=$DATABASE_URL diesel migration run
