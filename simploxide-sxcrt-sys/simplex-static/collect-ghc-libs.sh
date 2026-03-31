#!/usr/bin/env bash
# collect-ghc-libs.sh <dest-dir>
#
# Copies the GHC boot .so files required at runtime by libsimplex.a into <dest-dir>.
# The selected set mirrors the needed_ghc_lib() filter in build.rs exactly.
#
# Also copies libsqlcipher.so (built by build-sqlcipher.sh / make sqlcipher) if it
# exists next to this script.  Shipping our own libsqlcipher.so in sxcrt/ ensures
# that consumers load the exact SQLCipher version that simplex-chat was compiled
# against, regardless of what the system has installed.
#
# By default the source dir is auto-detected via `ghc --print-libdir`.
# Set GHC_LIBS to override.

set -euo pipefail

if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <dest-dir>" >&2
    exit 1
fi

DEST="$1"
mkdir -p "$DEST"

# --- locate GHC boot libs dir (mirrors ghc_libs_dir() in build.rs) ---

if [[ -n "${GHC_LIBS:-}" ]]; then
    if [[ ! -d "$GHC_LIBS" ]]; then
        echo "error: GHC_LIBS dir not found: $GHC_LIBS" >&2
        exit 1
    fi
    GHC_LIB_DIR="$GHC_LIBS"
else
    if ! command -v ghc &>/dev/null; then
        echo "error: ghc not found in PATH" >&2
        exit 1
    fi
    LIBDIR="$(ghc --print-libdir)"
    GHC_LIB_DIR="$(find "$LIBDIR" -maxdepth 1 -type d -name 'x86_64-linux-ghc-*' | head -1)"
    if [[ -z "$GHC_LIB_DIR" ]]; then
        echo "error: GHC platform dir (x86_64-linux-ghc-*) not found under $LIBDIR" >&2
        exit 1
    fi
fi

echo "Collecting from: $GHC_LIB_DIR"

# --- needed_ghc_lib() filter (mirrors build.rs exactly) ---

needed_ghc_lib() {
    local filename="$1"
    # strip lib prefix and .so suffix
    [[ "$filename" == lib*.so ]] || return 1
    local stem="${filename#lib}"
    stem="${stem%.so}"

    # RTS: only the threaded non-debug variant
    if [[ "$stem" == HSrts-* ]]; then
        if [[ "$stem" == *_thr* ]] && [[ "$stem" != *debug* ]]; then
            return 0
        fi
        return 1
    fi

    # All other boot packages
    local needed=(
        "HSarray-"
        "HSbase-"
        "HSbinary-"
        "HSbytestring-"
        "HScontainers-"
        "HSdeepseq-"
        "HSdirectory-"
        "HSexceptions-"
        "HSfilepath-"
        "HSghc-bignum-"
        "HSghc-boot-th-"
        "HSghc-prim-"
        "HSinteger-gmp-"
        "HSmtl-"
        "HSparsec-"
        "HSpretty-"
        "HSprocess-"
        "HSstm-"
        "HStemplate-haskell-"
        "HStext-"
        "HStime-"
        "HStransformers-"
        "HSunix-"
    )

    for prefix in "${needed[@]}"; do
        if [[ "$stem" == "$prefix"* ]]; then
            return 0
        fi
    done
    return 1
}

# --- copy matching files ---

count=0
for filepath in "$GHC_LIB_DIR"/lib*.so; do
    [[ -e "$filepath" ]] || continue
    filename="$(basename "$filepath")"
    if needed_ghc_lib "$filename"; then
        cp -v "$filepath" "$DEST/$filename"
        (( count++ )) || true
    fi
done

echo "Collected $count GHC boot .so files into $DEST"
