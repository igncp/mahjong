{
  pkgs,
  system,
  is-docker-ci,
}: let
  service_manifest = (pkgs.lib.importTOML ../../service/Cargo.toml).package;
in {
  extra-shell-packages = with pkgs;
    [
      openssl
      wasm-pack
      rustup
    ]
    ++ (
      if is-docker-ci
      then []
      else
        with pkgs; [
          curl
          cargo-flamegraph
          diesel-cli
        ]
    )
    ++ (
      if system == "aarch64-darwin"
      then [
        pkgs.libiconv
        pkgs.darwin.apple_sdk.frameworks.Security
        wasm-bindgen-cli
      ]
      else []
    );
  mahjong_service = pkgs.rustPlatform.buildRustPackage {
    pname = service_manifest.name;
    version = service_manifest.version;

    src = ./../..;
    cargoLock = {lockFile = ../../Cargo.lock;};

    buildInputs = with pkgs; [
      openssl
    ];
    nativeBuildInputs = [pkgs.pkgconfig];
  };
}
