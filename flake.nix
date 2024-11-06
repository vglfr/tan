{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }:
    let mkShell = system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in {
        defaultPackage =
          pkgs.rustPlatform.buildRustPackage rec {
            pname = "tan";
            version = "0.1.0";
            cargoLock.lockFile = ./Cargo.lock;
            src = pkgs.lib.cleanSource ./.;
          };

        devShells.default =
          pkgs.mkShell {
            packages = [
              pkgs.cargo
              pkgs.clippy
              pkgs.libiconvReal
              pkgs.pre-commit
              pkgs.rust-analyzer
              pkgs.rustc
              pkgs.rustfmt
            ];
          };
      };
    in flake-utils.lib.eachDefaultSystem mkShell;
}
