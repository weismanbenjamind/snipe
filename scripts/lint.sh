#!/bin/zsh

exit_code=0

echo Checking format
cargo fmt --check
last=$?
if [ $last -ne 0 ]; then
    exit_code=$last
fi
echo Done checking format

echo Running clippy on debug build artifact
./scripts/clippy_debug.sh
last=$?
if [ $last -ne 0 ]; then
    exit_code=$last
fi
echo Done running clippy on debug build artifact

echo Running clippy on release build artifact
./scripts/clippy_release.sh
last=$?
if [ $last -ne 0 ]; then
    exit_code=$last
fi
echo Done running clippy on release build artifact

exit $exit_code
