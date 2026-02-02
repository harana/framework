#!/bin/bash

# Install script for cargo-generate-models

echo "📦 Installing cargo-generate-models..."

cd "$(dirname "$0")"

# Build in release mode
cargo build --release

# Install to cargo bin directory
cargo install --path .

echo "✅ Installation complete!"
echo ""
echo "Usage:"
echo "  cargo generate-models generate --schema-dir <path> --rust-output <path> --python-output <path>"
echo ""
echo "Example:"
echo "  cargo generate-models generate \\"
echo "    --schema-dir core/schema/data \\"
echo "    --rust-output generated/rust \\"
echo "    --python-output generated/python"
