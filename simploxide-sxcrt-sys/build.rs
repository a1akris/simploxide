type BuildResult<T = ()> = ::std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> BuildResult {
    println!("cargo:rerun-if-env-changed=SXCRT");
    println!("cargo:rerun-if-env-changed=SIMPLEX_STATIC_DIR");
    println!("cargo:rerun-if-env-changed=GHC_LIBS");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/bindings.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=simplex-static/Makefile");
    println!("cargo:rerun-if-changed=simplex-static/bundle.sh");
    println!("cargo:rerun-if-changed=simplex-static/cabal.project");

    let sxcrt = std::env::var("SXCRT");
    let static_dir = std::env::var("SIMPLEX_STATIC_DIR");

    match (sxcrt, static_dir) {
        (Ok(_), Ok(_)) => Err("SXCRT and SIMPLEX_STATIC_DIR cannot be set at the same time".into()),
        // Dynamic path: pre-built .so bundle from SimpleX-Chat team (or custom dir)
        (Ok(runtime_dir), Err(_)) => link_with_sxcrt(runtime_dir),
        // Static path: user-provided simplex-static dir, build artifacts stay there
        (Err(_), Ok(dir)) => {
            let dir = std::fs::canonicalize(&dir)
                .map_err(|_| format!("SIMPLEX_STATIC_DIR not found: {dir}"))?;
            link_with_static_lib(dir)
        }
        // Static path: copy embedded simplex-static into OUT_DIR and build there
        (Err(_), Err(_)) => {
            if std::env::var("CARGO_FEATURE_BUILD_SXCRT").is_err() {
                return Err("set SXCRT to a directory with pre-built SimpleX .so files, or enable the build-sxcrt feature to build libsimplex.a from source".into());
            }
            let out_dir = std::env::var("OUT_DIR")?;
            let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
            let src = std::path::PathBuf::from(manifest_dir).join("simplex-static");
            let build_dir = std::path::PathBuf::from(out_dir).join("simplex-static");
            std::fs::create_dir_all(&build_dir)?;
            for file in &["Makefile", "bundle.sh", "cabal.project"] {
                std::fs::copy(src.join(file), build_dir.join(file))?;
            }
            link_with_static_lib(build_dir)
        }
    }
}

// SXCRT must point to a directory with libsimplex.so and all Haskell .so dependencies.
// Use SimpleX-Chat team so distributions or custo prebuilt ones
fn link_with_sxcrt(runtime_dir: String) -> BuildResult {
    let runtime_path = std::fs::canonicalize(runtime_dir)?;
    let libsimplex_path = runtime_path.join("libsimplex.so");

    if !libsimplex_path.exists() {
        return Err(format!("Cannot find libsimplex.so at {}", runtime_path.display()).into());
    }

    println!("cargo:rustc-link-search=native={}", runtime_path.display());
    // rpath for local tests
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", runtime_path.display());

    // Link all .so files found in runtime explicitly because -rpath-link doesn't work for some
    // reason
    println!("cargo:rustc-link-arg=-Wl,--start-group");
    for entry in std::fs::read_dir(runtime_path).expect("failed to read runtime dir") {
        let entry = entry.unwrap();
        let path = entry.path();

        if let Some(ext) = path.extension()
            && ext == "so"
        {
            // Extract the lib name: libfoo.so -> foo
            if let Some(stem) = path.file_stem() {
                let name = stem.to_str().unwrap();
                if let Some(name) = name.strip_prefix("lib") {
                    println!("cargo:rustc-link-lib=dylib={}", name);
                }
            }
        }
    }
    println!("cargo:rustc-link-arg=-Wl,--end-group");
    Ok(())
}

