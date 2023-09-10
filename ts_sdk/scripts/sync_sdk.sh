#!/usr/bin/env bash

set -e

bun i

bun run build

mkdir -p ../mobile_apps/ts_sdk/dist ../web_client/ts_sdk/dist

rsync -avz --delete ./dist/ ../mobile_apps/ts_sdk/dist/
rsync -avz --delete ./dist/ ../web_client/ts_sdk/dist/

cp package.json ../mobile_apps/ts_sdk/
cp package.json ../web_client/ts_sdk/

(cd ../mobile_apps && rm -rf node_modules/ts_sdk && bun i)
(cd ../web_client && rm -rf node_modules/ts_sdk && bun i)
