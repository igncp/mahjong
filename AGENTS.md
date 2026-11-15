# Shared Test Fixtures

`core_spec` is the authoritative specification for Mahjong game mechanics. Keep it independent of the Rust implementation; `mahjong_core` must match this specification.

Shared notation fixtures live in `core_spec/test/fixtures/`. Each fixture has its own directory containing `input.txt`, parsed and rendered by the Haskell specification, and `output.txt`, the expected rendered output. Rust tests should consume these same files when equivalent coverage is added. Name fixture directories with a two-digit case suffix, such as `discard_ClaimedAnotherTile_01`; add cases by incrementing that suffix.
