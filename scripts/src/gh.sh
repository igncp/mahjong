#!/usr/bin/env bash

set -e

gh_deploy() {
  WORKFLOW_PARAMS=()

  if [ -n "$DEPLOY_SKIP" ]; then
    WORKFLOW_PARAMS+=(-f deploySkip="$DEPLOY_SKIP")
  fi

  if [ -n "$DEPLOY_ONLY" ]; then
    WORKFLOW_PARAMS+=(-f deployOnly="$DEPLOY_ONLY")
  fi

  if [ ${#WORKFLOW_PARAMS[@]} -gt 0 ]; then
    echo "Workflow parameters: ${WORKFLOW_PARAMS[@]}"
  fi

  gh workflow run \
    .github/workflows/deploy.yml \
    "${WORKFLOW_PARAMS[@]}"
}

gh_checks() {
  gh workflow run \
    .github/workflows/checks.yml
}