// Static path builds libsimplex.a inside `dir` if not already present,
// then links it together with GHC boot .so files.
//
// Required: GHC must be in PATH (needed both to build and to locate boot .so files).
//
// Environment variables:
//   SIMPLEX_STATIC_DIR  path to a simplex-static project dir overriding the embedded one
//   SQLCIPHER_LINKAGE   passed through to make: "dyanmic" (default) or "static"
//   GHC_LIBS            dir containing GHC boot .so files
//                       (default: auto-detected via `ghc --print-libdir`)
fn link_with_static_lib(dir: std::path::PathBuf) -> BuildResult {
    let libsimplex = dir.join("libsimplex.a");

    // Build only if libsimplex.a is absent. Delete it to force a rebuild.
    if !libsimplex.exists() {
        build_libsimplex(&dir)?;
    }

    // Link libsimplex.a statically
    println!("cargo:rustc-link-search=native={}", dir.display());
    println!("cargo:rustc-link-lib=static=simplex");

    // C dependencies of simplex-chat
    let embed_sqlcipher = std::env::var("CARGO_FEATURE_EMBED_SQLCIPHER").is_ok();
    if !embed_sqlcipher {
        println!("cargo:rustc-link-lib=sqlcipher");
    }
    for lib in &["crypto", "ssl", "ffi", "gmp", "z", "bz2", "pthread", "m"] {
        println!("cargo:rustc-link-lib={lib}");
    }

    // GHC boot libs (.so) — not bundled in libsimplex.a because they lack -fPIC.
    // Both link-time and runtime paths must point to the same directory.
    let ghc_libs = ghc_libs_dir()?;
    println!("cargo:rustc-link-search=native={}", ghc_libs.display());
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", ghc_libs.display());

    println!("cargo:rustc-link-arg=-Wl,--start-group");
    for entry in std::fs::read_dir(&ghc_libs)? {
        let path = entry?.path();
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            if let Some(lib_name) = needed_ghc_lib(filename) {
                println!("cargo:rustc-link-lib=dylib={lib_name}");
            }
        }
    }
    println!("cargo:rustc-link-arg=-Wl,--end-group");

    Ok(())
}

fn build_libsimplex(dir: &std::path::Path) -> BuildResult {
    let sqlcipher_linkage = if std::env::var("CARGO_FEATURE_EMBED_SQLCIPHER").is_ok() {
        "static"
    } else {
        "dynamic"
    };
    let status = std::process::Command::new("make")
        .arg("bundle")
        .arg(format!("SQLCIPHER_LINKAGE={sqlcipher_linkage}"))
        .current_dir(dir)
        .status()?;
    if !status.success() {
        return Err(format!("make bundle failed in {}", dir.display()).into());
    }
    Ok(())
}

// Returns the directory containing GHC boot .so files.
// Uses GHC_LIBS env var if set, otherwise runs `ghc --print-libdir` and
// finds the platform-specific subdir (e.g. x86_64-linux-ghc-9.6.7/).
fn ghc_libs_dir() -> BuildResult<std::path::PathBuf> {
    if let Ok(dir) = std::env::var("GHC_LIBS") {
        let path = std::path::PathBuf::from(&dir);
        if !path.exists() {
            return Err(format!("GHC_LIBS dir not found: {dir}").into());
        }
        return Ok(path);
    }

    let output = std::process::Command::new("ghc")
        .arg("--print-libdir")
        .output()
        .map_err(|_| "ghc not found in PATH — required for static linking")?;

    if !output.status.success() {
        return Err("ghc --print-libdir failed".into());
    }

    let libdir = std::path::PathBuf::from(String::from_utf8(output.stdout)?.trim());

    // Find the platform subdir: x86_64-linux-ghc-X.Y.Z/
    std::fs::read_dir(&libdir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| {
            p.is_dir()
                && p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with("x86_64-linux-ghc-"))
        })
        .ok_or_else(|| {
            "GHC platform dir (x86_64-linux-ghc-*) not found under ghc --print-libdir".into()
        })
}

// Returns the link name (strip "lib" prefix and ".so" suffix) for GHC boot .so files
// that are actually needed at runtime, or None to skip the file.
//
// The required set was determined by computing truly external symbols in libsimplex.a
// (undefined and not defined anywhere in the archive) and matching them against each .so.
// Packages with zero matches are excluded.
fn needed_ghc_lib(filename: &str) -> Option<&str> {
    const NEEDED: &[&str] = &[
        "HSarray-",
        "HSbase-",
        "HSbinary-",
        "HSbytestring-",
        "HScontainers-",
        "HSdeepseq-",
        "HSdirectory-",
        "HSexceptions-",
        "HSfilepath-",
        "HSghc-boot-th-",
        "HSghc-bignum-",
        "HSghc-prim-",
        "HSinteger-gmp-",
        "HSmtl-",
        "HSparsec-",
        "HSpretty-",
        "HSprocess-",
        "HSstm-",
        "HStemplate-haskell-",
        "HStext-",
        "HStime-",
        "HStransformers-",
        "HSunix-",
    ];

    let stem = filename.strip_prefix("lib")?.strip_suffix(".so")?;

    // RTS: only the threaded non-debug variant
    if stem.starts_with("HSrts-") {
        return (stem.contains("_thr") && !stem.contains("debug")).then_some(stem);
    }

    // All other boot packages needed by libsimplex.a (zero-match packages omitted)
    NEEDED.iter().any(|p| stem.starts_with(p)).then_some(stem)
}
