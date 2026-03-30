# simploxide-sxcrt-sys

Low-level Rust FFI bindings for [SimpleX Chat](https://github.com/simplex-chat/simplex-chat).

**Consider the WebSocket interface first.** `simploxide` implements WebSocket
API you can use with SimpleX-Chat CLI application(`simplex-chat -p <port>`)
that is far easier to work with. FFI avoids the process boundary but requires
managing shared libraries, rpaths, and a Haskell runtime. Only switch to FFI
when the overhead is genuinely unacceptable or you require FFI specific
features.


This crate can be used in 2 ways:

- `SXCRT` way: link to dynamic libraries distributed by SimpleX team.
- manual build: build `libsimplex.a` statically and link to it


## SXCRT way (recommended)

Download the pre-built library bundle from the [simplex-chat-libs
releases](https://github.com/simplex-chat/simplex-chat-libs/releases) and
extract it into a `sxcrt/` directory at the root of your crate:

```
your-crate/
  sxcrt/
    .gitignore   <- tracks the dir, ignores its contents
    libsimplex.so
    ... (all bundled .so files)
  src/
  Cargo.toml
```

Each developer extracts the bundle themselves; the dir is tracked in git but
its contents are not:

```gitignore
# sxcrt/.gitignore
*
!.gitignore
```

Create `.cargo/config.toml`

```toml
[env]
SXCRT = { value = "sxcrt", relative = true }

[build]
rustflags = ["-C", "link-arg=-Wl,-rpath,sxcrt"]
```

`-rpath,sxcrt` is a CWD-relative path. the loader finds the libs as long as you
run your binary from the project root, which is the normal case during
development. With `SXCRT` pointing at the same directory, the build just works
after cloning the project and downloading the libraries into `sxcrt`. For
workspaces it's recommended to keep `sxcrt/` and the `.cargo/config.toml` at
the workspace root and create relative symlinks inside crates that depend on
`sxcrt`.

```bash
# In workspace crate directory
ln -sr ../sxcrt sxcrt
```

This way workspce binaries will find `sxcrt` whether you run them from the
workspace root or from crate directories.

### Distributing

For release binaries, `$ORIGIN`-relative rpath is needed so the loader finds
the `.so` files next to the binary regardless of CWD. Because `RUSTFLAGS`
overrides `rustflags` from `config.toml`, a build script can swap the rpath
cleanly:

```sh
#!/usr/bin/env bash
# dist-build.sh
RUSTFLAGS="-C link-arg=-Wl,-rpath,\$ORIGIN/sxcrt" cargo build --release
```

Ship the resulting binary alongside a `sxcrt/` directory containing
the `.so` bundle.

---

## Manual build (`build-sxcrt` feature)

The simplex chat `.so` distribution has SQLCipher statically linked inside it.
SQLCipher manages global state, so having two instances in the same process -
one from the bundle and one from your project - will cause segfaults or
heisenbugs. The same problem can occur with other libraries that overlap
between your project and the `.so` bundle.

`build-sxcrt` feature manually builds `libsimplex.a` in a way that links
`libsqlcipher.so` dynamically instead. This allows your project and
`libsimplex.a` to share a single system `libsqlcipher.so` instance but only if
your project also links it dynamically. If any code in the process links
SQLCipher statically, two instances will still end up in memory and the
conflict remains.

```toml
[dependencies]
simploxide-sxcrt-sys = { version = "*", features = ["build-sxcrt"] }
```

Building simplex-chat from source takes 10+ minutes. To avoid rebuilding on
every `cargo clean`, copy the `simplex-static/` directory from this crate
somewhere permanent on your system and point `SIMPLEX_STATIC_DIR` at it:

```sh
cp -r /path/to/simploxide-sxcrt-sys/simplex-static /opt/simplex-static
export SIMPLEX_STATIC_DIR=/opt/simplex-static
```

The build script reuses the existing `libsimplex.a` in that directory. Delete
it to force a rebuild. Keeping your own copy of `simplex-static/` also lets you
tweak the build flags for your specific needs.

### SQLCipher

By default, `build-sxcrt` links system `libsqlcipher.so` dynamically. If your
project has no SQLCipher dependency and you want to drop the system
`libsqlcipher.so` runtime requirement, you can embed it statically:

```toml
[dependencies]
simploxide-sxcrt-sys = { version = "*", features = ["embed-sqlcipher"] } # implies build-sxcrt
```

**Do not use `embed-sqlcipher` if anything else in your project uses SQLCipher**
— it reintroduces the two-instance conflict described above.

### GHC boot libraries

Static linking does not embed GHC boot `.so` files (they lack `-fPIC`) but they
still must be present at runtime. For dev builds the build script automatically
detects them via `ghc --print-libdir`(set `GHC_LIBS` env variable to override
this path). For distribution you must collect the required GHC boot libraries
into `sxcrt` and set relative `rpath`. To collect the required libs use the
script available in this repo inside the `simplex-static/` dir:

```sh
./simplex-static/collect-ghc-libs.sh /path/to/output/sxcrt
```

Then use the same `dist-build.sh` approach as in `SXCRT` paragraph —
`RUSTFLAGS` overrides the CWD-relative rpath with `$ORIGIN/sxcrt` for the
release binary. Ship the binary with `sxcrt/` next to it.
