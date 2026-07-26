#!/usr/bin/env bash
# Build the CLI locally in release mode and install it to ~/.local/bin — for local verification.
# Usage: ./scripts/install-local.sh   (run from anywhere; script cd's to the repo root automatically)
# Override target dir: INSTALL_DIR=/some/dir ./scripts/install-local.sh

set -euo pipefail

cd "$(dirname "$0")/.."

BIN_NAME="jj-llms-txt-wiki"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

echo "==> Building ${BIN_NAME} (release)"
cargo build --release --locked

echo "==> Installing ${BIN_NAME} → ${INSTALL_DIR}"
mkdir -p "$INSTALL_DIR"
cp -f "target/release/${BIN_NAME}" "${INSTALL_DIR}/${BIN_NAME}"
chmod +x "${INSTALL_DIR}/${BIN_NAME}"

echo "==> Installed: ${INSTALL_DIR}/${BIN_NAME}"

case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *) echo "warning: add to PATH → export PATH=\"$INSTALL_DIR:\$PATH\"" ;;
esac

echo "==> Verifying"
"${INSTALL_DIR}/${BIN_NAME}" --version
