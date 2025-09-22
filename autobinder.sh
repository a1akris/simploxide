#!/usr/bin/bash

set -e

git submodule init
git submodule update --remote
cd simploxide-bindgen/
cargo run
mv generated/*rs ../simploxide-api-types/src/
cd ../simploxide-api-types/
cargo fmt
cargo clippy --all-features --all-targets
cd ../

echo ""
echo "[+] API types generated successfully"
echo ""
