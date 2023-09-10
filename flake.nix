{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/23.05";
    unstable.url = "github:nixos/nixpkgs/nixos-unstable";
    # Added for `v1.0` until it is available in `nixos-unstable`
    bun.url = "github:nixos/nixpkgs/master";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    nixpkgs,
    bun,
    unstable,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      is-docker-ci = builtins.pathExists ./scripts/nix/is-docker-ci;
      pkgs = import unstable {
        inherit system;
        config = {
          allowUnfree = true;
          android_sdk.accept_license = true;
        };
      };
      pkgs-stable = import nixpkgs {
        inherit system;
      };
      pkgs-bun = import bun {
        inherit system;
      };

      android = import ./scripts/nix/android.nix {inherit pkgs system is-docker-ci;};
      rust = import ./scripts/nix/rust.nix {inherit pkgs system is-docker-ci;};
    in rec {
      devShell = pkgs.mkShell ({
          packages = with pkgs-stable;
            [pkgs-bun.bun pkgs.patchelf]
            ++ (
              if is-docker-ci
              then []
              else with pkgs-stable; [sqlfluff litecli inkscape pkgs.nodejs]
            )
            ++ android.extra-shell-packages
            ++ rust.extra-shell-packages;
        }
        // android.extra-shell);

      packages.mahjong_service = rust.mahjong_service;

      defaultPackage = packages.mahjong_service;
    });
}
