fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-env-changed=SXCRT");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/bindings.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");

    let runtime_dir = std::env::var("SXCRT").unwrap_or_else(|_| "sxcrt".to_owned());

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
