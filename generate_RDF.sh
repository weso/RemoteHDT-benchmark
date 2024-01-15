#!/bin/bash

# Set the path to the generate_RDF.sh script
GENERATE_RDF_SCRIPT="./generate_RDF.sh"
# Set the path to the generate directory
GENERATE_DIR="./generate"

# Check if the script and directory exist
if [ -f "$GENERATE_RDF_SCRIPT" ] && [ -d "$GENERATE_DIR" ]; then
    # Enter the generate directory
    cd "$GENERATE_DIR" || exit

    # Specify the four parameters you want to pass to generate_RDF.sh
    PARAM1="$1"
    PARAM2="$2"
    PARAM3="$3"
    PARAM4="$4"

    # Execute the generate_RDF.sh script with parameters
    "$GENERATE_RDF_SCRIPT" "$PARAM1" "$PARAM2" "$PARAM3" "$PARAM4"

    # Return to the original directory
    cd - || exit
else
    echo "Error: $GENERATE_RDF_SCRIPT not found, or $GENERATE_DIR does not exist."
fi
