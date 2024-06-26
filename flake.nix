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
      is-docker-ci = builtins.pathExists ./scripts/nix/is-docker-ci;
      is-checks-ci = builtins.pathExists ./scripts/nix/is-checks-ci;
      pkgs = import unstable {
        inherit system;
      };

      rust = import ./scripts/nix/rust.nix {inherit pkgs system is-docker-ci is-checks-ci;};
    in {
      devShell = pkgs.mkShell {
        shellHook =
          ''
            export PATH=$PATH:$HOME/.cargo/bin
            export PATH=$PATH:$PWD/cli
          ''
          + (
            if (is-docker-ci || is-checks-ci)
            then ""
            else rust.dev-hook
          );
        packages = with pkgs;
          [bun patchelf postgresql nodejs_22] # without nodejs_22 prettier throws an error
          ++ (
            if is-docker-ci
            then []
            else [sqlfluff]
          )
          ++ (
            if is-docker-ci == false && is-checks-ci == false
            then [libargon2]
            else []
          )
          ++ rust.extra-shell-packages;
      };
    });
}
