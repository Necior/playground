{ pkgs ? import (builtins.fetchTarball {
  url = "https://github.com/NixOS/nixpkgs/archive/80c94d2204d3617b9d098a439ed3c5179e311d9b.tar.gz";
  sha256 = "1646rh19yb2r39w1k51a3s1ya1aj3ix84rid4pzyz2cq5b84rva4";
}) { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # runtime
    python39
    python39Packages.flask
    python39Packages.requests

    # dev
    python39Packages.black
    python39Packages.flake8
    python39Packages.isort
    python39Packages.mypy
    python39Packages.types-requests
  ];
}
