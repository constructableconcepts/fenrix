#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

echo "--- Building entire Fenrix project and examples ---"

# Use cargo to build the entire workspace.
# The --all-targets flag ensures that we build libraries, binaries, tests, and examples.
cargo build --workspace --all-targets

echo "--- Build complete! ---"