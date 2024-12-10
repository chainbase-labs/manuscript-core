# Variables
VERSION ?= 1.1.0
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
CLI_DIR=.
GUI_DIR=gui

# Build both binaries with `make all`
.PHONY: all
all: cli gui

# Build only CLI with `make cli`
.PHONY: cli
cli:
	@echo "Building CLI..."
	@echo "Git Commit: $(GIT_COMMIT)"
	@echo "Build Date: $(BUILD_DATE)"
	@echo "Go Version: $(GO_VERSION)"
	@echo "Rust Version: $(RUST_VERSION)"
	cd $(CLI_DIR) && go build $(GO_LDFLAGS) -o $(BINARY_NAME_CLI)

# Build only GUI with `make gui`
.PHONY: gui
gui:
	@echo "Building GUI..."
	cd $(GUI_DIR) && cargo build --release

# Clean build artifacts with `make clean`
.PHONY: clean
clean:
	@echo "Cleaning build artifacts..."
	cd $(CLI_DIR) && go clean
	rm -f $(CLI_DIR)/$(BINARY_NAME_CLI)
	cd $(GUI_DIR) && cargo clean

# Run respective tests with `make test`
.PHONY: test
test:
	@echo "Running tests..."
	cd $(CLI_DIR) && go test ./...
	cd $(GUI_DIR) && cargo test

# Install binaries to /usr/local/bin with `make install`
.PHONY: install
install:
	@echo "Installing binaries..."
	cp $(CLI_DIR)/$(BINARY_NAME_CLI) /usr/local/bin/
	cp $(GUI_DIR)/target/release/$(BINARY_NAME_GUI) /usr/local/bin/

# Help target
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  all           - Build both CLI and GUI"
	@echo "  cli           - Build only the CLI"
	@echo "  gui           - Build only the GUI"
	@echo "  clean         - Clean build artifacts"
	@echo "  test          - Run tests for both projects"
	@echo "  install       - Install binaries to /usr/local/bin"
