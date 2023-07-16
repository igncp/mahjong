{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    gitignore = {
      url = "github:hercules-ci/gitignore.nix";
      flake = false;
    };
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-utils,
    ...
  }: let
    pkgs = nixpkgs.legacyPackages.aarch64-linux;
    gitignoreSrc = pkgs.callPackage inputs.gitignore {};
  in rec {
    packages.aarch64-linux.mahjong_service = pkgs.callPackage ./default.nix {inherit gitignoreSrc;};

    legacyPackages = packages;

    defaultPackage.aarch64-linux = packages.aarch64-linux.mahjong_service;
  };
}
