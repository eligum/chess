#!/bin/sh

# Exit immediately if any command fails
set -e

echo "Building..."
cargo build --package app

BINARY_NAME="chess"
BINARY_PATH="target/debug/$BINARY_NAME"
PROJECT_ROOT=`pwd`

if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: Executable not found at $BINARY_PATH" >&2
    exit 1
fi

echo "Copying executable to project root..."
cp "$BINARY_PATH" "$PROJECT_ROOT/$BINARY_NAME"

echo "Running executable..."
$PROJECT_ROOT/$BINARY_NAME
