#!/bin/sh

if [ ! -d "venv" ]; then
    python -m venv venv
fi
venv/Scripts/pip install texttest
venv/Scripts/texttest -d . -con "$@"
