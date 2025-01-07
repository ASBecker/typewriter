#!/bin/bash

set -e

echo "Uninstalling typewriter..."

# Define paths
INSTALL_DIR="$HOME/.local/bin"
SOUND_DIR="$HOME/.local/share/typewriter"
BINARY="$INSTALL_DIR/typewriter"

# Remove binary
if [ -f "$BINARY" ]; then
    rm "$BINARY"
    echo "Removed binary: $BINARY"
else
    echo "Binary not found at: $BINARY"
fi

# Remove sound files and directory
if [ -d "$SOUND_DIR" ]; then
    rm -rf "$SOUND_DIR"
    echo "Removed sound files: $SOUND_DIR"
else
    echo "Sound directory not found at: $SOUND_DIR"
fi

# Try to remove parent directory if empty
parent_dir="$(dirname "$SOUND_DIR")"
if [ -d "$parent_dir" ]; then
    rmdir --ignore-fail-on-non-empty "$parent_dir" 2>/dev/null || true
fi

echo "Uninstallation complete!"
echo "Note: You may want to remove the PATH addition from your ~/.bashrc or ~/.zshrc if you added it." 