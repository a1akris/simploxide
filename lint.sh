#!/bin/bash

set -e

cd ./simploxide-bindgen
cargo fmt --check --verbose
cargo clippy --all-features --all-targets -- -D warnings
cargo test --all-features --all-targets

cd ../
cargo fmt --check --verbose
cargo clippy --workspace --all-features --all-targets -- -D warnings
cargo test --workspace --all-features --all-targets
