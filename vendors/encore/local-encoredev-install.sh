#!/usr/bin/env bash

set -euo pipefail

# === CONFIGURATION ===
SOURCE_DIR="./target/release"
INSTALL_PREFIX="/opt/homebrew/Cellar/encore/dev"
LIBEXEC_DIR="${INSTALL_PREFIX}/libexec"
BIN_DIR="${INSTALL_PREFIX}/bin"
BASH_COMPLETION_DIR="${INSTALL_PREFIX}/etc/bash_completion.d"
ZSH_COMPLETION_DIR="${INSTALL_PREFIX}/share/zsh/site-functions"
FISH_COMPLETION_DIR="${INSTALL_PREFIX}/share/fish/vendor_completions.d"

# Create necessary directories
echo "Creating directories..."
mkdir -p "${LIBEXEC_DIR}/bin"
mkdir -p "${LIBEXEC_DIR}/runtimes/js"
mkdir -p "${BIN_DIR}"
mkdir -p "${BASH_COMPLETION_DIR}"
mkdir -p "${ZSH_COMPLETION_DIR}"
mkdir -p "${FISH_COMPLETION_DIR}"

# Install binaries
echo "Installing binaries..."
for binary in encore git-remote-encore tsbundler-encore tsparser-encore; do
    if [ -f "${SOURCE_DIR}/${binary}" ]; then
        echo "Installing ${binary}..."
        cp "${SOURCE_DIR}/${binary}" "${LIBEXEC_DIR}/bin/${binary}"
        chmod +x "${LIBEXEC_DIR}/bin/${binary}"
        ln -sf "${LIBEXEC_DIR}/bin/${binary}" "${BIN_DIR}/${binary}"
    else
        echo "Warning: ${binary} not found in ${SOURCE_DIR}"
    fi
done

# Install runtime files
echo "Installing runtime files..."
cp -r ./runtimes/js/encore.dev/* "${LIBEXEC_DIR}/runtimes/js/"
cp -r ./runtimes/core "${LIBEXEC_DIR}/runtimes/"
cp -r ./runtimes/go "${LIBEXEC_DIR}/runtimes/"

# Install shell completions
echo "Installing shell completions..."
"${BIN_DIR}/encore" completion bash > "${BASH_COMPLETION_DIR}/encore"
"${BIN_DIR}/encore" completion zsh > "${ZSH_COMPLETION_DIR}/_encore"
"${BIN_DIR}/encore" completion fish > "${FISH_COMPLETION_DIR}/encore.fish"

# Add to PATH if not already present
if [[ ":$PATH:" != *":${BIN_DIR}:"* ]]; then
    echo "Adding ${BIN_DIR} to PATH..."
    echo "export PATH=\"${BIN_DIR}:\$PATH\"" >> ~/.zshrc
    echo "export PATH=\"${BIN_DIR}:\$PATH\"" >> ~/.bashrc
    echo "Please restart your shell or run 'source ~/.zshrc' (or 'source ~/.bashrc') to update your PATH"
fi
