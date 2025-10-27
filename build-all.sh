#!/bin/bash
# Build script for all platforms

set -e

echo "🚀 Building PlankTransformer for all platforms..."
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

build_component() {
    local name=$1
    local path=$2
    echo -e "${BLUE}Building $name...${NC}"
    (cd "$path" && cargo build --release)
    echo -e "${GREEN}✓ $name built successfully${NC}"
    echo ""
}

# Build core libraries
echo "📦 Building Core Libraries"
echo "=========================="
build_component "transformer-core" "crates/transformer-core"
build_component "transformer-ffi" "crates/transformer-ffi"

# Build desktop app
echo "🖥️  Building Desktop App"
echo "======================="
build_component "desktop app" "apps/desktop"

# Build web if wasm-pack is installed
if command -v wasm-pack &> /dev/null; then
    echo "🌐 Building Web App"
    echo "==================="
    echo -e "${BLUE}Building WASM module...${NC}"
    (cd apps/web && wasm-pack build --target web --out-dir pkg)
    echo -e "${GREEN}✓ WASM built successfully${NC}"
    echo ""
else
    echo "⚠️  wasm-pack not found, skipping Web build"
    echo "   Install with: cargo install wasm-pack"
    echo ""
fi

# iOS build (macOS only)
if [[ "$OSTYPE" == "darwin"* ]]; then
    if [ -f "apps/ios/build.sh" ]; then
        echo "📱 Building iOS Libraries"
        echo "========================"
        (cd apps/ios && ./build.sh)
        echo ""
    fi
else
    echo "⚠️  iOS build skipped (macOS required)"
    echo ""
fi

# Android build (if cargo-ndk is installed)
if command -v cargo-ndk &> /dev/null && [ ! -z "$ANDROID_NDK_HOME" ]; then
    echo "🤖 Building Android Libraries"
    echo "============================="
    (cd apps/android && ./build.sh)
    echo ""
else
    echo "⚠️  Android build skipped"
    echo "   Requires: cargo-ndk and ANDROID_NDK_HOME"
    echo ""
fi

echo ""
echo -e "${GREEN}✅ Build complete!${NC}"
echo ""
echo "Run the desktop app:"
echo "  ./target/release/transformer-desktop"
echo ""
echo "Serve the web app:"
echo "  cd apps/web && python3 -m http.server 8080"
