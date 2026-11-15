set -e

DOCKER_IMAGE_TAG=$(uname -m)

run_docker_prod() {
  export NEXT_PUBLIC_SERVICE_URL=https://mahjong-rust.com/api
  if [ -f scripts/.env ]; then
    export $(cat scripts/.env | xargs)
  fi
  run_docker
}

run_docker() (
  nix_closure() {
    local architecture="$1"
    local platform="$2"
    local system="$3"
    local closure_dir

    closure_dir="build/mahjong-service-closure-$architecture"
    docker run --rm \
      --platform "$platform" \
      -e GIT_CONFIG_COUNT=1 \
      -e GIT_CONFIG_KEY_0=safe.directory \
      -e GIT_CONFIG_VALUE_0=/app \
      -e NIX_PATH= \
      -v "$PWD:/app" \
      nixos/nix:2.32.4 \
      bash -c \
      'git config --system --add safe.directory /app
       cd /app
       output="$(nix build .#container-app --no-link --no-update-lock-file --system "$2" --option build-users-group "" --extra-experimental-features "nix-command flakes" --print-out-paths)"
       rm -rf "$1"
       mkdir -p "$1"
       mapfile -t closure < <(nix-store -qR "$output")
       cp -R "${closure[@]}" "$1/"
       chmod -R a+rw "$1"' \
      -- "/app/$closure_dir" "$system"
  }

  web() {
    if [ -d web_client/out/doc ]; then
      return
    fi

    cargo doc --release --no-deps
    run_pack_wasm

    (cd web_client && bun install && bun run build)
    mv target/doc web_client/out/doc
  }

  docker_service() {
    BRANCH_NAME="$(git rev-parse --abbrev-ref HEAD)"

    if [ -z "$DOCKER_NO_PUSH" ]; then
      nix_closure arm64 linux/arm64 aarch64-linux
      docker build \
        --platform linux/arm64 \
        -f Dockerfile.service \
        -t "public.ecr.aws/u6b3o0i4/mahjong-repo:$BRANCH_NAME" \
        --progress=plain \
        .

      aws ecr-public get-login-password --region us-east-1 |
        docker login --username AWS --password-stdin public.ecr.aws/u6b3o0i4

      docker push \
        "public.ecr.aws/u6b3o0i4/mahjong-repo:$BRANCH_NAME"
    else
      echo "DOCKER_NO_PUSH is set, not pushing"
      case "$(uname -m)" in
      aarch64 | arm64) nix_closure arm64 linux/arm64 aarch64-linux ;;
      x86_64 | amd64) nix_closure amd64 linux/amd64 x86_64-linux ;;
      *)
        echo "Unsupported architecture: $(uname -m)" >&2
        exit 1
        ;;
      esac

      docker build \
        -f Dockerfile.service \
        -t "public.ecr.aws/u6b3o0i4/mahjong-repo:$BRANCH_NAME" \
        --progress=plain \
        .
    fi
  }

  web
  docker_service
)

run_docker_build() (
  cd service

  PLATFORM=$(uname -m)

  echo "PLATFORM: $PLATFORM"

  cargo build --release

  mv \
    ../target/release/mahjong_service \
    ../mahjong_service
)

run_check_docker() {
  cp -r /base /app
  cd /app/base
  cp -r /target .
  cp -r /web_client_modules web_client/node_modules
  cd /app/base
  nix develop path:$(pwd) -c bash -c 'cd scripts && bash src/main.sh check'
}
