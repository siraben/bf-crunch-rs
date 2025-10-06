{
  description = "BF Crunch in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rustPlatform = pkgs.rustPlatform;
        src = builtins.path { path = ./.; name = "bf-crunch-src"; };
        lib = pkgs.lib;
      in
      {
        packages.default = rustPlatform.buildRustPackage {
          pname = "bf-crunch";
          version = "0.1.0";
          src = src;
          cargoLock.lockFile = ./Cargo.lock;
          meta = {
            description = "BF Crunch in Rust";
            platforms = lib.platforms.unix;
            maintainers = [ lib.maintainers.siraben ];
            mainProgram = "bfcrunch";
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.rustc
            pkgs.cargo
            pkgs.rustfmt
            pkgs.clippy
          ];
        };
      }
    );
}
