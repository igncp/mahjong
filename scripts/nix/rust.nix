{
  pkgs,
  system,
  ...
}: {
  extra-shell-packages = with pkgs;
    [
      cargo
      cargo-flamegraph
      cargo-watch
      clippy
      diesel-cli
      openssl
      pkg-config
      rust-analyzer
      rustfmt
      rustc
      wasm-bindgen-cli
      wasm-pack
    ]
    ++ (
      if system == "aarch64-darwin"
      then [
        libiconv
        darwin.apple_sdk.frameworks.Security
      ]
      else []
    );
}
