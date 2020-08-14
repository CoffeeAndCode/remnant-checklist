#!/usr/bin/env sh

cargo test --all-features --color=always --quiet
wasm-pack test --chrome --headless -- --color=always
