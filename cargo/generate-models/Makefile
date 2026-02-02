.PHONY: build install test clean generate example help

# Default target
help:
	@echo "cargo-generate-models - Makefile targets:"
	@echo ""
	@echo "  build     - Build the project in debug mode"
	@echo "  release   - Build the project in release mode"
	@echo "  install   - Install the plugin to cargo bin"
	@echo "  test      - Run tests"
	@echo "  example   - Generate code from example schema"
	@echo "  generate  - Generate from core schemas (requires SCHEMA_DIR)"
	@echo "  clean     - Clean build artifacts"
	@echo ""
	@echo "Examples:"
	@echo "  make release"
	@echo "  make install"
	@echo "  make example"
	@echo "  make generate SCHEMA_DIR=../core/schema/data"

# Build in debug mode
build:
	cargo build

# Build in release mode
release:
	cargo build --release

# Install to cargo bin
install: release
	cargo install --path .

# Run tests
test:
	cargo test

# Generate from example schema
example: release
	@echo "Generating from example schema..."
	./target/release/cargo-generate-models generate-models generate \
		--schema-dir examples \
		--rust-output /tmp/example-output/rust \
		--python-output /tmp/example-output/python
	@echo ""
	@echo "Generated files:"
	@ls -lh /tmp/example-output/rust/
	@ls -lh /tmp/example-output/python/

# Generate from schema directory (use: make generate SCHEMA_DIR=path/to/schemas)
generate: release
ifndef SCHEMA_DIR
	@echo "Error: SCHEMA_DIR not set"
	@echo "Usage: make generate SCHEMA_DIR=path/to/schemas"
	@exit 1
endif
	./target/release/cargo-generate-models generate-models generate \
		--schema-dir $(SCHEMA_DIR) \
		--rust-output generated/rust \
		--python-output generated/python

# Clean build artifacts
clean:
	cargo clean
	rm -rf generated/
	rm -rf /tmp/example-output/

# Watch for changes and rebuild
watch:
	cargo watch -x build

# Format code
fmt:
	cargo fmt

# Run clippy
lint:
	cargo clippy -- -D warnings

# Full check before commit
check: fmt lint test
	@echo "All checks passed!"
