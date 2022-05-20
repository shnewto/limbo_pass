#!/usr/local/bin/bash
wasm-opt -Oz dist/*.wasm -o dist/*.wasm
vercel --prod