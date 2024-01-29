#!/bin/bash

# Set the path to the execute_benchmark.sh script
EXECUTE_BENCHMARK_SCRIPT="./execute_benchmark.sh"
# Set the path to the benchmarks directory
BENCHMARKS_DIR="./benchmarks"

# Check if the script and directory exist
if [ -f "$EXECUTE_BENCHMARK_SCRIPT" ] && [ -d "$BENCHMARKS_DIR" ]; then
    # Enter the benchmarks directory
    cd "$BENCHMARKS_DIR" || exit


    # Execute the execute_benchmark.sh script with the parameter
    "$EXECUTE_BENCHMARK_SCRIPT" "$1" "$2"

    # Return to the original directory
    cd - || exit
else
    echo "Error: $EXECUTE_BENCHMARK_SCRIPT not found, or $BENCHMARKS_DIR does not exist."
fi
