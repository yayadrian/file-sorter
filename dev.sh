#!/bin/bash
# Quick start script for development

echo "🚀 Starting File Sorter development environment..."

# Check if npm dependencies are installed
if [ ! -d "node_modules" ]; then
    echo "📦 Installing npm dependencies..."
    npm install
fi

# Check if Cargo.lock exists (Rust dependencies)
if [ ! -f "src-tauri/Cargo.lock" ]; then
    echo "🦀 Building Rust dependencies (first time may take a while)..."
    cd src-tauri && cargo build && cd ..
fi

echo "✨ Starting development server..."
npm run tauri dev
