{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/23.05";
    unstable.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    nixpkgs,
    unstable,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
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

      android = import ./scripts/nix/android.nix {inherit pkgs system;};
      rust = import ./scripts/nix/rust.nix {inherit pkgs system;};
    in rec {
      devShell = pkgs.mkShell ({
          packages = with pkgs-stable;
            [nodejs inkscape]
            ++ android.extra-shell-packages
            ++ rust.extra-shell-packages;
        }
        // android.extra-shell);

      packages.mahjong_service = rust.mahjong_service;

      defaultPackage = packages.mahjong_service;
    });
}
