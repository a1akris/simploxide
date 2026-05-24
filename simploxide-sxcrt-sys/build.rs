type BuildResult<T = ()> = ::std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> BuildResult {
    // docs.rs cannot install the SimpleX runtime libraries; skip linking there.
    if std::env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    println!("cargo:rerun-if-env-changed=SIMPLEX_STATIC_AUTOBUILD");
    println!("cargo:rerun-if-env-changed=SXCRT");
    println!("cargo:rerun-if-env-changed=SIMPLEX_STATIC_DIR");
    println!("cargo:rerun-if-env-changed=SIMPLEX_STATIC_BUNDLED");
    println!("cargo:rerun-if-env-changed=GHC_LIBS");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/bindings.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=simplex-static/Makefile");
    println!("cargo:rerun-if-changed=simplex-static/bundle.sh");
    println!("cargo:rerun-if-changed=simplex-static/cabal.project");

    if std::env::var("SIMPLEX_STATIC_AUTOBUILD").is_ok_and(|v| v == "1") {
        return link_with_autobuild();
    }

    let sxcrt = std::env::var("SXCRT");
    let static_dir = std::env::var("SIMPLEX_STATIC_DIR");

    match (sxcrt, static_dir) {
        (Ok(_), Ok(_)) => Err("SXCRT and SIMPLEX_STATIC_DIR cannot be set at the same time".into()),
        // Dynamic path: pre-built shared library bundle from SimpleX-Chat team (or custom dir)
        (Ok(runtime_dir), Err(_)) => link_with_sxcrt(runtime_dir),
        // Static path: user-provided simplex-static dir, build artifacts stay there
        (Err(_), Ok(dir)) => link_with_static_lib(dir),
        (Err(_), Err(_)) => Err(
            "set SXCRT to use a pre-built SimpleX shared library bundle, \
             or set SIMPLEX_STATIC_DIR to link libsimplex.a manually  \
             or set SIMPLEX_STATIC_AUTOBUILD=1 for a fully automatic static build"
                .into(),
        ),
    }
}

// Returns the platform dylib extension without a dot: "so", "dylib", or "dll".
fn dylib_ext() -> &'static str {
    match std::env::var("CARGO_CFG_TARGET_OS")
        .as_deref()
        .unwrap_or("")
    {
        "macos" => "dylib",
        _ => "so",
    }
}

// --start-group / --end-group may not be recognized on Apple
fn uses_link_groups() -> bool {
    !matches!(std::env::var("CARGO_CFG_TARGET_OS").as_deref(), Ok("macos"))
}

// SXCRT must point to a directory with libsimplex and all Haskell shared library
// dependencies. Use SimpleX-Chat team distributions or custom prebuilt ones.
fn link_with_sxcrt(runtime_dir: String) -> BuildResult {
    let runtime_path = std::fs::canonicalize(&runtime_dir)
        .map_err(|_| format!("SXCRT not found: {runtime_dir}"))?;

    let ext = dylib_ext();
    let libsimplex_name = format!("libsimplex.{ext}");
    let libsimplex_path = runtime_path.join(&libsimplex_name);

    if !libsimplex_path.exists() {
        return Err(format!(
            "Cannot find {libsimplex_name} at {}",
            runtime_path.display()
        )
        .into());
    }

    println!("cargo:rustc-link-search=native={}", runtime_path.display());
    // rpath for local tests
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", runtime_path.display());

    // Link all shared libraries found in the runtime dir explicitly because -rpath-link doesn't
    // work for some reason. --start-group resolves circular symbol refs between GHC's libraries
    // on GNU ld; omitted on Apple platforms where ld64 handles this automatically.
    if uses_link_groups() {
        println!("cargo:rustc-link-arg=-Wl,--start-group");
    }
    for entry in std::fs::read_dir(runtime_path).expect("failed to read runtime dir") {
        let entry = entry.unwrap();
        let path = entry.path();

        if let Some(file_ext) = path.extension()
            && file_ext == ext
        {
            // Extract the link name: libfoo.{ext} -> foo
            if let Some(stem) = path.file_stem() {
                let name = stem.to_str().unwrap();
                if let Some(name) = name.strip_prefix("lib") {
                    println!("cargo:rustc-link-lib=dylib={}", name);
                }
            }
        }
    }
    if uses_link_groups() {
        println!("cargo:rustc-link-arg=-Wl,--end-group");
    }
    Ok(())
}

