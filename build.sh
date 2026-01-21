#!/bin/bash

set -e

# Create build directory
mkdir -p build

# Build the Docker image
echo "Building Docker image..."
docker build -t quantframe-react:build .

# Create a container and copy the build output
echo "Extracting build artifacts..."
CONTAINER_ID=$(docker create quantframe-react:build)

# Copy all files from build-output directory
docker cp $CONTAINER_ID:/app/build-output/. ./build/

# Remove the container
docker rm $CONTAINER_ID

echo "Build artifacts extracted to ./build/"
echo "Contents:"
ls -lh ./build/
