#!/bin/bash

# Script to install rtop binary from GitHub latest release based on platform and add to PATH

# Determine the platform
OS=$(uname -s)
case "$OS" in
    Linux*)     PLATFORM=linux; BINARY_NAME="rtop-linux";;
    Darwin*)    PLATFORM=mac; BINARY_NAME="rtop-darwin";;
    CYGWIN*|MINGW*|MSYS*) PLATFORM=windows; BINARY_NAME="rtop-windows.exe";;
    *)          echo "Unsupported platform: $OS"; exit 1;;
esac

# Define installation directory
INSTALL_DIR="/usr/local/bin"
if [ "$PLATFORM" = "windows" ]; then
    INSTALL_DIR="$HOME/bin"
    mkdir -p "$INSTALL_DIR"
fi
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
if [ "$PLATFORM" != "windows" ]; then
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        echo "Adding $INSTALL_DIR to PATH..."
        echo "export PATH=\$PATH:$INSTALL_DIR" >> ~/.bashrc
        echo "export PATH=\$PATH:$INSTALL_DIR" >> ~/.zshrc
        export PATH=$PATH:$INSTALL_DIR
    fi
else
    echo "Please manually add $INSTALL_DIR to your system PATH in Windows settings."
fi

# Verify installation
if command -v rtop >/dev/null 2>&1; then
    echo "rtop installed successfully! You can now run it by typing 'rtop'."
else
    echo "Installation completed, but rtop is not in PATH. Please restart your terminal or source your shell configuration."
fi