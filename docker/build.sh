#!/bin/bash

set -e

# Create build directory
mkdir -p ../build

# Build the Docker image
echo "Building Docker image..."
docker build -f Dockerfile.fedora -t quantframe-react:build ..

# Create named volumes for caching (important for Rust compilation)
docker volume create quantframe-build-cache 2>/dev/null || true
docker volume create quantframe-cargo-cache 2>/dev/null || true

# Create a container with volumes for caching and copy the build output
echo "Extracting build artifacts..."
CONTAINER_ID=$(docker create \
  -v quantframe-build-cache:/app/target \
  -v quantframe-cargo-cache:/root/.cargo \
  quantframe-react:build)

# Copy all files from build-output directory
docker cp $CONTAINER_ID:/app/build-output/. ../build/

# Remove the container
docker rm $CONTAINER_ID

echo "Build artifacts extracted to ../build/"
echo "Contents:"
ls -lh ../build/
echo ""
echo "Docker volumes for caching:"
echo "  - quantframe-build-cache (Rust target/)"
echo "  - quantframe-cargo-cache (Cargo cache)"
