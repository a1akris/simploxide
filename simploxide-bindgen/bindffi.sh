#!/bin/bash

set -euo pipefail

if [[ ! -f "./simplex-chat/cabal.project" ]]; then
    echo "ERROR: simplex-chat submodule not initialised."
    echo "Run: git submodule update --init"
    exit 1
fi

OUT_DIR="generated/ffi"

mkdir -p "$OUT_DIR"

out="$OUT_DIR/bindings.rs"

bindgen \
    --no-copy '.*' \
    ./simplex-chat/packages/simplex-chat-nodejs/cpp/simplex.h \
    -o "$OUT_DIR/bindings.rs" \
    -- -x c++

echo "written: $out"

commit=$(git -C "./simplex-chat" rev-parse HEAD)
echo "simplex-chat-ref: ${commit}"

srps=$(awk '
    /^source-repository-package/ { in_srp = 1 }
    in_srp { print }
    /^[[:space:]]*$/ { in_srp = 0 }
' "./simplex-chat/cabal.project")

out="$OUT_DIR/cabal.project"

cat <<EOF > "$out"
source-repository-package
    type: git
    location: https://github.com/simplex-chat/simplex-chat
    tag: ${commit}

EOF

echo "$srps" >> "$out"

cat <<EOF >> "$out"

constraints: zip +disable-bzip2 +disable-zstd

package *
  ghc-options: -split-sections -fPIC -fexternal-dynamic-refs -optc-fPIC

package simplex-chat
  flags: +client_library

package cryptostore
  flags: +use_crypton

package direct-sqlcipher
  flags: +openssl
EOF

echo "written: $out"
