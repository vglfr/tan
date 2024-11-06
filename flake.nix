{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }:
    let mkShell = system: {
      devShells.default =
        let pkgs = nixpkgs.legacyPackages.${system};
            libs = [ pkgs.libz pkgs.stdenv.cc.cc.lib ];

        in pkgs.mkShell {
          packages = [
            pkgs.cargo
            pkgs.clippy
            pkgs.libiconvReal
            pkgs.pre-commit
            pkgs.python312
            pkgs.rust-analyzer
            pkgs.rustc
            pkgs.rustfmt
          ];

          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath libs}:$LD_LIBRARY_PATH"
            source .venv/bin/activate
          '';
        };
    };
    in flake-utils.lib.eachDefaultSystem mkShell;
}
