{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };
      in rec {
        packages = {
          frontend = pkgs.buildNpmPackage {
            pname = "frontend";
            version = "0.1.0";

            src = ./frontend;
            npmDepsHash = "sha256-tIiX0qfnQIaLfc/LGj8QOwmLcGykmWmzXZsQtLKZ/eo=";

            nativeBuildInputs = with pkgs; [
              tailwindcss
            ];

            npmBuildScript = "build";
          };
          backend = pkgs.rustPlatform.buildRustPackage {
            pname = "backend";
            version = "0.1.0";

            src = ./backend;

            cargoLock = {
              lockFile = ./backend/Cargo.lock;
            };

            nativeBuildInputs = with pkgs; [
              pkg-config
              makeWrapper
            ];
          };

          default = self.packages.${system}.backend;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [packages.default];

          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          packages = with pkgs; [
            clippy
            rustup

            tailwindcss
            nodejs_23
          ];
        };
      }
    );
}