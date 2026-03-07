#!/bin/bash

# Build script that runs cargo fmt and cargo clippy before building
# Usage: ./build.sh [cargo build args]

set -e

echo "Running cargo fmt..."
cargo fmt

echo "Running cargo clippy..."
#cargo clippy

echo "Running cargo build $@..."
cargo build "$@"
