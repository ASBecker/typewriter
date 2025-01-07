#!/bin/bash

set -e

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Convert architecture names
case $ARCH in
    x86_64)
        ARCH="x86_64"
        ;;
    arm64|aarch64)
        ARCH="aarch64"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Set the appropriate asset name based on OS and architecture
case $OS in
    linux)
        ASSET="typewriter-linux-x86_64.tar.gz"
        ;;
    darwin)
        ASSET="typewriter-macos-$ARCH.tar.gz"
        ;;
    *)
        echo "Unsupported operating system: $OS"
        exit 1
        ;;
esac

# Get the latest release URL
LATEST_RELEASE_URL=$(curl -s https://api.github.com/repos/ASBecker/typewriter/releases/latest | grep "browser_download_url.*$ASSET" | cut -d '"' -f 4)

if [ -z "$LATEST_RELEASE_URL" ]; then
    echo "Failed to find the latest release for your system"
    exit 1
fi

echo "Installing typewriter..."

# Create installation directory
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"
TEMP_DIR=$(mktemp -d)

# Download and extract
curl -L "$LATEST_RELEASE_URL" -o "$TEMP_DIR/$ASSET"
tar xzf "$TEMP_DIR/$ASSET" -C "$TEMP_DIR"

# Install binary and sounds
mkdir -p "$HOME/.local/share/typewriter"
cp -r "$TEMP_DIR/sounds" "$HOME/.local/share/typewriter/"
cp "$TEMP_DIR/typewriter" "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/typewriter"

# Clean up
rm -rf "$TEMP_DIR"

echo "Installation complete!"
echo "The typewriter binary has been installed to $INSTALL_DIR/typewriter"

# Check if the directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "NOTE: Add $INSTALL_DIR to your PATH by adding this line to your ~/.bashrc or ~/.zshrc:"
    echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
fi 