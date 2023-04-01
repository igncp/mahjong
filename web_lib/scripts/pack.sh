#!/usr/bin/env bash

rm -rf pkg ../web_client/pkg

wasm-pack build --release

mv pkg ../web_client/pkg
