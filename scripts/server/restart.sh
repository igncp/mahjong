set -e
set -x

docker compose pull
docker compose up -d
docker system prune -fa
