#!/usr/bin/env bash

$RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' && wasm-pack build --target web --out-dir dist -- --features=parallel -Zbuild-std=panic_abort,std
cp index.html wasm-worker.js dist