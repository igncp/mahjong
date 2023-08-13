# 麻雀 - Mahjong

![Checks](https://github.com/igncp/mahjong/actions/workflows/checks.yml/badge.svg) ![Deploy](https://github.com/igncp/mahjong/actions/workflows/deploy.yml/badge.svg)

Proof of concept project to practice Rust and TypeScript programming. The idea is to include
these subpackages:

1. Core library with the game utilities and tests
1. Service that persists and handles games for clients
    - Can be communicated via an HTTP API
    - It creates a WebSocket per active game, to process the game and communicate with clients
    - It has a GraphQL endpoint to reduce the number of requests
1. Scripts to handle different tasks related to the code deployment and tests
    - For example running linting, tests, and deploying to Docker
1. Web client to play, both for desktop and mobile web
    - It uses a drag-n-drop UI to play, and it has translated texts
    - Includes E2E tests
1. React Native apps which reuse some code with the web code
1. A Rust TUI application as a prototype for playing games and running simulations

The game uses Hong Kong style Mahjong. The goal is to write it as much as
possible in Rust, except the web client frontend where it uses TypeScript.

> This is WIP, the subprojects will remain v0.x until the project reaches the MVP state

## Development

The project main dependencies are Rust and Nodejs. There is a
[flake.nix](./flake.nix) file with all the required libraries needed for
development, including android, but it requires that you have the Nix package
manager installed.

Once you have cloned the repository there are a few things to setup:

1. Generate the `.env` files where present by copying the templates
1. Install the dev dependencies: `cd scripts && cargo run -- dev_install`
