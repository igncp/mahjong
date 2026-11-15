{pkgs, ...}: let
  testGhc = pkgs.haskellPackages.ghcWithPackages (packages: [
    packages.hspec
  ]);
  test = pkgs.writeShellApplication {
    name = "test";
    runtimeInputs = [testGhc];
    text = ''
      runhaskell -i. -isrc -itest "$1"
    '';
  };
in {
  test-ghc-package-db = "${testGhc}/lib/ghc-${pkgs.ghc.version}/lib/package.conf.d";
  apps.test = {
    type = "app";
    program = "${test}/bin/test";
  };
  extra-shell-packages = [
    testGhc
    pkgs.cabal-install
    pkgs.fourmolu
    pkgs.haskell-language-server
    pkgs.haskellPackages.hlint
  ];
}
