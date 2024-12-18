#!/bin/bash

# Fetch the latest version tag from GitHub API
REPO="chainbase-labs/manuscript-core"
LATEST_VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [[ -z "$LATEST_VERSION" ]]; then
    echo "Failed to fetch the latest version. Please check your internet connection or the repository."
    exit 1
fi

echo "Latest version: $LATEST_VERSION"

# Define the base URL with the latest version
BASE_URL="https://github.com/$REPO/releases/download/$LATEST_VERSION"

# Determine OS type and set the download URL
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    BINARY_URL="$BASE_URL/manuscript-gui-linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    BINARY_URL="$BASE_URL/manuscript-gui-mac"
else
    echo "Unsupported OS type: $OSTYPE"
    exit 1
fi

# Download the binary
echo "Downloading from $BINARY_URL..."
curl -L -o manuscript-gui "$BINARY_URL"

if [[ $? -ne 0 ]]; then
    echo "Failed to download the binary. Please check the URL or your network connection."
    exit 1
fi

# Make the binary executable
chmod +x manuscript-gui

# Run the binary
echo -e "🚀 Success! The binary is locked, loaded, and ready to go. \n🏃 Start it with: ./manuscript-gui"


