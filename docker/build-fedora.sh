#!/bin/bash

set -e

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Build the Docker image
echo "Building Docker image (Fedora)..."
docker build -f "$SCRIPT_DIR/Dockerfile.fedora" -t quantframe-react:fedora "$PROJECT_ROOT"

# Run the container with the project mounted and build inside
echo "Building application in container..."
docker run --rm \
  -v "$PROJECT_ROOT:/app" \
  -v quantframe-cargo-cache:/root/.cargo \
  quantframe-react:fedora \
  bash -c "cd /app && pnpm install --frozen-lockfile && pnpm tauri build --bundles rpm"

echo ""
echo "   - RPM packages: src-tauri/target/release/bundle/rpm/"
echo "   - Binary: src-tauri/target/release/"
echo "   - Node modules: ./node_modules"
echo "   - Rust build cache: ./src-tauri/target"

