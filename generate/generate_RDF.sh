#!/bin/bash

# Assign arguments to variables
command_name="$1"
arg1="$2"
arg2="$3"
arg3="$4"

# Run the cargo command
cargo run --release $command_name $arg1 $arg2 $arg3
