#!/bin/bash
set -e

# Get the project root (where this script is located)
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_ROOT"

# Install Rust if not present
if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
    export PATH="$HOME/.cargo/bin:$PATH"
    source "$HOME/.cargo/env" 2>/dev/null || true
fi

# Ensure cargo is in PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Install wasm32 target if not present
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Install wasm-bindgen-cli if not present
if ! command -v wasm-bindgen &> /dev/null; then
    echo "Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli --version 0.2.106
fi

# Install wasm-opt if not present
if ! command -v wasm-opt &> /dev/null; then
    echo "Installing wasm-opt..."
    cargo install wasm-opt
fi

# Build the WASM binary with size optimizations
echo "Building WASM binary with size optimizations..."
# Use release for production, but can use debug for faster iteration
if [ "${BUILD_MODE:-release}" = "debug" ]; then
    cargo build --target wasm32-unknown-unknown
else
    RUSTFLAGS="-C link-arg=-zstack-size=8388608" \
    cargo build --target wasm32-unknown-unknown --release
fi

# Create output directory
OUTPUT_DIR="dist"
mkdir -p "$OUTPUT_DIR"

# Copy assets (including .meta files)
echo "Copying assets..."
cp -r assets "$OUTPUT_DIR/"

# Find the WASM file
WASM_FILE="target/wasm32-unknown-unknown/release/limbo_pass.wasm"
if [ "${BUILD_MODE:-release}" = "debug" ]; then
    WASM_FILE="target/wasm32-unknown-unknown/debug/limbo_pass.wasm"
fi

if [[ ! -f "$WASM_FILE" ]]; then
    echo "Error: Could not find $WASM_FILE"
    exit 1
fi

# Generate WASM bindings
echo "Generating WASM bindings..."
wasm-bindgen \
    --target web \
    --out-dir "$OUTPUT_DIR" \
    --no-typescript \
    "$WASM_FILE"

# Compress WASM file to reduce size (Cloudflare Pages has a 25MB limit)
echo "Optimizing WASM file..."
ORIGINAL_SIZE=$(du -h "$OUTPUT_DIR/limbo_pass_bg.wasm" | cut -f1)
if command -v wasm-opt &> /dev/null; then
    # Use size optimization: -Os (balance between size and speed) with additional flags
    # Enable required WASM features for validation
    wasm-opt -Os --strip-debug --strip-producers --enable-bulk-memory --enable-nontrapping-float-to-int -o "$OUTPUT_DIR/limbo_pass_bg.wasm.opt" "$OUTPUT_DIR/limbo_pass_bg.wasm"
    mv "$OUTPUT_DIR/limbo_pass_bg.wasm.opt" "$OUTPUT_DIR/limbo_pass_bg.wasm"
    OPTIMIZED_SIZE=$(du -h "$OUTPUT_DIR/limbo_pass_bg.wasm" | cut -f1)
    echo "WASM optimized: $ORIGINAL_SIZE -> $OPTIMIZED_SIZE"
    
    # Check if still too large
    SIZE_BYTES=$(stat -f%z "$OUTPUT_DIR/limbo_pass_bg.wasm" 2>/dev/null || stat -c%s "$OUTPUT_DIR/limbo_pass_bg.wasm" 2>/dev/null)
    SIZE_MB=$((SIZE_BYTES / 1024 / 1024))
    if [ "$SIZE_MB" -gt 25 ]; then
        echo "Warning: WASM file is still ${SIZE_MB}MB (limit is 25MB)"
        echo "Note: You may need to reduce dependencies or use code splitting to get under the limit"
    fi
else
    echo "Warning: wasm-opt not found. Install it with: cargo install wasm-opt"
    echo "WASM file size: $ORIGINAL_SIZE"
fi

# Copy sound.js and index.html
echo "Copying sound.js and index.html..."
cp sound.js "$OUTPUT_DIR/" 2>/dev/null || true
cp index.html "$OUTPUT_DIR/"
# Update the import path in index.html to match the generated file
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS sed
    sed -i '' 's|./target/wasm32-unknown-unknown/release/limbo_pass.js|./limbo_pass.js|g' "$OUTPUT_DIR/index.html"
else
    # Linux sed
    sed -i 's|./target/wasm32-unknown-unknown/release/limbo_pass.js|./limbo_pass.js|g' "$OUTPUT_DIR/index.html"
fi

# Copy _redirects file if it exists
if [[ -f "_redirects" ]]; then
    cp _redirects "$OUTPUT_DIR/"
fi

echo "Build complete! Output in $OUTPUT_DIR/"

