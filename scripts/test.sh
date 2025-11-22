#!/bin/bash
# PyRouterSploit Test Script

set -e

echo "🧪 Running PyRouterSploit tests..."

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

# Rust tests
echo -e "${BLUE}Running Rust unit tests...${NC}"
cd rust
cargo test
echo -e "${GREEN}✓ Unit tests passed${NC}"

# Rust integration tests
echo -e "${BLUE}Running integration tests...${NC}"
cargo test --test '*'
echo -e "${GREEN}✓ Integration tests passed${NC}"

# Rust benchmarks (optional)
if [ "$1" = "--bench" ]; then
    echo -e "${BLUE}Running benchmarks...${NC}"
    cargo bench
fi

cd ..

echo -e "${GREEN}✅ All tests passed!${NC}"
