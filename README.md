# 麻雀 - Mahjong

![Checks](https://github.com/igncp/mahjong/actions/workflows/checks.yml/badge.svg)

Proof of concept project to practice Rust programming. The idea is to include
these subpackages:

1. Core library with the game utilities and tests
1. Service that persists and handles games for clients
    - Can be communicated via an HTTP API and potentially gRPC
1. CLI application with a TUI to play and debug games
1. Web client using websockets with a better playing experience

The game uses Hong Kong style Mahjong. The goal is to write it as much as
possible in Rust, except the web client frontend where it uses TypeScript.

> This is WIP, the subprojects will remain v0.x until the project reaches the MVP state
