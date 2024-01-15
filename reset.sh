#!/bin/bash

# Specify the paths to the folders you want to delete
RESULTS_FOLDER="./results"
ZARR_FILES_FOLDER="./zarr-files"

# Check if the folders exist before attempting to delete them
if [ -d "$RESULTS_FOLDER" ] || [ -d "$ZARR_FILES_FOLDER" ]; then
    # Delete the results folder and its contents
    rm -rf "$RESULTS_FOLDER"

    # Delete the zarr-files folder and its contents
    rm -rf "$ZARR_FILES_FOLDER"

    echo "Folders deleted successfully."
else
    echo "Error: One or both folders do not exist."
fi
