set -e
set -x

docker compose kill server
docker compose kill front
docker system prune -fa
docker compose up --quiet-pull -d
