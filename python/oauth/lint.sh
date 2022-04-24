#!/usr/bin/env sh

isort --check app.py && black --check --line-length 79 app.py && flake8 --extend-ignore E501 app.py && mypy app.py 
