#!/bin/bash

# Move the files needed to deploy
mv ../zarr-files .
-simulation/

# Stop and remove the existing container
docker stop nginx-zarr-container
docker rm nginx-zarr-container

# Build the Docker image
docker build -t nginx-zarr-server .

# Run the Docker container in detached mode, mapping port 8080 on the host to port 80 in the container
docker run -d -p 8080:80 --name nginx-zarr-container nginx-zarr-server

# Revert the changes
mv ./zarr-files ../.