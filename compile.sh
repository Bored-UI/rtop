#!/bin/bash

# Exit on any error
set -e

# Set the project name (must match [package.name] in Cargo.toml)
PROJECT_NAME="rtop"

# --- Pre-flight Checks ---

# Ensure Docker is running (required for cross)
if ! docker info >/dev/null 2>&1; then
  echo "Error: Docker is not running. Please start Docker Desktop or your Docker daemon."
  exit 1
fi

# Ensure cross is installed
if ! command -v cross >/dev/null 2>&1; then
  echo "Installing cross..."
  # Use --locked to ensure reproducible installation based on Cargo.lock if present
  # Use --force if you want to reinstall even if it's already there (e.g., for updates)
  cargo install cross --locked || cargo install cross # Try locked first, then fallback
fi

# Ensure required targets are installed
echo "Checking Rust targets..."
rustup target add aarch64-unknown-linux-musl x86_64-unknown-linux-musl x86_64-pc-windows-gnu

# clean those build
cargo clean
# --- Platform Detection for Cross-compilation (THE FIX) ---
# If running on an ARM64/AARCH64 host, explicitly tell Docker to use amd64 images
# as the cross-rs images are typically amd64 containers containing the toolchains.
if [[ "$(uname -m)" == "arm64" || "$(uname -m)" == "aarch64" ]]; then
  export DOCKER_DEFAULT_PLATFORM="linux/amd64"
  echo "Detected ARM64/AARCH64 host. Setting DOCKER_DEFAULT_PLATFORM to linux/amd64 for cross-compilation via emulation."
else
  echo "Detected x86_64 host. No DOCKER_DEFAULT_PLATFORM override needed."
fi

# --- Native Build ---

# Build for macOS (native)
# This uses cargo directly, as it's not cross-compiling on a macOS host.
echo "Building for macOS (native)..."
cargo build --release
mv target/release/$PROJECT_NAME target/release/rtop-darwin

# --- Cross-Compilation Builds ---

# Build for Windows x86_64
echo "Building for Windows x86_64..."
cross build --target x86_64-pc-windows-gnu --release 
mv target/x86_64-pc-windows-gnu/release/$PROJECT_NAME.exe target/x86_64-pc-windows-gnu/release/rtop-windows-x86_64.exe

# Build for Linux aarch64
echo "Building for Linux aarch64..."
# The DOCKER_DEFAULT_PLATFORM variable will ensure the correct image is pulled/used.
cross build --target aarch64-unknown-linux-musl --release 
mv target/aarch64-unknown-linux-musl/release/$PROJECT_NAME target/aarch64-unknown-linux-musl/release/rtop-linux

# Build for Linux x86_64
echo "Building for Linux x86_64..."
cross build --target x86_64-unknown-linux-musl --release 
mv target/x86_64-unknown-linux-musl/release/$PROJECT_NAME target/x86_64-unknown-linux-musl/release/rtop-linux-x86_64

echo "Builds completed successfully!"
echo "Binaries:"
echo "- target/release/rtop-darwin"
echo "- target/x86_64-pc-windows-gnu/release/rtop-windows-x86_64.exe"
echo "- target/aarch64-unknown-linux-musl/release/rtop-linux"
echo "- target/x86_64-unknown-linux-musl/release/rtop-linux-x86_64"
