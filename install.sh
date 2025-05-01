#!/bin/bash

# Script to install rtop binary from GitHub latest release based on platform and add to PATH

# Determine the platform
OS=$(uname -s)
ARCH=$(uname -m) # e.g., x86_64, aarch64, arm64
# Determine the correct binary name based on OS and Arch
case "$OS" in
    Linux)
        PLATFORM="linux"
        if [ "$ARCH" = "x86_64" ]; then
            # Specific name for Linux x86_64
            BINARY_NAME="rtop-linux_x86_64"
        elif [[ "$ARCH" == "aarch64" || "$ARCH" == "arm64" ]]; then
             # Name for Linux ARM64 (adjust if repo uses a different name like 'rtop-linux_arm64')
            BINARY_NAME="rtop-linux" # Assuming 'rtop-linux' is the ARM binary based on your request
        else
            # Fallback or other architectures - assumes 'rtop-linux' exists or fails later
            echo "Warning: Detected Linux architecture '$ARCH'. Attempting to use 'rtop-linux'."
            echo "Please verify this asset exists in the release if you encounter issues."
            BINARY_NAME="rtop-linux"
        fi
        ;;
    Darwin)
        PLATFORM="mac"
        # mac only support for mac that are on apple silicon
        BINARY_NAME="rtop-darwin"
        ;;
    *)
        echo "Error: Unsupported operating system: $OS"
        exit 1
        ;;
esac

# Define installation directory
INSTALL_DIR="/usr/local/bin"
TEMP_DIR=$(mktemp -d)

# Fetch the latest release download URL using GitHub API
REPO="gohyuhan/rtop" # Replace with your GitHub username and repo name
API_URL="https://api.github.com/repos/$REPO/releases/latest"
DOWNLOAD_URL=$(curl -s "$API_URL" | grep "browser_download_url.*$BINARY_NAME" | cut -d '"' -f 4)

if [ -z "$DOWNLOAD_URL" ]; then
    echo "Failed to find $BINARY_NAME in the latest release of $REPO"
    rm -rf "$TEMP_DIR"
    exit 1
fi

# Download the binary
echo "Downloading $BINARY_NAME from $REPO latest release..."
curl -L -o "$TEMP_DIR/$BINARY_NAME" "$DOWNLOAD_URL"
if [ $? -ne 0 ]; then
    echo "Failed to download rtop binary"
    rm -rf "$TEMP_DIR"
    exit 1
fi

# Make the binary executable
chmod +x "$TEMP_DIR/$BINARY_NAME"

# Move the binary to the installation directory
echo "Installing rtop to $INSTALL_DIR..."
if [ "$PLATFORM" = "windows" ]; then
    mv "$TEMP_DIR/$BINARY_NAME" "$INSTALL_DIR/rtop.exe"
else
    sudo mv "$TEMP_DIR/$BINARY_NAME" "$INSTALL_DIR/rtop"
fi
if [ $? -ne 0 ]; then
    echo "Failed to install rtop binary"
    rm -rf "$TEMP_DIR"
    exit 1
fi

# Clean up
rm -rf "$TEMP_DIR"

# Update PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "Adding $INSTALL_DIR to PATH..."
    echo "export PATH=\$PATH:$INSTALL_DIR" >> ~/.bashrc
    echo "export PATH=\$PATH:$INSTALL_DIR" >> ~/.zshrc
    export PATH=$PATH:$INSTALL_DIR
fi

# Verify installation
if command -v rtop >/dev/null 2>&1; then
    echo "rtop installed successfully! You can now run it by typing 'rtop'."
else
    echo "Installation completed, but rtop is not in PATH. Please restart your terminal or source your shell configuration."
fi