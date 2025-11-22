#!/bin/bash
# PyRouterSploit Development Environment Launcher

set -e

echo "🚀 Starting PyRouterSploit development environment..."

# Start API server in background
echo "Starting API server on port 8080..."
./rust/target/release/pyroutersploit serve --host 127.0.0.1 --port 8080 &
API_PID=$!

# Wait for API to start
sleep 2

# Start UI in background
if command -v npm &> /dev/null && [ -d "ui" ]; then
    echo "Starting Svelte UI on port 5173..."
    cd ui
    npm run dev &
    UI_PID=$!
    cd ..
fi

echo ""
echo "✅ Development environment running!"
echo ""
echo "API Server: http://localhost:8080"
echo "Web UI: http://localhost:5173"
echo ""
echo "Press Ctrl+C to stop..."

# Trap SIGINT and kill background processes
trap "kill $API_PID $UI_PID 2>/dev/null; exit" INT

wait
