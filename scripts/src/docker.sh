set -e

DOCKER_IMAGE_TAG=$(uname -m)

run_docker() (
  web() {
    run_pack_wasm

    (cd ts_sdk && bun run sync_sdk)

    (cd web_client && bun run build)
  }

  docker_service() {
    (
      cd service
      cargo build --release --target-dir target
      patchelf --set-interpreter /lib/x86_64-linux-gnu/ld-linux-$(uname -m).so.2 \
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

  web
  docker_front
  docker_service
)
