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
      docker buildx build \
        --platform linux/arm64 \
        -f Dockerfile.service \
        -t "igncp/mahjong_service:$BRANCH_NAME" \
        --push \
        --progress=plain \
        .

      if [ "$NO_K8S_DEPLOY" != "1" ]; then
        docker_deploy
      fi
    else
      echo "DOCKER_NO_PUSH is set, not pushing"

      docker build \
        -f Dockerfile.service \
        -t "igncp/mahjong_service:$BRANCH_NAME" \
        --progress=plain \
        .
    fi
  }

  docker_deploy() {
    DEPLOYMENT_NAME="mahjong-server"
    MISSING_ENVS=""

    if [ -z "$DEPLOYMENT_LOCATION" ]; then
      echo "DEPLOYMENT_LOCATION is not set"
      MISSING_ENVS="true"
    fi

    if [ -z "$DEPLOYMENT_TOKEN" ]; then
      echo "DEPLOYMENT_TOKEN is not set"
      MISSING_ENVS="true"
    fi

    if [ -n "$MISSING_ENVS" ]; then
      exit 1
    fi

    curl "$DEPLOYMENT_LOCATION/apis/apps/v1/namespaces/default/deployments/$DEPLOYMENT_NAME" \
      -i \
      --insecure \
      --silent \
      --show-error \
      -X PATCH \
      -H "Authorization: Bearer $DEPLOYMENT_TOKEN" \
      -H "Accept: application/json" \
      -H "Content-Type: application/strategic-merge-patch+json" \
      --data "@-" <<EOF
{
  "spec": {
    "template": {
      "metadata": {
        "annotations": {
          "$DEPLOYMENT_NAME/restartedAt": "$(date +%Y-%m-%d_%T%Z)"
        }
      }
    }
  }
}
EOF

    echo ''
    echo "Redeployed $DEPLOYMENT_NAME"
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
