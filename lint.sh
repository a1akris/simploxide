#!/bin/bash

set -e

cd ./simploxide-bindgen
cargo fmt --check --verbose
cargo clippy --all-features --all-targets -- -D warnings
cargo test --all-features --all-targets

cd ../
cargo fmt --check --verbose
cargo clippy -p simploxide-ws-core -p simploxide-core -p simploxide-api-types --all-features --all-targets -- -D warnings
cargo clippy -p simploxide-client --features fullcli -- -D warnings
