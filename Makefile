# Variables
VERSION ?= 1.1.1
GIT_COMMIT=$(shell git rev-parse --short HEAD)
BUILD_DATE=$(shell date -u '+%Y-%m-%d:%H:%M:%S')
GO_VERSION=$(shell go version | cut -d' ' -f3)
RUST_VERSION=$(shell rustc --version | cut -d' ' -f2)
BINARY_NAME_CLI=manuscript-cli
BINARY_NAME_GUI=manuscript-gui

# Go build flags matching version_manuscript.go
GO_LDFLAGS=-ldflags "\
-X manuscript-core/commands.gitCommit=$(GIT_COMMIT) \
-X manuscript-core/commands.buildDate=$(BUILD_DATE) \
-X manuscript-core/commands.goVersion=$(GO_VERSION) \
-X manuscript-core/commands.rustVersion=$(RUST_VERSION)"

# Directory Locations
CLI_DIR=cli
GUI_DIR=gui

# Default to help when nothing is specified by user
.DEFAULT_GOAL := help

# Validate version format
VERSION_PATTERN := ^[0-9]+\.[0-9]+\.[0-9]+$$
$(if $(shell echo "$(VERSION)" | grep -E "$(VERSION_PATTERN)"),,\
    $(error "💥 Error: Invalid version format. Format should conform to X.Y.Z"))

# Build both binaries with `make all`
.PHONY: all
all: cli gui
	@if [ ! -f "$(CLI_DIR)/$(BINARY_NAME_CLI)" ] || [ ! -f "$(GUI_DIR)/target/release/$(BINARY_NAME_GUI)" ]; then \
		echo "💥 Error: One or more binaries failed to build"; \
		exit 1; \
	fi
	@echo "🎉 All builds succeeded"
	@echo "📦 CLI binary: $(CLI_DIR)/$(BINARY_NAME_CLI)"
	@echo "📦 GUI binary: $(GUI_DIR)/target/release/$(BINARY_NAME_GUI)"

# Build only CLI with `make cli`
.PHONY: cli
cli:
	@echo "🚀 Building CLI..."
	@echo "🔖 Git Commit: $(GIT_COMMIT)"
	@echo "📅 Build Date: $(BUILD_DATE)"
	@echo "🐹 Go Version: $(GO_VERSION)"
	@echo "🦀 Rust Version: $(RUST_VERSION)"
	@cd $(CLI_DIR) && go build $(GO_LDFLAGS) -o $(BINARY_NAME_CLI)
	@if [ $$? -ne 0 ]; then \
		echo "💥 Error: CLI build failed"; \
		exit 1; \
	else \
		echo "✅ CLI build succeeded"; \
	fi

# Build only GUI with `make gui`
.PHONY: gui
gui:
	@echo "🚀 Building GUI..."
	@cd $(GUI_DIR) && cargo build --release
	@if [ $$? -ne 0 ]; then \
		echo "💥 Error: GUI build failed"; \
		exit 1; \
	else \
		echo "✅ GUI build succeeded"; \
	fi

# Clean build artifacts with `make clean`
.PHONY: clean
clean:
	@echo "🧹 Cleaning build artifacts..."
	@cd $(CLI_DIR) && go clean || { echo "💥 Error: go clean failed"; exit 1; }
	@rm -f $(CLI_DIR)/$(BINARY_NAME_CLI) || { echo "💥 Error: Failed to remove CLI binary"; exit 1; }
	@cd $(GUI_DIR) && cargo clean || { echo "💥 Error: cargo clean failed"; exit 1; }
	@echo "✅ Clean succeeded"

# Run respective tests with `make test`
.PHONY: test
test:
	@echo "🧪 Running tests..."
	@cd $(CLI_DIR) && go test ./...
	@if [ $$? -ne 0 ]; then \
		echo "💥 Error: CLI tests failed"; \
		exit 1; \
	fi
	@cd $(GUI_DIR) && cargo test
	@if [ $$? -ne 0 ]; then \
		echo "💥 Error: GUI tests failed"; \
		exit 1; \
	else \
		echo "✅ All tests passed"; \
	fi

# Install binaries to /usr/local/bin with `make install`
.PHONY: install
install: install-cli install-gui
	@if [ ! -f "/usr/local/bin/$(BINARY_NAME_CLI)" ] || [ ! -f "/usr/local/bin/$(BINARY_NAME_GUI)" ]; then \
		echo "💥 Error: Installation failed - one or more binaries not installed"; \
		exit 1; \
	fi
	@echo "🎉 All installs succeeded"
	@echo "📦 CLI binary: /usr/local/bin/$(BINARY_NAME_CLI)"
	@echo "📦 GUI binary: /usr/local/bin/$(BINARY_NAME_GUI)"

# Install only CLI binary to /usr/local/bin with `make install-cli`
.PHONY: install-cli
install-cli:
	@echo "📦 Installing CLI binary..."
	@if [ ! -w "/usr/local/bin" ]; then \
		echo "💥 Error: Insufficient permissions. Try using sudo."; \
		exit 1; \
	fi
	@if [ ! -f $(CLI_DIR)/$(BINARY_NAME_CLI) ]; then \
		echo "💥 Error: CLI binary not found. Please build first."; \
		exit 1; \
	fi
	@cp $(CLI_DIR)/$(BINARY_NAME_CLI) /usr/local/bin/
	@if [ $$? -ne 0 ]; then \
		echo "💥 Error: CLI install failed"; \
		exit 1; \
	else \
		echo "✅ CLI install succeeded"; \
	fi

# Install only GUI binary to /usr/local/bin with `make install-gui`
.PHONY: install-gui
install-gui:
	@echo "📦 Installing GUI binary..."
	@if [ ! -w "/usr/local/bin" ]; then \
		echo "💥 Error: Insufficient permissions. Try using sudo."; \
		exit 1; \
	fi
	@if [ ! -f $(GUI_DIR)/target/release/$(BINARY_NAME_GUI) ]; then \
		echo "💥 Error: GUI binary not found. Please build first."; \
		exit 1; \
	fi
	@cp $(GUI_DIR)/target/release/$(BINARY_NAME_GUI) /usr/local/bin/
	@if [ $$? -ne 0 ]; then \
		echo "💥 Error: GUI install failed"; \
		exit 1; \
	else \
		echo "✅ GUI install succeeded"; \
	fi

# Help target
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  🔧 all           - Build both CLI and GUI"
	@echo "  🐹 cli           - Build only the CLI"
	@echo "  🦀 gui           - Build only the GUI"
	@echo "  🧹 clean         - Clean build artifacts"
	@echo "  🧪 test          - Run tests for both projects"
	@echo "  📦 install       - Install both binaries to /usr/local/bin"
	@echo "  📦 install-cli   - Install only the CLI binary to /usr/local/bin"
	@echo "  📦 install-gui   - Install only the GUI binary to /usr/local/bin"
