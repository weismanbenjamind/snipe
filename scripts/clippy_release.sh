#!/bin/zsh

RUSTFLAGS="-D warnings" cargo clippy --release
