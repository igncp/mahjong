{
  inputs = {
    unstable.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    unstable,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import unstable {
        inherit system;
      };
      rust = import ./scripts/nix/rust.nix {inherit pkgs system;};
      haskell = import ./scripts/nix/haskell.nix {inherit pkgs;};
    in {
      packages.container-app = pkgs.rustPlatform.buildRustPackage {
        pname = "mahjong-service";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
        cargoBuildFlags = ["-p" "mahjong_service"];
        doCheck = false;
        nativeBuildInputs = [pkgs.pkg-config];
        buildInputs = [pkgs.openssl pkgs.postgresql];

        installPhase = ''
          mkdir -p "$out/bin"
          cp target/${pkgs.stdenv.hostPlatform.rust.rustcTargetSpec}/release/mahjong_service "$out/bin/mahjong_service"
        '';
      };

      devShell = pkgs.mkShell {
        TS_RS_EXPORT_DIR = "../web_client/bindings";
        RUST_BACKTRACE = 1;
        # HLS uses its own GHC package database unless this exposes the test dependencies.
        GHC_PACKAGE_PATH = "${haskell.test-ghc-package-db}";

        shellHook = ''
          export PATH="$PWD:$PATH"
        '';

        packages = with pkgs;
          [
            alejandra
            awscli2
            bun
            entr
            libargon2
            nodejs_22 # without nodejs_22 prettier throws an error
            patchelf
            postgresql
            scc
            shfmt
            sqlfluff
          ]
          ++ rust.extra-shell-packages
          ++ haskell.extra-shell-packages;
      };
      apps = haskell.apps;
    });
}
