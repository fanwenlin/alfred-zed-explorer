# OpenInZed Makefile
TARGET_DIR=target/release
OUTPUT_DIR=output
WORKFLOW_NAME=OpenInZed.alfredworkflow

.PHONY: all build release release-lint test install-local clean install-deps

# Default target
all: build

# Install dependencies
install-deps:
	@echo "📦 Installing Rust dependencies..."
	cargo check

# Build debug version
build:
	@echo "🔨 Building debug version..."
	cargo build

# Release build
release:
	@echo "🏗️  Building release version..."
	cargo build --release
	@echo "✅ Release build complete"

# Release build with linting
release-lint: lint
	@echo "🏗️  Building release version with linting..."
	cargo build --release
	@echo "✅ Release build complete"

# Run tests
test:
	@echo "🧪 Running tests..."
	cargo test

# Lint code
lint:
	@echo "🔍 Linting code..."
	cargo clippy -- -D warnings
	cargo fmt --all -- --check

# Format code
fmt:
	@echo "🎨 Formatting code..."
	cargo fmt --all

# Build and package release for Alfred
package: release
	@echo "📦 Packaging Alfred workflow..."

	# Clean and create output directory
	@rm -rf "$(OUTPUT_DIR)"
	@mkdir -p "$(OUTPUT_DIR)/$(WORKFLOW_NAME)"

	# Copy built binaries
	@cp "$(TARGET_DIR)/zed-search" "$(OUTPUT_DIR)/$(WORKFLOW_NAME)/"
	@cp "$(TARGET_DIR)/zed-recent" "$(OUTPUT_DIR)/$(WORKFLOW_NAME)/"
	@cp "$(TARGET_DIR)/zed" "$(OUTPUT_DIR)/$(WORKFLOW_NAME)/"
	@cp "$(TARGET_DIR)/open" "$(OUTPUT_DIR)/$(WORKFLOW_NAME)/"

	# Copy info.plist and icon.png
	@cp info.plist "$(OUTPUT_DIR)/$(WORKFLOW_NAME)/"
	@if [ -f "icon.png" ]; then cp "icon.png" "$(OUTPUT_DIR)/$(WORKFLOW_NAME)/"; fi

	# Copy to Desktop
	@rm -rf ~/Desktop/"$(WORKFLOW_NAME)"
	@cp -r "$(OUTPUT_DIR)/$(WORKFLOW_NAME)" ~/Desktop/

	@echo ""
	@echo "✅ Workflow packaged successfully!"
	@echo "📍 Location: ~/Desktop/$(WORKFLOW_NAME)"
	@echo ""
	@echo "To install:"
	@echo "  1. Double-click: ~/Desktop/$(WORKFLOW_NAME)"
	@echo "  2. Or drag into Alfred Preferences > Workflows"

# Build and install locally for testing
install-local: release
	@echo "🔧 Installing locally for testing..."

	@WORKFLOWS_DIR="$$HOME/Library/Application Support/Alfred/Alfred.alfredpreferences/workflows"; \
	UUID=$$(uuidgen | tr '[:upper:]' '[:lower:]'); \
	TARGET_WORKFLOW_DIR="$$WORKFLOWS_DIR/com.fwl.openinzed.$$UUID"; \
	echo "🧹 Cleaning up existing OpenInZed workflows..."; \
	if [ -d "$$WORKFLOWS_DIR" ]; then \
		find "$$WORKFLOWS_DIR" -maxdepth 1 -type d -name 'com.fwl.openinzed.*' 2>/dev/null | while IFS= read -r dir; do \
			if [ -n "$$dir" ]; then \
				echo "   Removing: $$dir"; \
				rm -rf "$$dir"; \
			fi; \
		done; \
		echo "   ✅ Cleanup complete"; \
	else \
		echo "   No existing workflow found"; \
	fi; \
	echo ""; \
	echo "📁 Installing to: $$TARGET_WORKFLOW_DIR"; \
	mkdir -p "$$TARGET_WORKFLOW_DIR"; \
	cp $(TARGET_DIR)/zed-search "$$TARGET_WORKFLOW_DIR/"; \
	cp $(TARGET_DIR)/zed-recent "$$TARGET_WORKFLOW_DIR/"; \
	cp $(TARGET_DIR)/zed "$$TARGET_WORKFLOW_DIR/"; \
	cp $(TARGET_DIR)/open "$$TARGET_WORKFLOW_DIR/"; \
	cp info.plist "$$TARGET_WORKFLOW_DIR/"; \
	if [ -f "icon.png" ]; then cp "icon.png" "$$TARGET_WORKFLOW_DIR/"; fi; \
	echo ""; \
	echo "✅ Local installation complete!"; \
	echo "🔑 Workflow installed with UUID: $$UUID"; \
	echo ""; \
	echo "To use:"; \
	echo "  • Type 'zopen <query>' to search projects"; \
	echo "  • Type 'zrecent <query>' to search recent projects"; \
	echo "  • Type 'zed <query>' to search both recent and directory projects"; \
	echo "  • Configure PROJECT_DIRS in Alfred workflow settings"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	rm -rf $(OUTPUT_DIR)
	@echo "✅ Clean complete"

# Full clean (including desktop workflow)
clean-all: clean
	@echo "🧹 Removing workflow from Desktop..."
	rm -rf ~/Desktop/$(WORKFLOW_NAME)
	@echo "✅ Clean complete"

# Debug: list all recent projects from Zed DB
debug-zed-db: release
	@echo "🔍 Running Zed DB debug tool..."
	@echo ""
	@"$(TARGET_DIR)/debug-zed-db"

# Pre-commit checks
pre-commit: fmt lint test
	@echo "✅ Pre-commit checks passed"

# Help
help:
	@echo "OpenInZed Makefile"
	@echo ""
	@echo "Usage:"
	@echo "  make build              Build debug version"
	@echo "  make release           Build optimized release version"
	@echo "  make release-lint      Build with linting"
	@echo "  make debug-zed-db      Debug: list all projects from Zed DB"
	@echo "  make test              Run tests"
	@echo "  make lint              Lint code"
	@echo "  make fmt               Format code"
	@echo "  make package           Build and package workflow for release"
	@echo "  make install-local     Install locally to Alfred for testing"
	@echo "  make clean             Clean build artifacts"
	@echo "  make clean-all         Clean everything including Desktop workflow"
	@echo "  make pre-commit        Run pre-commit checks (fmt, lint, test)"
	@echo "  make help              Show this help"
	@echo ""
	@echo "Typical workflow:"
	@echo "  1. make install-deps   # First time setup"
	@echo "  2. make test           # Run tests"
	@echo "  2. make debug-zed-db   # Test Zed DB reading (optional)"
	@echo "  3. make release-lint   # Build release with checks"
	@echo "  4. make package        # Package for distribution"
	@echo "  Or for local testing:"
	@echo "  4. make install-local  # Install to Alfred"
	@echo ""
	@echo "For debugging Zed DB:"
	@echo "  make debug-zed-db      # List all recent projects from Zed"
