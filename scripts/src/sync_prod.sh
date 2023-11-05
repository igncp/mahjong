# It could be added to the CI but in some cases it will not be needed to clean
# the database, so running it manually. It can be moved to the scripts folder.

# This script clears the database completely in prod, it is a temporaly
# approach until the whole project is stable
run_sync_prod() {
  IS_DB_RUNNING=$(docker ps | grep mahjong_db || true)

  if [ -z "$IS_DB_RUNNING" ]; then
    (cd service && bash scripts/local_db.sh)
    sleep 10
  fi

  docker exec mahjong_db psql -U postgres \
    -c 'DROP DATABASE IF EXISTS mahjong_prod' \
    -c 'CREATE DATABASE mahjong_prod'

  (cd service && DATABASE_URL='postgres://postgres:postgres@localhost/mahjong_prod' diesel setup)

  local ADMIN_PASS=$(ssh mahjong-rust.com "cat .env | grep MAHJONG_ADMIN_PASS | cut -d '=' -f2" | tr -d '\r')
  local SALT=$(uuidgen)
  local HASH=$(echo -n "$ADMIN_PASS" | argon2 $SALT -t 1 -e)
  local USER_ID=$(uuidgen)
  local ROLE='"Admin"'

  cat <<EOF >/tmp/mahjong_query.sql
INSERT INTO player (id, is_ai, name, created_at) VALUES \
  ('$USER_ID', '0', 'Admin', NOW());

INSERT INTO auth_info (user_id, role, provider) VALUES ('$USER_ID', '$ROLE', 'email');

INSERT INTO auth_info_email (user_id, username, hashed_pass) VALUES
  ('$USER_ID', 'admin',  '$HASH');
EOF

  cat /tmp/mahjong_query.sql | docker exec -i mahjong_db psql -U postgres -d mahjong_prod
  rm -rf /tmp/mahjong_query.sql

  docker exec mahjong_db pg_dump -U postgres -d mahjong_prod >/tmp/mahjong_prod.sql
  scp /tmp/mahjong_prod.sql mahjong-rust.com:data/mahjong_prod.sql
  rm -rf /tmp/mahjong_prod.sql

  (cd scripts && scp docker-compose.yml mahjong-rust.com:)
  (cd scripts && scp -r sql-queries mahjong-rust.com:)

  # If not running in a file and passing it as a here document, the script will
  # stop after the SQL query, so it is better to create a file and run it
  cat >/tmp/mahjong_restart.sh <<"EOF"
set -e pipefail
docker compose pull
docker compose down
# This needs an entry in sudoers (and special permissions to the file)
# - sudoers: mahjong ALL=(ALL) NOPASSWD: /home/mahjong/rm_volume.sh
# - script: chown root:root /home/mahjong/rm_volume.sh
sudo /home/mahjong/rm_volume.sh
docker compose up -d db
sleep 5
(docker compose exec -i db psql -U postgres \
  -c 'DROP DATABASE IF EXISTS mahjong;' \
  -c 'CREATE DATABASE mahjong;')
echo "Database created"
cat ./data/mahjong_prod.sql | docker compose exec -i db psql -U postgres -d mahjong  > /dev/null 2>&1
docker compose up -d --quiet-pull
docker system prune -fa
rm -rf data/mahjong_prod.sql
rm -rf /tmp/mahjong_restart.sh
echo "SSH finished"
EOF
  scp /tmp/mahjong_restart.sh mahjong-rust.com:/tmp
  ssh mahjong-rust.com "bash /tmp/mahjong_restart.sh"

  echo "Synced prod"
}
