{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }:
    let mkShell = system: {
      devShells.default =
        let pkgs = nixpkgs.legacyPackages.${system};
        in pkgs.mkShell {
          packages = with pkgs ; [
            pkgs.cargo
            pkgs.libiconvReal
            pkgs.rust-analyzer
            pkgs.rustc
          ];
        };
    };
    in flake-utils.lib.eachDefaultSystem mkShell;
}
