#!/bin/sh
# Inspired by https://github.com/release-lab/install

echo "Installing Chainbase Manuscript CLI"

set -e

get_arch() {
    a=$(uname -m)
    case ${a} in
        "x86_64" | "amd64" )
            echo "amd64"
        ;;
        "aarch64" | "arm64" | "arm")
            echo "arm64"
        ;;
        *)
            echo "unsupported"
        ;;
    esac
}

get_os(){
    os=$(uname -s | awk '{print tolower($0)}')
    case ${os} in
        "darwin")
            echo "darwin"
        ;;
        "linux")
            echo "linux"
        ;;
        *)
            echo "unsupported"
        ;;
    esac
}

# Improved sudo handling for different platforms
check_sudo() {
    if command -v sudo >/dev/null 2>&1; then
        echo "sudo"
    else
        echo ""
    fi
}

# Function to sign the binary (Mac specific)
sign_binary() {
    os=$1
    executable=$2
    use_sudo=${3:-false}

    if [ "$os" = "darwin" ]; then
        echo "Signing '${executable}'"
        if [ "$use_sudo" = "true" ] && [ -n "$(check_sudo)" ]; then
            sudo codesign -s - "${executable}"
        else
            codesign -s - "${executable}"
        fi
    fi
}

# Parse arguments
for i in "$@"; do
    case $i in
        -v=*|--version=*)
            version="${i#*=}"
            shift
        ;;
        *)
        ;;
    esac
done

owner="chainbase-labs"
repo="manuscript-core"
githubUrl="https://github.com"
exe_name="manuscript-cli"

# Get OS and architecture
os=$(get_os)
arch=$(get_arch)

# Validate OS and architecture
if [ "$arch" = "unsupported" ]; then
    echo "Error: Unsupported architecture: $(uname -m)"
    exit 1
fi

if [ "$os" = "unsupported" ]; then
    echo "Error: Unsupported operating system: $(uname -s)"
    exit 1
fi

# Set up sudo command if available
SUDO=$(check_sudo)

# Determine install location based on OS
if [ "$os" = "darwin" ]; then
    default_install_dir="/usr/local/bin"
else
    # On Linux, prefer /usr/local/bin, fallback to $HOME/.local/bin
    if [ -w "/usr/local/bin" ] || [ -n "$SUDO" ]; then
        default_install_dir="/usr/local/bin"
    else
        default_install_dir="$HOME/.local/bin"
        mkdir -p "$default_install_dir"
    fi
fi

# Set up temporary directory with proper permissions
if [ "$os" = "darwin" ]; then
    downloadFolder="$(mktemp -d)/manuscript/"
else
    downloadFolder="${TMPDIR:-/tmp}/manuscript/"
fi

# Create directories with appropriate permissions
$SUDO mkdir -p "${downloadFolder}"
if [ -n "$SUDO" ]; then
    $SUDO chmod 777 "${downloadFolder}"
fi

file_extension="tar.gz"
file_name="${repo}-${os}-${arch}.${file_extension}"
downloaded_file="${downloadFolder}${file_name}"
executable_folder="$default_install_dir"

$SUDO mkdir -p "${executable_folder}"

# Determine download URL
if [ -z "$version" ]; then
    asset_uri="${githubUrl}/${owner}/${repo}/releases/latest/download/${file_name}"
else
    asset_uri="${githubUrl}/${owner}/${repo}/releases/download/${version}/${file_name}"
fi

echo "[1/6] Detected '${os}-${arch}' architecture"
echo "[2/6] Downloading '${asset_uri}' to '${downloaded_file}'"

# Download with appropriate permissions
if [ -w "${downloadFolder}" ]; then
    curl --fail --location --output "${downloaded_file}" "${asset_uri}"
else
    $SUDO curl --fail --location --output "${downloaded_file}" "${asset_uri}"
fi

echo "[3/6] Extracting files from '${downloaded_file}'"
extract_folder="${downloadFolder}"
$SUDO mkdir -p "${extract_folder}"
if [ -w "${extract_folder}" ]; then
    tar -xzf "${downloaded_file}" -C "${extract_folder}"
else
    $SUDO tar -xzf "${downloaded_file}" -C "${extract_folder}"
fi

echo "[4/6] Installing '${exe_name}' to '${executable_folder}'"
exe_source="${extract_folder}/manuscript-cli"
exe="${executable_folder}/${exe_name}"

# Install binary with appropriate permissions
if [ -w "${executable_folder}" ]; then
    mv "${exe_source}" "${exe}"
    chmod +x "${exe}"
    sign_binary "$os" "$exe" "false"
else
    $SUDO mv "${exe_source}" "${exe}"
    $SUDO chmod +x "${exe}"
    sign_binary "$os" "$exe" "true"
fi

# Remove Temporary Setup directory
echo "[5/6] Cleaning '${downloaded_file}' and extracted files"
if [ -w "${downloadFolder}" ]; then
    rm -rf "${downloadFolder}" || echo "Note: Cleanup of temp files failed - don't worry, your system will automatically clean the temporary directory"
else
    $SUDO rm -rf "${downloadFolder}" || echo "Note: Cleanup of temp files failed - don't worry, your system will automatically clean the temporary directory"
fi || true  # Prevent script termination from set -e

echo "[6/6] Adding '${exe_name}' to the environment variables"
if command -v $exe_name --version >/dev/null; then
    echo "Manuscript CLI was installed successfully"
else
    if [ "$executable_folder" = "$HOME/.local/bin" ]; then
        echo "Installation successful. Please add the following to your shell configuration file:"
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
        
        # Attempt to detect shell and provide specific instructions
        case "$SHELL" in
            */bash)
                echo "For bash, add it to ~/.bashrc"
                ;;
            */zsh)
                echo "For zsh, add it to ~/.zshrc"
                ;;
            */fish)
                echo "For fish, you can run: fish_add_path ~/.local/bin"
                ;;
        esac
    else
        echo "Installation successful. The executable is in ${executable_folder}"
        echo "Please ensure this directory is in your PATH"
    fi
fi

"${exe}" --help
