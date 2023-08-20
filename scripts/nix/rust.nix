{
  pkgs,
  system,
}: let
  service_manifest = (pkgs.lib.importTOML ../../service/Cargo.toml).package;
in {
  extra-shell-packages = with pkgs;
    [
      sqlite
      openssl
      wasm-pack
      wasm-bindgen-cli # Required in darwin
      diesel-cli
      cargo-flamegraph
      rustup
      curl
    ]
    ++ (
      if system == "aarch64-darwin"
      then [pkgs.libiconv pkgs.darwin.apple_sdk.frameworks.Security]
      else []
    );
  mahjong_service = pkgs.rustPlatform.buildRustPackage {
    pname = service_manifest.name;
    version = service_manifest.version;

    src = ./../..;
    cargoLock = {lockFile = ../../Cargo.lock;};

    buildInputs = with pkgs; [
      openssl
      sqlite
    ];
    nativeBuildInputs = [pkgs.pkgconfig];
  };
}
