{
  pkgs ? import <nixpkgs> {},
  stdenv ? pkgs.stdenv,
  lib ? stdenv.lib,
  rustPlatform ? pkgs.rustPlatform,
  fetchFromGitHub ? pkgs.fetchFromGitHub,
  gitignoreSrc ? null,
}: let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
  gitignoreSource =
    if gitignoreSrc != null
    then gitignoreSrc.gitignoreSource
    else
      (import (fetchFromGitHub {
        owner = "hercules-ci";
        repo = "gitignore";
        rev = "c4662e662462e7bf3c2a968483478a665d00e717";
        sha256 = "0jx2x49p438ap6psy8513mc1nnpinmhm8ps0a4ngfms9jmvwrlbi";
      }) {inherit lib;})
      .gitignoreSource;
in
  # https://github.com/NixOS/nixpkgs/blob/master/pkgs/build-support/rust/build-rust-package/default.nix
  rustPlatform.buildRustPackage rec {
    pname = manifest.name;
    version = manifest.version;

    src = gitignoreSource ./..;
    cargoLock = {lockFile = ../Cargo.lock;};

    buildInputs = [
      pkgs.openssl
      pkgs.sqlite
    ];
    nativeBuildInputs = [pkgs.pkgconfig];
  }
