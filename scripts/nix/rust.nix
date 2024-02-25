{
  pkgs,
  system,
  is-checks-ci,
  is-docker-ci,
}: {
  dev-hook = ''
    PATH="$HOME/.rustup/bin:$PATH"

    if [ -z "$(rustup component list | grep analy | grep install || true)" ]; then
      rustup component add rust-analyzer
    fi
  '';
  extra-shell-packages = with pkgs;
    [
      openssl
      pkg-config
      wasm-pack
      rustup
      wasm-bindgen-cli
    ]
    ++ (
      if ((is-docker-ci == true) || (is-checks-ci == true))
      then []
      else [
        curl
        cargo-flamegraph
        diesel-cli
      ]
    )
    ++ (
      if system == "aarch64-darwin"
      then [
        libiconv
        darwin.apple_sdk.frameworks.Security
      ]
      else []
    );
}
