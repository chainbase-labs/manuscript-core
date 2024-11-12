#!/bin/bash

# Define the version and base URL
VERSION="v1.0.4"
BASE_URL="https://github.com/chainbase-labs/manuscript-core/releases/download/$VERSION"

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
curl -L -o manuscript "$BINARY_URL"

# Make the binary executable
chmod +x manuscript

# Run the binary
./manuscript
