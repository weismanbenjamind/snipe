#!/bin/zsh

set -e

echo Building debug binary
cargo build
echo Debug build complete

echo Building release binary
cargo build --release
echo Release build complete
