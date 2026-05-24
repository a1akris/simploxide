#!/usr/bin/env bash
# Merges all Haskell .a files from cabal-store into libsimplex.a.
# GHC boot libs (base, rts, ghc-prim, etc.) are NOT bundled by default — they are NOT
# PIC-compiled and would break PIE linking. Consumers must link them dynamically from
# their GHC installation.
#
# Pass --include-ghc-boot to also bundle GHC boot .a archives for a fully static build.
# All downstream binaries then must be linked with -no-pie.
#
# Run 'make build' (or 'make autobuild') before this to populate cabal-store.
set -e

INCLUDE_GHC_BOOT=0
for arg in "$@"; do
    [[ "$arg" == "--include-ghc-boot" ]] && INCLUDE_GHC_BOOT=1
done

OS="$(uname -s)"
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

echo "Found ${#ARCHIVES[@]} cabal-store archives to merge"

if [[ $INCLUDE_GHC_BOOT -eq 1 ]]; then
    # Locate GHC boot .a files — mirrors the needed_ghc_lib() filter in build.rs/collect-ghc-libs.sh
    # but selects static archives instead of shared libraries.
    LIBDIR=$(ghc --print-libdir)
    GHC_PLATFORM_DIR=$(find "$LIBDIR" -maxdepth 1 -type d -name '*-ghc-*' | head -1)

    if [[ -z "$GHC_PLATFORM_DIR" ]]; then
        echo "ERROR: GHC platform dir (<arch>-<os>-ghc-*) not found under $LIBDIR" >&2
        exit 1
    fi

    echo "Including GHC boot libs from: $GHC_PLATFORM_DIR"

    needed_ghc_boot_archive() {
        local filename="$1"
        [[ "$filename" == lib*.a ]] || return 1
        local stem="${filename#lib}"
        stem="${stem%.a}"

        # RTS: only the threaded non-debug variant
        if [[ "$stem" == HSrts-* ]]; then
            [[ "$stem" == *_thr* ]] && [[ "$stem" != *debug* ]] && return 0
            return 1
        fi

        local needed=(
            "HSarray-" "HSbase-" "HSbinary-" "HSbytestring-"
            "HScontainers-" "HSdeepseq-" "HSdirectory-" "HSexceptions-"
            "HSfilepath-" "HSghc-bignum-" "HSghc-boot-th-" "HSghc-prim-"
            "HSinteger-gmp-" "HSmtl-" "HSparsec-" "HSpretty-"
            "HSprocess-" "HSstm-" "HStemplate-haskell-" "HStext-"
            "HStime-" "HStransformers-" "HSunix-"
        )
        for prefix in "${needed[@]}"; do
            [[ "$stem" == "$prefix"* ]] && return 0
        done
        return 1
    }

    mapfile -d '' ALL_GHC_ARCHIVES < <(
        find "$GHC_PLATFORM_DIR" \
            -maxdepth 2 \
            -name "libHS*.a" ! -name "*_p.a" ! -name "*debug*.a" \
            -print0 2>/dev/null
    )

    GHC_BOOT_ARCHIVES=()
    for path in "${ALL_GHC_ARCHIVES[@]}"; do
        if needed_ghc_boot_archive "$(basename "$path")"; then
            GHC_BOOT_ARCHIVES+=("$path")
        fi
    done

    echo "Found ${#GHC_BOOT_ARCHIVES[@]} GHC boot archives to bundle"
fi

# Merge all archives into libsimplex.a.
# GNU ar supports MRI scripts (-M) which merge archives without extracting objects — important
# for the GHC RTS whose objects use non-standard extensions like .thr_o.
# macOS ar (BSD/LLVM) lacks MRI support; use libtool -static instead, which also operates at
# the archive level without extraction.
echo "Packing $OUT ..."
rm -f "$OUT"

if [[ "$OS" == "Darwin" ]]; then
    ALL_ARCHIVES=("${ARCHIVES[@]}")
    [[ $INCLUDE_GHC_BOOT -eq 1 ]] && ALL_ARCHIVES+=("${GHC_BOOT_ARCHIVES[@]}")
    libtool -static -o "$OUT" "${ALL_ARCHIVES[@]}"
    strip -S "$OUT"
else
    MRI=$(mktemp)
    trap 'rm -f "$MRI"' EXIT
    {
        echo "CREATE $OUT"
        printf 'ADDLIB %s\n' "${ARCHIVES[@]}"
        if [[ $INCLUDE_GHC_BOOT -eq 1 ]]; then
            printf 'ADDLIB %s\n' "${GHC_BOOT_ARCHIVES[@]}"
        fi
        echo "SAVE"
        echo "END"
    } > "$MRI"
    ar -M < "$MRI"
    strip --strip-unneeded "$OUT"
fi

echo ""
echo "Done: $OUT ($(du -sh "$OUT" | cut -f1))"
echo ""

if [[ $INCLUDE_GHC_BOOT -eq 1 ]]; then
    echo "GHC boot libs bundled in $OUT."
    echo "All downstream binaries must be linked with -no-pie."
else
    echo "GHC boot libs must be linked dynamically from: $(ghc --print-libdir)"
fi
