#!/bin/sh
# Inspired by https://github.com/release-lab/install

echo "Installing Chainbase Manuscript CLI"

set -e

get_arch() {
    # darwin/amd64: Darwin axetroydeMacBook-Air.local 20.5.0 Darwin Kernel Version 20.5.0: Sat May  8 05:10:33 PDT 2021; root:xnu-7195.121.3~9/RELEASE_X86_64 x86_64
    # linux/amd64: Linux test-ubuntu1804 5.4.0-42-generic #46~18.04.1-Ubuntu SMP Fri Jul 10 07:21:24 UTC 2020 x86_64 x86_64 x86_64 GNU/Linux
    a=$(uname -m)
    case ${a} in
        "x86_64" | "amd64" )
            echo "x64"
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
        echo "osx"
    else
        echo "$os"
    fi
}

# Function to sign the binary
sign_binary() {
    os=$1
    executable=$2
    use_sudo=${3:-false}  # Set default value to false if not provided explicitly

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
            shift # past argument=value
        ;;
        *)
            # unknown option
        ;;
    esac
done

owner="chainbase-labs"
repo="manuscript-core"
githubUrl="https://github.com"
exe_name="manuscript-cli"

downloadFolder="${TMPDIR:-/tmp}"
mkdir -p ${downloadFolder} # make sure download folder exists
os=$(get_os)
arch=$(get_arch)

if [ "$arch" = "unsupported" ]; then
    echo "Unsupported architecture: ${arch}"
    exit 1
fi

file_extension="tar.gz"
file_name="${os}-${arch}.${file_extension}" # the file name to download

downloaded_file="${downloadFolder}/${file_name}" # the file path to download
executable_folder="/usr/local/bin" # Eventually, the executable file will be placed here

# Create executable_folder if it does not exist
mkdir -p "${executable_folder}"

# if version is empty
if [ -z "$version" ]; then
    asset_uri="${githubUrl}/${owner}/${repo}/releases/latest/download/${file_name}"
else
    asset_uri="${githubUrl}/${owner}/${repo}/releases/download/${version}/${file_name}"
fi

echo "[1/5] Detected '${os}-${arch}' architecture"
echo "[2/5] Downloading '${asset_uri}' to '${downloaded_file}'"
curl --fail --location --output "${downloaded_file}" "${asset_uri}"

echo "[3/5] Installing '${exe_name}' to '${executable_folder}'"
if [ ! -w "${executable_folder}" ]; then
    echo "Permission denied for ${executable_folder}. Trying with sudo..."
    sudo tar -xzf "${downloaded_file}" -C "${executable_folder}"
    exe="${executable_folder}/${exe_name}"
    sudo chmod +x "${exe}"
    sign_binary "$os" "$exe" "true"
else
    tar -xzf "${downloaded_file}" -C "${executable_folder}"
    exe="${executable_folder}/${exe_name}"
    chmod +x "${exe}"
    sign_binary "$os" "$exe"
fi

echo "[4/5] Cleaning '${downloaded_file}'"
rm -f "${downloaded_file}"

echo "[5/5] Adding '${exe_name}' to the environment variables"
if command -v $exe_name --version >/dev/null; then
    echo "Manuscript CLI was installed successfully"
else
    echo "We couldn't add '${exe_name}' to the environment variables automatically."
    echo "Please add the directory to your \$HOME/.bash_profile (or similar):"
    echo "  export PATH=${executable_folder}:\$PATH"
fi

echo "Run '${exe_name} --help' to get started"
exit 0
