#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# Add cargo bin to path to find the cli
export PATH="$HOME/.cargo/bin:$PATH"

# Ensure fenrix-cli is installed
if ! command -v cargo-fenrix &> /dev/null
then
    echo "cargo-fenrix could not be found. Please install it first by running:"
    echo "cargo install --path crates/fenrix-cli"
    exit 1
fi

# Create a directory for screenshots if it doesn't exist
mkdir -p screenshots

# Define the list of examples to test.
EXAMPLES=(
    "data-binding"
    "di-example"
    "hello-world"
    "routing-example"
    "server-function-example"
    "simple-component"
)

# --- Run tests for each example ---
for example in "${EXAMPLES[@]}"; do
    echo ""
    echo "--- Running test for example: $example ---"

    # Ensure port is free before starting
    echo "Ensuring port 8080 is free..."
    pkill -f 'cargo-fenrix dev' || true
    pkill -f 'server-function-example-server' || true
    sleep 2 # Give a moment for the port to be released

    # Special handling for the server-function-example
    if [ "$example" = "server-function-example" ]; then
        cd "examples/$example"

        echo "Building client for $example..."
        wasm-pack build --target web

        echo "Building server for $example..."
        (cd ../.. && cargo build --bin server-function-example-server --features server)

        echo "Starting dedicated server for $example..."
        ../../target/debug/server-function-example-server >> ../../server.log 2>&1 &
        SERVER_PID=$!

        echo "Waiting for dedicated server to become available..."
        timeout=120
        while ! grep -q "Starting server for server-function-example..." ../../server.log; do
            sleep 1
            timeout=$((timeout-1))
            if [ $timeout -eq 0 ]; then
                echo "Error: Server did not start within 120 seconds."
                echo "--- Server Log ---"
                cat ../../server.log
                echo "------------------"
                kill $SERVER_PID || true
                exit 1
            fi
        done
        echo "Server is up!"

        sleep 1

        echo "Running verification script for $example..."
        python "../../scripts/verify_$example.py"

        echo "Stopping dedicated server..."
        kill $SERVER_PID || true
        wait $SERVER_PID || true
        sleep 2

        cd ../..
    else
        # Navigate to the example's directory
        cd "examples/$example"

        # Start the dev server in the background, redirecting output
        echo "Starting dev server for $example..."
        cargo-fenrix dev > ../../server.log 2>&1 &
        SERVER_PID=$!

        # Wait for the server to be ready by watching its log output
        echo "Waiting for dev server to become available..."
        timeout=120 # Increased timeout for wasm-pack build
        while ! grep -q "View your app at http://127.0.0.1:8080" ../../server.log; do
            sleep 1
            timeout=$((timeout-1))
            if [ $timeout -eq 0 ]; then
                echo "Error: Server did not start within 120 seconds."
                echo "--- Server Log ---"
                cat ../../server.log
                echo "------------------"
                pkill -f 'cargo-fenrix dev' || true
                exit 1
            fi
        done
        echo "Server is up!"

        # Give it an extra second just in case
        sleep 1

        # Run the corresponding verification script
        echo "Running verification script for $example..."
        python "../../scripts/verify_$example.py"

        # Kill the server process and any lingering children on the port
        echo "Stopping dev server..."
        pkill -f 'cargo-fenrix dev' || true
        wait $SERVER_PID || true
        sleep 2

        # Navigate back to the root directory
        cd ../..
    fi

    echo "--- Test for $example complete! ---"
done

echo ""
echo "--- All system tests passed successfully! ---"