#!/usr/bin/env bash

# This is a temporary script until there is a proper deployment pipeline (which
# would sync other files like `docker-compose.yml` or `httpd.conf`) and
# database migrations.
#
# It could be added to the CI but in some cases it will not be needed to clean
# the database, so running it manually. It can be moved to the scripts folder.

rm -rf mahjong_prod.db

DATABASE_URL=sqlite://mahjong_prod.db diesel setup

scp mahjong_prod.db mahjong-rust.com:data/mahjong_prod.db

ssh mahjong-rust.com << EOF
# It is not necessary to bring all containers down but for now this is fine.
docker compose down
rm -rf data/mahjong.db
cp data/mahjong_prod.db data/mahjong.db
docker compose up -d --quiet-pull
EOF
