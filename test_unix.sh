#!/bin/sh
set -e

# Test no_std builds
cargo test --no-default-features --features=""
cargo test --no-default-features --features="" --release

# Test std builds
cargo test --no-default-features --features="std"
cargo test --no-default-features --features="std" --release