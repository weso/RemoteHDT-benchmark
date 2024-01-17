#!/bin/bash

# Check if Cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Cargo is not installed. Please install Rust and Cargo before running this script."
    exit 1
fi

# Check if the correct number of arguments is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: cargo run <number_of_iterations>"
    exit 1
fi

# Assign arguments to variables

arg1="$1"


# Run the cargo command
cargo run --release $arg1 
