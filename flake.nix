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
             pkgs.haskellPackages.hoogle
             pkgs.haskell-language-server
             pkgs.haskell.compiler.ghc96
           ];
         };
    };
    in flake-utils.lib.eachDefaultSystem mkShell;
}
