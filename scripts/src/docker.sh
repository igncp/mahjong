set -e

DOCKER_IMAGE_TAG=$(uname -m)

run_docker() (
  web() {
    run_pack_wasm

    (cd web_client && bun install && bun run build)
  }

  docker_service() {
    (
      cd service
      cargo build --release --target-dir target
      patchelf --set-interpreter /lib/x86_64-linux-gnu/ld-linux-$(uname -m | sed 's|_|-|g').so.2 \
        ./target/release/mahjong_service
    )

    docker build \
      -t "igncp/mahjong_service:$DOCKER_IMAGE_TAG" \
      -f scripts/Dockerfile.service \
      --progress=plain \
      .

    docker image push \
      "igncp/mahjong_service:$DOCKER_IMAGE_TAG"
  }

  docker_front() {
    docker build \
      -t "igncp/mahjong_front:$DOCKER_IMAGE_TAG" \
      -f scripts/Dockerfile.front \
      --progress=plain \
      .

    docker image push \
      igncp/mahjong_front:$DOCKER_IMAGE_TAG
  }

  echo "DEPLOY_SKIP='$DEPLOY_SKIP'"
  echo "DEPLOY_ONLY='$DEPLOY_ONLY'"

  if [ -z "$(echo "$DEPLOY_SKIP" | grep 'web' || true)" ] &&
    [ -z "$(echo "$DEPLOY_ONLY" | grep -v 'web' || true)" ]; then
    web
  fi

  if [ -z "$(echo "$DEPLOY_SKIP" | grep 'front' || true)" ] &&
    [ -z "$(echo "$DEPLOY_ONLY" | grep -v 'front' || true)" ]; then
    docker_front
  fi

  if [ -z "$(echo "$DEPLOY_SKIP" | grep 'service' || true)" ] &&
    [ -z "$(echo "$DEPLOY_ONLY" | grep -v 'service' || true)" ]; then
    docker_service
  fi
)

run_check_docker() {
  cp -r /base /app
  cd /app/base
  cp -r /target .
  cp -r /web_client_modules web_client/node_modules
  cd /app/base
  nix develop path:$(pwd) -c bash -c 'cd scripts && bash src/main.sh check'
}
