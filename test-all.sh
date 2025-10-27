#!/bin/bash
# Test script for all components

set -e

echo "🧪 Running PlankTransformer Test Suite"
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

test_component() {
    local name=$1
    local path=$2
    echo -e "${BLUE}Testing $name...${NC}"
    (cd "$path" && cargo test)
    echo -e "${GREEN}✓ $name tests passed${NC}"
    echo ""
}

# Test core libraries
echo "📦 Testing Core Libraries"
echo "========================="
test_component "transformer-core" "crates/transformer-core"
test_component "transformer-ffi" "crates/transformer-ffi"

# Test desktop app
echo "🖥️  Testing Desktop App"
echo "======================"
test_component "desktop app" "apps/desktop"

echo ""
echo -e "${GREEN}✅ All tests passed!${NC}"
