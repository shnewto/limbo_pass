#!/bin/bash
set -e

# Get the project root (where this script is located)
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_ROOT"

# Build the WASM binary
echo "Building WASM binary..."
cargo build --target wasm32-unknown-unknown --release --bin limbo_pass

# Create output directory in limbo_pass
OUTPUT_DIR="limbo_pass/dist"
mkdir -p "$OUTPUT_DIR"

# Copy assets
echo "Copying assets..."
cp -r limbo_pass/assets "$OUTPUT_DIR/"

# Find the WASM file (could be in parent or child target directory)
WASM_FILE=""
if [[ -f "target/wasm32-unknown-unknown/release/limbo_pass.wasm" ]]; then
    WASM_FILE="target/wasm32-unknown-unknown/release/limbo_pass.wasm"
elif [[ -f "limbo_pass/target/wasm32-unknown-unknown/release/limbo_pass.wasm" ]]; then
    WASM_FILE="limbo_pass/target/wasm32-unknown-unknown/release/limbo_pass.wasm"
else
    echo "Error: Could not find limbo_pass.wasm"
    exit 1
fi

# Generate WASM bindings
echo "Generating WASM bindings..."
wasm-bindgen \
    --target web \
    --out-dir "$OUTPUT_DIR" \
    --no-typescript \
    "$WASM_FILE"

# Copy and update index.html
echo "Copying index.html..."
cp limbo_pass/index.html "$OUTPUT_DIR/"
# Update the import path in index.html to match the generated file
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS sed
    sed -i '' 's|./target/wasm32-unknown-unknown/release/limbo_pass.js|./limbo_pass.js|g' "$OUTPUT_DIR/index.html"
else
    # Linux sed
    sed -i 's|./target/wasm32-unknown-unknown/release/limbo_pass.js|./limbo_pass.js|g' "$OUTPUT_DIR/index.html"
fi

# Copy _redirects file if it exists
if [[ -f "limbo_pass/_redirects" ]]; then
    cp limbo_pass/_redirects "$OUTPUT_DIR/"
fi

echo "Build complete! Output in $OUTPUT_DIR/"

