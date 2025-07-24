#!/usr/bin/env bash

set -e
FORGE_DIR="$HOME/.forge"
REPO_URL="https://github.com/anthonyb8/forge.git"

if [ -d "$FORGE_DIR/.git" ]; then
  echo "Updating forge..."
  git -C "$FORGE_DIR" pull
else
  echo "Cloning forge..."
  rm -rf "$FORGE_DIR" # clean out any old directories
  mkdir -p "$FORGE_DIR"
  git clone "$REPO_URL" "$FORGE_DIR"
fi

if cd "$FORGE_DIR"; then
  echo "Building forge..."
  cargo build --release

  echo "Installing forge..."
  mkdir -p ~/.forge/bin
  cp target/release/forge ~/.forge/bin/

  echo ""
  echo "To use 'forge' from anywhere, add this to your shell config:"
  echo ""
  echo "    export PATH=\"\$HOME/.forge/bin:\$PATH\""
  echo "or run: "
  echo "    echo 'export PATH=\"\$HOME/.forge/bin:\$PATH\"' >> ~/.zshrc "
  echo ""
  echo "Then restart your shell"
else
  echo "Error finding forge directory"
fi
