#!/bin/bash
# Optimize for Hetzner server
# Usage: ./build.sh 

set -e
echo "Running cargo build $@..."
RUSTFLAGS="-C target-cpu=znver2" cargo build --release --bin rs-scrape
