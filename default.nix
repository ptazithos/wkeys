{ pkgs ? import <nixpkgs> {} }:

pkgs.stdenv.mkDerivation {
  name = "wkeys";
  src = ./.;
  buildInputs = with pkgs; [cargo gtk4 gtk4-layer-shell];
  buildPhase = ''
    cargo build
  '';
}
