#!/usr/bin/env bash

set -e

E2E_BASE_URL=${E2E_BASE_URL:-http://localhost:5000}
ID=$(id -u)

docker run -it \
  --rm --net=host \
  -u $ID \
  -v $(pwd)/..:/app \
  -w /app \
  mcr.microsoft.com/playwright:v1.37.0-jammy \
    /bin/bash \
    -c \
    "cd web_client && E2E_BASE_URL=$E2E_BASE_URL ./node_modules/.bin/playwright test tests/"
