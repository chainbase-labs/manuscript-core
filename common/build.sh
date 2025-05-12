#!/bin/bash

set -e

APP_NAME="api_server"
CLI_API_DIR="../cli/client"
GUI_API_DIR="../gui/api"

# 支持的目标平台
PLATFORMS=("darwin/amd64" "darwin/arm64" "linux/amd64")

echo "Building $APP_NAME for multiple platforms..."

for PLATFORM in "${PLATFORMS[@]}"
do
    GOOS="${PLATFORM%%/*}"
    GOARCH="${PLATFORM##*/}"
    OUTPUT_NAME="${APP_NAME}_${GOOS}_${GOARCH}"

    echo "Building for $GOOS/$GOARCH -> $OUTPUT_NAME"
    env GOOS=$GOOS GOARCH=$GOARCH go build -o "$OUTPUT_NAME"

    if [ $? -ne 0 ]; then
        echo "Failed to build for $GOOS/$GOARCH"
        exit 1
    fi

    chmod +x "$OUTPUT_NAME"

    # 拷贝到目标路径
    cp "$OUTPUT_NAME" "$CLI_API_DIR/"
    cp "$OUTPUT_NAME" "$GUI_API_DIR/"
done

echo "Build completed successfully."