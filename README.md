# 麻雀 - Mahjong

- ![Check](https://github.com/github/docs/actions/workflows/check.yml/badge.svg)

Proof of concept project to practice Rust programming. The idea is to include:

- Core library with the game utilities and tests
- Service that persists and handles games for clients
    - Can be communicated via an HTTP API and potentially gRPC
- TUI client and CLI application
- Web client using websockets

The game uses Hong Kong style mahjong. The goal is to write it as much as
possible in Rust, except the web client frontend where it uses TypeScript.

> This is WIP, the subprojects will remain v0.x until it reaches the MVP state
