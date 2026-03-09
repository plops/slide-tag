#!/bin/bash

# Build script that runs cargo fmt and cargo clippy before building
# Usage: ./build.sh [cargo build args]

set -e

# Get number of threads from environment variable or default to 4
THREADS=${THREADS:-17}

echo "Running cargo fmt..."
cargo fmt 

echo "Running cargo clippy..."
cargo clippy -j $THREADS --fix --allow-dirty

echo "Running cargo build $@..."
cargo build -j $THREADS "$@"
