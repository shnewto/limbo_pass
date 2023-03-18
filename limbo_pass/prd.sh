#!/usr/local/bin/bash
rm -rf dist
trunk build --release
#wasm-opt -Oz dist/*.wasm -o dist/*.wasm
vercel --prod