// Manually links libsimplex.a
// Environment variables:
//   SIMPLEX_STATIC_DIR        path to a simplex-static project dir
//   SIMPLEX_STATIC_BUNDLED=1  Set to do not link GHC libs dynamically
//   GHC_LIBS                  dir containing GHC boot shared libraries
//                             (default: auto-detected via `ghc --print-libdir`)
fn link_with_static_lib(dir: String) -> BuildResult {
    let dir_path =
        std::fs::canonicalize(&dir).map_err(|_| format!("SIMPLEX_STATIC_DIR not found: {dir}"))?;

    let libsimplex = dir_path.join("libsimplex.a");

    if !libsimplex.exists() {
        return Err(format!(
            "libsimplex.a not found at {}. Run `make build` or `make autobuild`
                there first",
            dir_path.display()
        )
        .into());
    }

    // Link libsimplex.a statically
    println!("cargo:rustc-link-search=native={}", dir_path.display());
    println!("cargo:rustc-link-lib=static=simplex");

    link_sys_deps();

    if std::env::var("SIMPLEX_STATIC_BUNDLED").is_ok_and(|v| v == "1") {
        // GHC boot libs are statically bundled no dynamic GHC runtime needed.
        return Ok(());
    }

    // Link required GHC runtime shared libraries
    let ghc_libs = ghc_libs_dir()?;
    println!("cargo:rustc-link-search=native={}", ghc_libs.display());
    // set -rpath for local tests
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", ghc_libs.display());

    if uses_link_groups() {
        println!("cargo:rustc-link-arg=-Wl,--start-group");
    }
    for entry in std::fs::read_dir(&ghc_libs)? {
        let path = entry?.path();
        if let Some(filename) = path.file_name().and_then(|n| n.to_str())
            && let Some(lib_name) = needed_ghc_lib(filename)
        {
            println!("cargo:rustc-link-lib=dylib={lib_name}");
        }
    }
    if uses_link_groups() {
        println!("cargo:rustc-link-arg=-Wl,--end-group");
    }

    Ok(())
}

// Autobuild path: uses the embedded simplex-static directory to produce a fully static
// libsimplex.a with GHC boot libs bundled in. No GHC installation is needed at runtime.
// Callers must pass RUSTFLAGS="-C link-arg=-Wl,-no-pie" on Linux because GHC boot archives lack -fPIC.
fn link_with_autobuild() -> BuildResult {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
    let src_dir = std::path::PathBuf::from(&manifest_dir).join("simplex-static");
    let src_dir = std::fs::canonicalize(&src_dir)
        .map_err(|_| format!("simplex-static not found at {}", src_dir.display()))?;

    let build_dir = out_dir.join("simplex-static");
    let out_lib = build_dir.join("libsimplex.a");

    if !out_lib.exists() {
        for entry in std::fs::read_dir(&src_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                std::fs::copy(entry.path(), build_dir.join(entry.file_name()))?;
            }
        }

        let status = std::process::Command::new("make")
            .current_dir(&build_dir)
            .arg("autobuild")
            .status()?;

        if !status.success() {
            return Err(format!("make autobuild failed in {}", src_dir.display()).into());
        }
        if !out_lib.exists() {
            return Err("libsimplex.a not found after make autobuild".into());
        }
    }

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=simplex");

    link_sys_deps();
    Ok(())
}

fn link_sys_deps() {
    for lib in &["crypto", "ssl", "z", "ffi", "gmp", "pthread", "m"] {
        println!("cargo:rustc-link-lib={lib}");
    }
}
// Returns the directory containing GHC boot shared libraries.
// Uses GHC_LIBS env var if set, otherwise runs `ghc --print-libdir` and
// finds the platform-specific subdir (e.g. x86_64-linux-ghc-9.6.7/, aarch64-apple-darwin-ghc-9.6.7/).
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

    let libdir = str::from_utf8(&output.stdout)?.trim();

    // Find the platform subdir: <arch>-<os>-ghc-<version>/
    std::fs::read_dir(libdir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| {
            p.is_dir()
                && p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.contains("-ghc-"))
        })
        .ok_or_else(|| {
            "GHC platform dir (<arch>-<os>-ghc-<version>) not found under ghc --print-libdir".into()
        })
}

// Returns the link name (strip "lib" prefix and dylib suffix) for GHC boot shared libraries
// that are actually needed at runtime, or None to skip the file.
//
// The required set was determined by computing truly external symbols in libsimplex.a
// (undefined and not defined anywhere in the archive) and matching them against each library.
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
        "HSghc-bignum-",
        "HSghc-boot-th-",
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

    let suffix = format!(".{}", dylib_ext());
    let stem = filename
        .strip_prefix("lib")?
        .strip_suffix(suffix.as_str())?;

    // RTS: only the threaded non-debug variant
    if stem.starts_with("HSrts-") {
        return (stem.contains("_thr") && !stem.contains("debug")).then_some(stem);
    }

    // All other boot packages needed by libsimplex.a (zero-match packages omitted)
    NEEDED.iter().any(|p| stem.starts_with(p)).then_some(stem)
}
