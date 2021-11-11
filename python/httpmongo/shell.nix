{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    gnumake
    python39Packages.black
    python39Packages.flake8
    python39Packages.flask
    python39Packages.gunicorn
    python39Packages.mypy
    python39Packages.pymongo
  ];
}

