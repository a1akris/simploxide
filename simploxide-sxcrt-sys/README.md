# simploxide-sxcrt-sys

Low-level Rust FFI bindings for [SimpleX Chat](https://github.com/simplex-chat/simplex-chat).

**Consider the WebSocket interface first.** `simploxide` implements a WebSocket
API you can use with the SimpleX-Chat CLI application (`simplex-chat -p
<port>`) that is far easier to work with. FFI avoids the process boundary but
requires managing shared libraries, link args, and Haskell runtime. Only switch
to FFI before releasing to production if extra security and perfomance are
important for your project.

This crate can be used in 3 ways:

1. `SXCRT`(recommended) : link to dynamic libraries distributed by SimpleX team.
1. `SIMPLEX_STATIC_AUTOBUILD`: `build.rs` automatically builds and links all Haskell deps statically(~20-30 minutes)
1. `SIMPLEX_STATIC_DIR`(**mandatory** when project needs to link with `libsqlite` or `libsqlcipher`): build and manage SimpleX libs manually

---

## SXCRT

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

Create `.cargo/config.toml`:

```toml
[env]
SXCRT = { value = "sxcrt", relative = true }

[build]
# Linux
rustflags = ["-C", "link-arg=-Wl,-rpath,sxcrt"]
```

`-rpath,sxcrt` is required for the Linux loader to find the libraries. This
configuration works as long as you run your binary from the project root, which
is the normal case during development.

For workspaces, keep `sxcrt/` and `.cargo/config.toml` at the workspace root
and create relative symlinks inside crates that depend on `sxcrt`:

```bash
# In workspace crate directory
ln -sr ../sxcrt sxcrt
```

This way workspace binaries will find `sxcrt` whether you run them from the
workspace root or from the crate directories.

#### Distributing

For production binaries on Linux, an `$ORIGIN`-relative rpath is needed so the
loader finds the `.so` files next to the binary regardless of the workding
directory. Because `RUSTFLAGS` env variable overrides `rustflags` from
`config.toml`, a build script can swap the rpath cleanly:

```sh
#!/usr/bin/env bash
# dist-build.sh
RUSTFLAGS="-C link-arg=-Wl,-rpath,\$ORIGIN/sxcrt" cargo build --release
```

Ship the resulting binary alongside the `sxcrt/` directory containing the `.so`
bundle.

---

## SIMPLEX_STATIC_AUTOBUILD

Use this when you want a single self-contained binary with no Haskell shared
library dependencies. The only downside of this approach is it requires your
executable to be `-no-pie`. The build script automatically pulls and builds
`libsimplex.a` with GHC boot libraries bundled in(takes ~20-30 minutes on the
first build), and links everything statically. The result is cached and
subsequent cargo builds are instant until cargo clean.

#### Prerequisites

- **Git**
- **Curl**
- **pkg-config**
- **gcc**
- **GHC toolchain** via [GHCup](https://www.haskell.org/ghcup/):
- **libclang**
- **libffi**
- **libgmp**
- **libssl**
- **zlib**

```bash
ghcup install ghc 9.6.7
ghcup install cabal 3.10
ghcup set ghc 9.6.7
ghcup set cabal 3.10
```

#### Building

GHC boot archives lack `-fPIC` and are incompatible with position-independent
executables which Rust tries to build by default. Passing
`link-arg=-Wl,-no-pie` via `RUSTFLAGS` may be required to build no-pie binary
compatible with GHC libs successfully.

Create `.cargo/config.toml`:

```toml
[env]
SIMPLEX_STATIC_AUTOBUILD = { value = "1" }

[build]
# Linux
rustflags = ["-C", "link-arg=-Wl,-no-pie"]
```

With this config the build must succeed on Linux

### Distributing

This configuration creates a standalone binary that should be compatible with
most Linux distros on the target arch.

---

## SIMPLEX_STATIC_DIR: **MUST USE** when your project also links **SQLite** or **SQLCipher**

Use this path only if your project links `libsqlcipher` or `libsqlite` directly.
The SimpleX-Chat shared library bundles SQLCipher statically. If two separate
copies of SQLCipher end up in the same process(one from the bundle and one
from your project) they manage the same global state independently, causing
segfaults and heisenbugs that are very hard to diagnose.

`SIMPLEX_STATIC_DIR` allows to build `libsimplex.a` and `libsqlcipher.a` together, where
`libsqlcipher.a` is the exact SQLCipher version embedded in `libsimplex.a`. By
linking your project against this `libsqlcipher.a`, both codepaths share one
SQLCipher instance at link time. Note that if any other code in the process
loads SQLCipher dynamically, the conflict can still occur.

#### Prerequisites

_Same deps as for the SIMPLEX_STATIC_AUTOBUILD_

- `simplex-static` directory must be copied from this repo and stored somewhere
  on the system. The recommended approach is to git clone `simploxide` and
  create a symlink to simplex-static somewhere(e.g. `ln -s
  /path/to/simploxide/simploxide-sxcrt-sys/simplex-static
  /opt/simplex-static`). This way you can easilly update the simplex-static
  libraries later by git pulling changes and rerunning make.

#### Build

Copy the `simplex-static/` directory from this repository somewhere permanent on
your system and build it there:

```sh
cp -r /path/to/simploxide-sxcrt-sys/simplex-static /opt/simplex-static
cd /opt/simplex-static
make build # requires linking GHC libs dynamically
# or
make autobuild # links everything statically but requires -no-pie
```

Then export the env vars for your Rust project:

```sh
export SIMPLEX_STATIC_BUNDLED=1 # only if you run `make autobuild`
export SIMPLEX_STATIC_DIR=/opt/simplex-static
export SQLCIPHER_LIB_DIR=/opt/simplex-static
export SQLCIPHER_INCLUDE_DIR=/opt/simplex-static
export SQLCIPHER_STATIC=1
```

`SIMPLEX_STATIC_DIR` and `SIMPLEX_STATIC_BUNDLED` are read by this crate's
build script. `SQLCIPHER_LIB_DIR`, `SQLCIPHER_INCLUDE_DIR`, and
`SQLCIPHER_STATIC` are read by rusqlite (with `features = ["sqlcipher",
"buildtime_bindgen"]`) so it links the same `libsqlcipher.a`.

#### GHC boot libraries

GHC boot libraries can be linked statically or dynamically based on how you run
the make script.

- If you use `make build` the `build.rs` automatically links GHC dynamic
  libraries from the ghc install dir for dev builds(this can be overriden with
  `GHC_LIBS` env variable that allows to manually set up the path to GHC
  runtime libs). For distribution use `collect-ghc-libs.sh` helper script
  available at `simplex-static` dir to collect all required libraries into a
  directory and set `-rpath` to this directory for production builds:

  ```bash
  /opt/simplex-static/collect-ghc-libs.sh /path/to/your/project/sxcrt
  RUSTFLAGS="-C link-arg=-Wl,-rpath,\$ORIGIN/sxcrt" cargo build --release
  ```
  Ship the resulting binary alongside the `sxcrt/` directory containing the `.so`
  bundle.

- If you use `make autobuild` then GHC libs will be linked statically. Set
  `SIMPLEX_STATIC_BUNDLED=1` env variable so that the build script won't try to
  search and link GHC dynamic libraries and configure the project as described
  in `SIMPLEX_STATIC_AUTOBUILD`:

  ```toml
  [build]
  # Linux
  rustflags = ["-C", "link-arg=-Wl,-no-pie"]
  ```

  The config is universal for dev and production builds but `-no-pie` binary
  implies weaker binary security(no ASLR).

---

### LICENSE

**Licensed under [AGPL-3.0](../LICENSE-AGPL3)**
