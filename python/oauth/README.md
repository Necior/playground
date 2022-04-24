# OAuth playground

A demo showing how to use OAuth 2.0 _without_ OAuth-related libraries.

This demo uses GitHub OAuth.

## Running

Assuming you're using NixOS (or at least Nix, the package manager):

```sh
$ cp config.py.sample config.py
$ # edit `config.py`
$ nix-shell --run ./app.py
```

## Development

```sh
$ nix-shell # enter a new shell with all needed software
$ # edit app.py
$ ./lint.sh # basic linting
```
