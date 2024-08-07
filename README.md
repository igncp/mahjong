# 麻雀 - Mahjong

![Checks](https://github.com/igncp/mahjong/actions/workflows/checks.yml/badge.svg) ![Deploy](https://github.com/igncp/mahjong/actions/workflows/deploy.yml/badge.svg)

Hong Kong style mahjong game engine and web application. It uses a server to
handle games which handles most of the game mechanics.

1. Core library with the game mechanics, tests, and AIs focused in performance and correctness
1. Service that persists and handles games for clients
    - Can be communicated via an HTTP API
    - It creates a WebSocket per active game, to process the game and communicate with clients
1. Scripts to handle different tasks related to the code deployment and tests
    - For example running linting, tests, and deploying to Docker
1. Web client to play, both for desktop and mobile web
    - It uses a drag-n-drop UI to play, and it has translated texts
    - Includes E2E tests
1. A Rust cli for running simulations

You can find the project's Rust documentation [here](https://mahjong-rust.com/doc/mahjong_core).

## Development

The project main dependencies are Rust and Nodejs. There is a
[flake.nix](./flake.nix) file with all the required libraries needed for
development, including android, but it requires that you have the Nix package
manager installed.

Once you have cloned the repository there are a few things to setup:

1. Generate the `.env` files where present by copying the templates
1. Install the dev dependencies: `cd scripts && bash src/main.sh dev_install`
