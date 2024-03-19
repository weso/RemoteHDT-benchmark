#!/bin/bash

# Specify the paths to the folders you want to delete
RESULTS_FOLDER="./results"
ZARR_FILES_FOLDER="./zarr-files"
NT_FILES="./nt-files"


 # Delete the results folder and its contents
    rm -rf "$RESULTS_FOLDER"

    # Delete the zarr-files folder and its contents
    rm -rf "$ZARR_FILES_FOLDER"

    rm -rf "$NT_FILES"

