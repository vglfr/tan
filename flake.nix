{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }:
    let mkShell = system:
      let pkgs = nixpkgs.legacyPackages.${system};
          # rust = makeRustPlatform {
          #   cargo = rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
          #   rustc = rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
          # };
      in {
        defaultPackage = pkgs.callPackage ./tan.nix { };
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

            shellHook = "export PATH=/home/vglfr/.cargo/bin:$PATH";
          };
      };
    in flake-utils.lib.eachDefaultSystem mkShell;
}
