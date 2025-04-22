#!/bin/bash

API_SERVER_BINARY="api_server"

echo "Building the Go project..."
go build -o $API_SERVER_BINARY

if [ $? -ne 0 ]; then
    echo "Go build failed. Exiting."
    exit 1
fi

CLI_API_DIR="../cli/client"
GUI_API_DIR="../gui/api"

echo "Copying to $CLI_API_DIR..."
mkdir -p CLI_API_DIR
cp $API_SERVER_BINARY $CLI_API_DIR/

echo "Copying to $GUI_API_DIR..."
mkdir -p $GUI_API_DIR
cp $API_SERVER_BINARY $GUI_API_DIR/

echo "Build and copy operations completed successfully."