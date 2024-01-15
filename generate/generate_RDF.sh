#!/bin/bash

# Check if Cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Cargo is not installed. Please install Rust and Cargo before running this script."
    exit 1
fi

# Check if the correct number of arguments is provided
if [ "$#" -ne 4 ]; then
    echo "Usage: cargo run <file_name> <n_nodes> <n_predicates> <n_triples>"
    exit 1
fi

# Assign arguments to variables
command_name="$1"
arg1="$2"
arg2="$3"
arg3="$4"

# Remove the 'outputs' folder
if [ -d "outputs" ]; then
    echo "Removing 'outputs' folder..."
    rm -r outputs
fi

# Run the cargo command
cargo run $command_name $arg1 $arg2 $arg3
