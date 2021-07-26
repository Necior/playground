{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    elmPackages.elm
    elmPackages.elm-format
  ];
}

