#!/usr/local/bin/bash
trunk build --release
wasm-opt -Oz dist/*.wasm -o dist/*.wasm
vercel --prod