#!/usr/bin/env bash
# OpenAPI Field Explorer - Installation Script for Linux/macOS
# Usage: curl -fsSL https://raw.githubusercontent.com/antikkorps/openapi_explorer/main/install.sh | bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO="antikkorps/openapi_explorer"
BINARY_NAME="openapi-explorer"
INSTALL_DIR="${HOME}/.local/bin"

echo -e "${BLUE}╔════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   OpenAPI Field Explorer - Installation       ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════╝${NC}"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}⚠  Rust is not installed.${NC}"
    echo -e "   OpenAPI Explorer requires Rust to build from source."
    echo ""
    echo -e "   Would you like to install Rust now? (y/N) "
    read -r response
    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        echo -e "${BLUE}→ Installing Rust...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        echo -e "${GREEN}✓ Rust installed successfully${NC}"
    else
        echo -e "${RED}✗ Installation cancelled. Please install Rust first:${NC}"
        echo -e "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
fi

# Check Rust version
RUST_VERSION=$(rustc --version | cut -d' ' -f2)
echo -e "${GREEN}✓ Rust $RUST_VERSION detected${NC}"
echo ""

# Determine installation method
echo -e "${BLUE}Choose installation method:${NC}"
echo "  1) Build from source (latest, requires ~2min)"
echo "  2) Download pre-built binary (faster, if available)"
echo ""
read -p "Enter choice [1-2]: " choice

case $choice in
    2)
        # Try to download pre-built binary
        echo -e "${BLUE}→ Detecting system...${NC}"
        OS=$(uname -s)
        ARCH=$(uname -m)

        case "$OS" in
            Linux)
                PLATFORM="unknown-linux-gnu"
                ;;
            Darwin)
                PLATFORM="apple-darwin"
                ;;
            *)
                echo -e "${YELLOW}⚠  Unsupported OS: $OS${NC}"
                echo -e "   Falling back to building from source..."
                choice=1
                ;;
        esac

        if [ "$choice" = "2" ]; then
            TARGET="${ARCH}-${PLATFORM}"
            echo -e "${BLUE}→ Downloading binary for $TARGET...${NC}"

            # Try to get latest release
            LATEST_URL="https://github.com/$REPO/releases/latest/download/${BINARY_NAME}-${TARGET}"

            if curl -fsSL "$LATEST_URL" -o "/tmp/${BINARY_NAME}" 2>/dev/null; then
                chmod +x "/tmp/${BINARY_NAME}"
                echo -e "${GREEN}✓ Binary downloaded${NC}"
            else
                echo -e "${YELLOW}⚠  Pre-built binary not available for $TARGET${NC}"
                echo -e "   Falling back to building from source..."
                choice=1
            fi
        fi
        ;;
esac

# Build from source if needed
if [ "$choice" = "1" ]; then
    echo -e "${BLUE}→ Cloning repository...${NC}"
    TMP_DIR=$(mktemp -d)
    git clone "https://github.com/$REPO.git" "$TMP_DIR" 2>/dev/null || {
        echo -e "${RED}✗ Failed to clone repository${NC}"
        exit 1
    }

    cd "$TMP_DIR"
    echo -e "${GREEN}✓ Repository cloned${NC}"
    echo ""

    echo -e "${BLUE}→ Building release binary...${NC}"
    echo -e "${YELLOW}   This may take a few minutes...${NC}"
    cargo build --release --quiet || {
        echo -e "${RED}✗ Build failed${NC}"
        exit 1
    }

    BINARY_PATH="$TMP_DIR/target/release/$BINARY_NAME"
    echo -e "${GREEN}✓ Build successful${NC}"
else
    BINARY_PATH="/tmp/${BINARY_NAME}"
fi

# Create installation directory
mkdir -p "$INSTALL_DIR"

# Install binary
echo -e "${BLUE}→ Installing to $INSTALL_DIR...${NC}"
cp "$BINARY_PATH" "$INSTALL_DIR/$BINARY_NAME"
chmod +x "$INSTALL_DIR/$BINARY_NAME"
echo -e "${GREEN}✓ Binary installed${NC}"

# Check if install directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo -e "${YELLOW}⚠  $INSTALL_DIR is not in your PATH${NC}"
    echo ""
    echo -e "   Add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
    echo -e "   ${BLUE}export PATH=\"\$PATH:$INSTALL_DIR\"${NC}"
    echo ""
fi

# Cleanup
if [ "$choice" = "1" ]; then
    rm -rf "$TMP_DIR"
fi

# Verify installation
echo ""
if command -v "$BINARY_NAME" &> /dev/null || [ -f "$INSTALL_DIR/$BINARY_NAME" ]; then
    VERSION=$("$INSTALL_DIR/$BINARY_NAME" --version 2>/dev/null || echo "unknown")
    echo -e "${GREEN}╔════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║   ✓ Installation Complete!                    ║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "  Run: ${BLUE}$BINARY_NAME examples/petstore.json${NC}"
    echo -e "  Help: ${BLUE}$BINARY_NAME --help${NC}"
    echo ""
else
    echo -e "${RED}✗ Installation verification failed${NC}"
    exit 1
fi
