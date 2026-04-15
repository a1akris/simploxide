#!/usr/bin/bash

set -e

# -------- SUBMODULE INIT
git submodule init
git submodule update --remote

# -------- GEN TYPES AND FFI INTERFACE
cd simploxide-bindgen/
cargo run
./bindffi.sh
mv generated/*rs ../simploxide-api-types/src/
mv generated/ffi/*rs ../simploxide-sxcrt-sys/src/
mv generated/ffi/cabal.project ../simploxide-sxcrt-sys/simplex-static/cabal.project

# -------- CHECK TYPES
cd ../simploxide-api-types/
cargo fmt
cargo clippy --all-features --all-targets

# -------- CHECK FFI
cd ../simploxide-sxcrt-sys
cargo fmt

cd ../

echo ""
echo "[+] API types generated successfully"
echo ""
