#!/usr/bin/env sh

touch src/lib.rs
cargo clippy --all-targets --all-features -- -D warnings
