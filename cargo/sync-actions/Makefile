.PHONY: build install test clean

build:
	@echo "🔨 Building cargo-sync-actions..."
	cargo build --release

install: build
	@echo "📦 Installing cargo-sync-actions..."
	./install.sh

test:
	@echo "🧪 Running tests..."
	cargo test

clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

dev:
	@echo "🔨 Building in dev mode..."
	cargo build

run-example:
	@echo "🚀 Running example sync (dry-run)..."
	cargo run -- sync-actions sync --dry-run

run-example-real:
	@echo "🚀 Running example sync..."
	cargo run -- sync-actions sync

help:
	@echo "Available targets:"
	@echo "  build         - Build the project in release mode"
	@echo "  install       - Build and install to ~/.cargo/bin"
	@echo "  test          - Run tests"
	@echo "  clean         - Clean build artifacts"
	@echo "  dev           - Build in development mode"
	@echo "  run-example   - Run a dry-run sync"
	@echo "  run-example-real - Run an actual sync"
