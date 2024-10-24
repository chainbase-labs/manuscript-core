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
    if [ "$os" = "darwin" ]; then
        echo "darwin"
    else
        echo "$os"
    fi
}

# Function to sign the binary
sign_binary() {
    os=$1
    executable=$2
    use_sudo=${3:-false}

    if [ "$os" = "osx" ]; then
        echo "Signing '${executable}'"
        if [ "$use_sudo" = "true" ]; then
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

downloadFolder="${TMPDIR:-/tmp}"
mkdir -p ${downloadFolder}
os=$(get_os)
arch=$(get_arch)

if [ "$arch" = "unsupported" ]; then
    echo "Unsupported architecture: ${arch}"
    exit 1
fi

file_extension="tar.gz"
file_name="${repo}-${os}-${arch}.${file_extension}" # the file name to download

downloaded_file="${downloadFolder}${file_name}"
executable_folder="/usr/local/bin"

mkdir -p "${executable_folder}"

if [ -z "$version" ]; then
    asset_uri="${githubUrl}/${owner}/${repo}/releases/latest/download/${file_name}"
else
    asset_uri="${githubUrl}/${owner}/${repo}/releases/download/${version}/${file_name}"
fi

echo "[1/6] Detected '${os}-${arch}' architecture"
echo "[2/6] Downloading '${asset_uri}' to '${downloaded_file}'"
curl --fail --location --output "${downloaded_file}" "${asset_uri}"

echo "[3/6] Extracting files from '${downloaded_file}'"
extract_folder="${downloadFolder}"
mkdir -p "${extract_folder}"
tar -xzf "${downloaded_file}" -C "${extract_folder}"

echo "[4/6] Installing '${exe_name}' to '${executable_folder}'"
exe_source="${extract_folder}/manuscript-cli"  # Point to the binary directly
exe="${executable_folder}/${exe_name}"

if [ ! -w "${executable_folder}" ]; then
    echo "Permission denied for ${executable_folder}. Trying with sudo..."
    sudo mv "${exe_source}" "${exe}"
    sudo chmod +x "${exe}"
    sign_binary "$os" "$exe" "true"
else
    mv "${exe_source}" "${exe}"
    chmod +x "${exe}"
    sign_binary "$os" "$exe"
fi

echo "[5/6] Cleaning '${downloaded_file}' and extracted files"
rm -f "${downloaded_file}"

echo "[6/6] Adding '${exe_name}' to the environment variables"
if command -v $exe_name --version >/dev/null; then
    echo "Manuscript CLI was installed successfully"
else
    echo "We couldn't add '${exe_name}' to the environment variables automatically."
    echo "Please add the directory to your \$HOME/.bash_profile (or similar):"
    echo "  export PATH=${executable_folder}:\$PATH"
fi

"${exe}" --help

exit 0