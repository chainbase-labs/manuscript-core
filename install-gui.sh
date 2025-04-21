#!/bin/bash
set -e

# Fetch the latest version tag from GitHub API
REPO="chainbase-labs/manuscript-core"
LATEST_VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [[ -z "$LATEST_VERSION" ]]; then
    echo "❌ Failed to fetch the latest version. Please check your internet connection or the repository."
    exit 1
fi

echo "📦 Latest version: $LATEST_VERSION"

# Determine OS type and set the download file name
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    TARBALL="manuscript-gui-linux.tar.gz"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    TARBALL="manuscript-gui-mac.tar.gz"
else
    echo "❌ Unsupported OS type: $OSTYPE"
    exit 1
fi

# Download to $HOME
cd "$HOME"

# Construct download URL
BASE_URL="https://github.com/$REPO/releases/download/$LATEST_VERSION"
DOWNLOAD_URL="$BASE_URL/$TARBALL"

echo "⬇️ Downloading $TARBALL to $HOME..."
curl -L -o "$TARBALL" "$DOWNLOAD_URL"

# Create extraction directory
EXTRACT_DIR="$HOME/manuscript-gui"
mkdir -p "$EXTRACT_DIR"

echo "📂 Extracting $TARBALL into $EXTRACT_DIR..."
tar -xzf "$TARBALL" -C "$EXTRACT_DIR"

# Remove the tarball
rm "$TARBALL"

# Make binary executable
chmod +x "$EXTRACT_DIR/manuscript-gui"

# Final success message
echo -e "\n🚀 Success! You can now run the app with:\n\n  $EXTRACT_DIR/manuscript-gui\n"