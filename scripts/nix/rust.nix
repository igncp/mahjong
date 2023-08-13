{
  pkgs,
  system,
}: let
  service_manifest = (pkgs.lib.importTOML ../../service/Cargo.toml).package;
  scripts_manifest = (pkgs.lib.importTOML ../../scripts/Cargo.toml).package;
in {
  extra-shell-packages = with pkgs; [
    sqlite
    openssl
    wasm-pack
    diesel-cli
    cargo-flamegraph
    rustup
  ];
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
