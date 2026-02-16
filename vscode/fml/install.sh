#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "==> Installing dependencies..."
npm install

echo "==> Packaging extension..."
npx @vscode/vsce package

VSIX_FILE=$(ls -t harana-fml-*.vsix 2>/dev/null | head -1)

if [ -z "$VSIX_FILE" ]; then
  echo "ERROR: No .vsix file found after packaging."
  exit 1
fi

echo "==> Installing $VSIX_FILE into VS Code..."
code --install-extension "$VSIX_FILE"

echo "==> Done! Restart VS Code or reload the window to activate."
