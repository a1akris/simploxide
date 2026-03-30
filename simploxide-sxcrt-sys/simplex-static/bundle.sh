#!/usr/bin/env bash
# Merges all Haskell .a files from cabal-store into libsimplex.a.
# GHC boot libs (base, rts, ghc-prim, etc.) are NOT bundled — they are NOT PIC-compiled
# and would break PIE linking. Consumers must link them dynamically from their GHC installation.
# Run 'make build' before this to populate cabal-store.
set -e

GHC_VER=$(ghc --numeric-version)
STORE_DIR="$(pwd)/cabal-store"
OUT="$(pwd)/libsimplex.a"

echo "GHC $GHC_VER"
echo "Store: $STORE_DIR"

[ -d "$STORE_DIR" ] || { echo "ERROR: cabal store not found — run 'make build' first"; exit 1; }

# Only cabal-store packages (no GHC boot .a files — those lack -fPIC)
mapfile -d '' ARCHIVES < <(
    find "$STORE_DIR/ghc-$GHC_VER" \
        -name "libHS*.a" ! -name "*_p.a" ! -name "*_debug*.a" \
        -print0 2>/dev/null
)

echo "Found ${#ARCHIVES[@]} archives to merge"

# Use ar -M MRI script to concatenate archives directly —
# avoids extracting objects, so non-.o extensions (e.g. .thr_o in RTS) are handled correctly
MRI=$(mktemp)
trap 'rm -f "$MRI"' EXIT

{
    echo "CREATE $OUT"
    printf 'ADDLIB %s\n' "${ARCHIVES[@]}"
    echo "SAVE"
    echo "END"
} > "$MRI"

echo "Packing $OUT via ar -M ..."
rm -f "$OUT"
ar -M < "$MRI"

echo "Stripping unneeded symbols..."
strip --strip-unneeded "$OUT"

echo ""
echo "=== Verifying FFI exports ==="
nm "$OUT" 2>/dev/null | grep -E "T (hs_init_with_rtsopts|chat_migrate_init|chat_send_cmd)" \
    || echo "WARNING: expected symbols not found"

echo ""
echo "Done: $OUT ($(du -sh "$OUT" | cut -f1))"
echo ""
if [ "${SQLCIPHER_LINKAGE:-static}" = "dynamic" ]; then
    echo "Consumers must link with: -lsqlcipher -lcrypto -lssl -lffi -lgmp -lpthread -lm"
else
    echo "Consumers must link with: -lcrypto -lssl -lffi -lgmp -lpthread -lm"
fi
echo "GHC boot libs must be linked dynamically from: \$(ghc --print-libdir)"
