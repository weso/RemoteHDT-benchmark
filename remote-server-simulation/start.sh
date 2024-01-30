#!/bin/bash

# Move the files needed to deploy
mv ../zarr-files .
-simulation/

# Stop and remove the existing container
docker stop nginx-zarr-container
docker rm nginx-zarr-container

docker-compose up -d

# Revert the changes
mv ./zarr-files ../.