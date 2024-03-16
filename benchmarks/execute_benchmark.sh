#!/bin/bash

# Check if Cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Cargo is not installed. Please install Rust and Cargo before running this script."
    exit 1
fi

# Assign arguments to variables
arg1="$1"
arg2="$2"

# Run the cargo command
cargo run --release -- -i  $arg1 -f ../zarr-files/*.zarr --context $arg2