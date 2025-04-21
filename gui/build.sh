#!/bin/bash
set -e

# 清理旧构建
echo "🧹 Cleaning old build..."
cargo clean

# 编译 Rust
echo "⚙️ Building Rust project..."
cargo build --release

# 获取目标路径
BIN_PATH=target/release/manuscript-gui
GO_BIN=target/release/api_server

# 判断系统类型
OS_TYPE=$(uname)
if [[ "$OS_TYPE" == "Darwin" ]]; then
  OUTPUT_TAR=target/manuscript-gui-mac.tar.gz
elif [[ "$OS_TYPE" == "Linux" ]]; then
  OUTPUT_TAR=target/manuscript-gui-linux.tar.gz
else
  echo "❌ Unsupported OS: $OS_TYPE"
  exit 1
fi

# 检查文件是否存在
if [ ! -f "$BIN_PATH" ]; then
  echo "❌ Rust binary not found: $BIN_PATH"
  exit 1
fi

if [ ! -f "$GO_BIN" ]; then
  echo "❌ Go binary not found: $GO_BIN"
  exit 1
fi

# 打包
echo "📦 Packaging binaries..."
tar -czf "$OUTPUT_TAR" -C target/release manuscript-gui api_server
echo "✅ Bundle created at $OUTPUT_TAR"