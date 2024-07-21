#!/usr/bin/env bash

set -e

sed -i 's|key: |key in |' bindings/DrawWall.ts
bun run lint:fix
