#!/bin/bash
set -e

echo "🔧 Installing cargo-sync-actions plugin..."

# Build the project
cargo build --release

# Get the target directory
TARGET_DIR="target/release"

# Get cargo's bin directory
CARGO_BIN="$HOME/.cargo/bin"

# Copy the binary
echo "📦 Installing to $CARGO_BIN"
cp "$TARGET_DIR/cargo-sync-actions" "$CARGO_BIN/"

echo "✅ Installation complete!"
echo ""
echo "You can now run:"
echo "  cargo sync-actions sync --help"
