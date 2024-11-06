{ pkgs, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "tan";
  version = "0.1.0";

  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;

  # src = fetchFromGitHub {
  #   owner = "vglfr";
  #   repo = pname;
  #   rev = "master";
  #   hash = "sha256-";
  # };

  # cargoHash = "sha256-";
}
