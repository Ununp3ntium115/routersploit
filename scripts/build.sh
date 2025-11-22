#!/bin/bash
# PyRouterSploit Build Script

set -e

echo "🚀 Building PyRouterSploit..."

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

# Build Rust core
echo -e "${BLUE}Building Rust core...${NC}"
cd rust
cargo build --release
cd ..
echo -e "${GREEN}✓ Rust core built${NC}"

# Create data directory
echo -e "${BLUE}Creating data directory...${NC}"
mkdir -p data
echo -e "${GREEN}✓ Data directory created${NC}"

# Initialize database
echo -e "${BLUE}Initializing database...${NC}"
./rust/target/release/pyroutersploit init --populate-cryptex
echo -e "${GREEN}✓ Database initialized${NC}"

# Build UI (optional)
if command -v npm &> /dev/null; then
    echo -e "${BLUE}Building Svelte UI...${NC}"
    cd ui
    if [ ! -d "node_modules" ]; then
        npm install
    fi
    npm run build
    cd ..
    echo -e "${GREEN}✓ UI built${NC}"
else
    echo "⚠ npm not found, skipping UI build"
fi

# Build Node-RED nodes (optional)
if command -v npm &> /dev/null; then
    echo -e "${BLUE}Building Node-RED nodes...${NC}"
    cd nodered
    if [ ! -d "node_modules" ]; then
        npm install
    fi
    cd ..
    echo -e "${GREEN}✓ Node-RED nodes ready${NC}"
fi

echo -e "${GREEN}✅ Build complete!${NC}"
echo ""
echo "Run PyRouterSploit:"
echo "  ./rust/target/release/pyroutersploit --help"
echo ""
echo "Start API server:"
echo "  ./rust/target/release/pyroutersploit serve"
echo ""
echo "Start MCP server:"
echo "  ./rust/target/release/pyroutersploit mcp"
