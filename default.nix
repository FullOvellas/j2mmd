{ pkgs ? import <nixpkgs> { } }:
let package = (pkgs.lib.importTOML ./Cargo.toml).package;
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = package.name;
  version = package.version;
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
  meta = with pkgs.lib; {
    description = "m2mmd cli tool";
    mainProgram = "m2mmd";
  };
}
