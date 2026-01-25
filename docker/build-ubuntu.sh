#!/bin/bash

set -e

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Build the Docker image
echo "Building Docker image (Ubuntu)..."
docker build -f "$SCRIPT_DIR/Dockerfile.ubuntu" -t quantframe-react:ubuntu "$PROJECT_ROOT"

# Run the container with the project mounted and build inside
echo "Building application in container..."
docker run --rm \
  -v "$PROJECT_ROOT:/app" \
  -v quantframe-cargo-cache:/root/.cargo \
  quantframe-react:ubuntu \
  bash -c "cd /app && pnpm install --frozen-lockfile && pnpm tauri build --bundles deb"

echo ""
echo "   - DEB packages: src-tauri/target/release/bundle/deb/"
echo "   - Binary: src-tauri/target/release/"
echo "   - Node modules: ./node_modules"
echo "   - Rust build cache: ./src-tauri/target"

