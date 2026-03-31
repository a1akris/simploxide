# simploxide-sxcrt-sys

Low-level Rust FFI bindings for [SimpleX Chat](https://github.com/simplex-chat/simplex-chat).

**Consider the WebSocket interface first.** `simploxide` implements WebSocket
API you can use with SimpleX-Chat CLI application(`simplex-chat -p <port>`)
that is far easier to work with. FFI avoids the process boundary but requires
managing shared libraries, rpaths, and a Haskell runtime — only switch to FFI
when the overhead is genuinely unacceptable or you require FFI specific
features.

This crate can be used in 2 ways:

- `SXCRT` way: link to dynamic libraries distributed by SimpleX team.
- `SIMPLEX_STATIC_DIR` way: Manually build and manage SimpleX libs

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
after downloading the repo and putting SimpleX libraries into `sxcrt`.

For workspaces it's recommended to keep `sxcrt/` and the `.cargo/config.toml`
at the workspace root and create relative symlinks inside crates that depend on
`sxcrt`.

```bash
# In workspace crate directory
ln -sr ../sxcrt sxcrt
```

This way workspace binaries will find `sxcrt` whether you run them from the
workspace root or from the crate directories.

### Distributing

For production binaries, `$ORIGIN`-relative rpath is needed so the loader finds
the `.so` files next to the binary regardless of CWD. Because `RUSTFLAGS`
overrides `rustflags` from `config.toml`, a build script can swap the `rpath`
cleanly:

```sh
#!/usr/bin/env bash
# dist-build.sh
RUSTFLAGS="-C link-arg=-Wl,-rpath,\$ORIGIN/sxcrt" cargo build --release
```

Ship the resulting binary alongside the `sxcrt/` directory containing the `.so`
bundle.

---

## SIMPLEX_STATIC_DIR way

> [!NOTE] Prerequisites: install all [dependencies
> required](https://github.com/simplex-chat/simplex-chat/blob/stable/docs/CLI.md#in-any-os)
> to build SimpleX-Chat locally

The SimpleX-Chat `.so` libraries have SQLCipher statically linked inside them.
SQLCipher manages global state, so having two instances in the same process -
one from the bundle and one from your project executable - will cause segfaults
and heisenbugs. The same problem can occur with other libraries that overlap
between your project and the SimpleX-Chat.

`SIMPLEX_STATIC_DIR` builds `libsimplex.a` and `libsqlcipher.a` where
`libsqlcipher.a` is the exact same version of the SQLCipher embedded into
`libsimplex.a`. This ensures `sqlite` state deduplication at link time and by
linking to `libsqlcipher.a` from `SIMPLEX_STATIC_DIR` you can be sure that your
resulting binary uses the SQLCipher version the SimpleX-Chat depends on. Beware
that if any code in the process links SQLCipher dynamically, two instances may
still end up in memory and the conflict remains.

Copy the `simplex-static/` directory from this crate somewhere permanent on
your system and build it there:

```sh
cp -r /path/to/simploxide-sxcrt-sys/simplex-static /opt/simplex-static
cd /opt/simplex-static

# Builds libsimplex.a and libsqlcipher.a with options used by SimpleX-Chat
make
```

Then export the env vars for your Rust project:

```sh
export SIMPLEX_STATIC_DIR=/opt/simplex-static
export SQLCIPHER_LIB_DIR=/opt/simplex-static
export SQLCIPHER_INCLUDE_DIR=/opt/simplex-static
export SQLCIPHER_STATIC=1
```

`SIMPLEX_STATIC_DIR` is read by this crate's build script. `SQLCIPHER_LIB_DIR`,
`SQLCIPHER_INCLUDE_DIR` and `SQLCIPHER_STATIC` are read by rusqlite (with
`features = ["sqlcipher, "buildtime_bindgen"]`) so it links the same
`libsqlcipher.a`. This way the resulting application  share the same SQLCipher
instance.

### GHC boot libraries

Static linking does not embed GHC boot libraries (they lack `-fPIC`) so you
still must link to GHC runtime dynamically.

You can collect all required dynamic dependencies with `collect-ghc-libs.sh`
script from `simplex-static`.

```sh
/opt/simplex-static/collect-ghc-libs.sh /path/to/your/project/sxcrt
```

Then use the same approach described in `SXCRT` to setup dev and distribution
builds.
