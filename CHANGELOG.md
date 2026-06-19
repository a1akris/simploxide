# v0.13.0 - Remote control support

- New `remote` module for accepting incoming SimpleX Desktop remote control
  sessions. See the `remote_control.rs` example for a demonstration

- Add extra `farm` bots and bot-farms type aliases simplifying naming `farm`
  types in the function signatures.

- Update types to support SimpleX-Chat `v6.5.5`

- Fixed a bug where creating a bot in a `BotFarm` would fail permanently if
  SimpleX rejected the requested display name as invalid. The farm now
  automatically retries with the corrected name.

- Renamed `send()` helper on `MessageBuilders` to `deliver()` to eliminate
  thautologies.

- Fixed a bug where failure to create a bot in a running farm would not cleanup
  the user slot properly preventing the bot from being recreated or accessed
  later.

- Fixed incorrect parallel execution of requests within a `BotFarm` that could
  cause commands to arrive out of order at the SimpleX backend.

[Full diff](https://github.com/a1akris/simploxide/compare/v0.12.0...v0.13.0)

# v0.12.0 - Bot farms to manage multibot instances

- New `BotFarm` type and `BotFarmBuilders` (available in both `ws` and `ffi` modules) for running
  multiple independent SimpleX identities on a single SimpleX-Chat instance. Commands are automatically
  multiplexed so each user stays active for the duration of its own requests; incoming events are
  demultiplexed and delivered to the corresponding user `EventStream`.

  The farm supports two user kinds: **bots** have dedicated `EventStreams` and are suited for
  interactive scenarios handling events independently; **ghosts** share the farm's stream and are
  suited for background or fire-and-forget tasks.

- A new `support_bots` example demonstrating how to set up and use a bot farm.

- `Bot::default_relays()` to return default relay IDs configured for the active user.

- `create_group` and `create_public_group` now automatically retry with the
  corrected display name when SimpleX rejects the requested one as invalid, instead of returning
  an error.

- `preferences` module adds `YES`, `NO` and other constants for different group preferences.

- `GroupLinkExt` trait with a `.link()` shorthand for extracting the group link
  from data structures containing them

- `Transcoder::jpeg()` and `Transcoder::thumbnail()` named constructors cover the two most common
  preview configurations without having to specify every field manually.

- Types updated to cover SimpleX-Chat `v6.5.4`.

- Fixed a bug where a bot's `EventStream` could receive events belonging to
  other users still present in the database, potentially causing spurious
  `StoreError`s. Streams are now filtered by owner user ID when constructed via
  `BotBuilder` or `BotFarmBuilder`. Manually-constructed streams can enable
  this behaviour with `EventStream::set_owner`/`EventStream::exclude_user`.

- Fixed error events being incorrectly handled as critical errors in certain
  scenarios.

- Fixed incorrect `delete_chat` behaviour.

[Full diff](https://github.com/a1akris/simploxide/compare/v0.11.0...v0.12.0)

# v0.11.0 - Fully static builds for FFI

- `simploxide-sxcrt-sys` now comes with an option to enable automatic fully
  static builds for downstream apps. The documentation was improved to describe
  the tradeoffs of each build mode in details

- Fix invalid constraints and holes in high-level APIs

- Update types to support `v6.5.3` and add typesafe wrappers for new methods

[Full diff](https://github.com/a1akris/simploxide/compare/v0.10.0...v0.11.0)

# v0.10.0 - High level API, FFI support and more

- High-level `Bot` type providing the typesafe idiomatic Rust API to work with SimpleX-Chat.

- High-level event dispatchers: The new `EventStream` carries per-kind event
  filters and avoids parsing events not processed by users. For convenience it
  can be converted into a dispatcher type that exposes an interface registering
  async event callbacks and efficiently firing them under the hood.

- High-level message sending APIs: `MessageBuilder` and `MulticastBuilder`(for
  sending to multiple chats in parallel) supporting plenty of builder methods
  for flexible message configuration before send.

- High-level multimedia message types: `Image` `Video` and `Link` with
  auto/semi-auto previews resolution.

- High-level `Report`, `Chat`, and `Custom` message types for content
  moderation reports, sharing SimpleX contact or group links in-chat, and
  arbitrary structured payloads respectively.

- High-level `ImagePreview` type for lazily reading and transcoding thumbnails

- The `multimedia` feature adds a `Transcoder` that decodes any image format
  supported by the `image` crate and re-encodes it as a size-checked JPEG
  thumbnail. Output dimensions, quality, and blur effect are each configurable.

- When the `multimedia` feature is enabled, `Image` messages automatically
  generate and attach a preview from their source file, falling back to a
  placeholder on error. With `native_crypto` also enabled, previews can be
  transcoded even from encrypted source files in memory.

- High-level `SimplexCli` wrapper which spawns and manages the `simplex-chat`
  process directly from bot code, replacing the requirement to start it
  out-of-band. The CLI is placed in its own process group on Unix so that
  Ctrl-C reaching the bot does not simultaneously kill it and disrupt the
  graceful shutdown sequence. Launch also validates that the process started
  successfully: if the configured port is already in use an error is returned
  rather than silently attaching to an unrelated process.

- New `simploxide-sxcrt-sys` crate with raw Rust bindings to `libsimplex`.
  Both dynamic (`SXCRT`) and static (`SIMPLEX_STATIC_DIR`) linking modes are
  supported; See crate README for details

- New `simploxide-ffi-core` crate wrapping `simploxide-sxcrt-sys` in a fully
  async client that mirrors the `simploxide-ws-core` interface. Multiple FFI
  chat instances share a single OS thread with configurable round-robin
  scheduling and per-instance execution caps to prevent starvation.

- The new `ffi` feature of `simploxide-client` exposes `ffi::Client` and
  `ffi::init()` as a drop-in replacement for the WebSocket backend. A `fullffi`
  meta-feature bundles all crate features for convenience.

- The `cancellation` feature adds dispatcher variants that accept a
  `CancellationToken` for externally stopping a running dispatcher loop.

- High-level `XftpClient<C>` wrapper with `download_file` helper streamlining
  the file downloads by processing XFTP events under the hood.

- New `crypto` module with with native Rust implementation of SimpleX
  client-side file encryption(secretbox) as well as primitives allowing to use
  any other SimpleX client-side encryption impls with simploxide

- New `crypto::fs` module with blocking (`crypto::fs::std`) and async
  (`crypto::fs::tokio`) implementations for correctly reading and writing
  SimpleX encrypted files.

- New `multimedia_bot` example demonstrating an image processing flow

- `simploxide-core` is deprecated and split into `simploxide-ws-core`
  (WebSocket) and `simploxide-ffi-core` (FFI). Existing direct users of
  `simploxide-core` should switch their dependency to `simploxide-ws-core`.

- Core event and response routing replaced with zero-cost JSON parsing. Events
  and responses are now routed as a single allocation and deserialized only once
  at the point of consumption, eliminating the nested allocations that occurred
  on every received message in previous versions.

- Both the WebSocket and FFI clients now validate the SimpleX backend version
  on launch and return an error immediately if it is outside the supported
  range.

- `disconnect()` on the WebSocket client is now `async` and can be awaited to
  confirm the connection has fully closed.

- API types updated to cover SimpleX-Chat `6.5.2`.

- `simploxide_client::connect()` and `retry_connect()` moved to
  `simploxide_client::ws::connect()` and `simploxide_client::ws::retry_connect()`.

[Full diff](https://github.com/a1akris/simploxide/compare/v0.9.0...v0.10.0)

# v0.9.0 - Better data shapes and optimizations

- `Arc<Event>` is turned into `Event(Arc<EventData>)` allowing to move events
  around and implementing `Send + 'static` event dispatchers

- Command syntax generators are now better optimized and generate command
  strings using only single memory allocation.

- Most discriminated unions now have constructor methods which makes code that
  builds complex structs significantly shorter

- The type of undocumented fields - `BTreeMap<String, serde_json::Value>` is reduced to simple `serde_json::Value`.

- All `HashMaps` are replaced with `BTreeMaps` to consume less memory and to
  support more uses in `const` contexts.

- Reduce dependencies by disabling default features in certain crates

[Full diff](https://github.com/a1akris/simploxide/compare/v0.8.0...v0.9.0)

# v0.8.0 - Further API simplifications and improvements

- Now if some response contains only a single documented field instead of
  returning the response struct the struct field is returned directly reducing
  data nesting

- Added `simploxide_client::retry_connect` method for scenarios when
  simplex-cli is run programmatically and it's impossible to tell when web
  socket port becomes available

[Full diff](https://github.com/a1akris/simploxide/compare/v0.7.0...v0.8.0)

# v0.7.0 - Dependencies upgrade

- Updated `tokio_tungstenite` and `tungstenite` dependencies to `v0.28`

[Full diff](https://github.com/a1akris/simploxide/compare/v0.6.0...v0.7.0)

# v0.6.0 - Major changes in code generation and generated types

- Now newly generated `ClientApi` methods segregate good responses from bad ones.
  `Undocumented` responses are considered to be bad by default and result in
  `ClientApiError` but `Result<T, impl ClientApiError>` implements the
  `AllowUndocumentedResponses` trait extension which allows to override this
  behavior by moving the `Undocumented` variant from `Err` to `Ok` side of the
  `Result`

- Code generation of the `ClientApi` now simplifies responses by turning
  response enums into structs where possible and by implementing helper
  response getters for enums where simplification is impossible.

- Introduced a new `ClientApiError` trait which must be implemented by
  `ClientApi::Error` types. This trait is mainly needed to be able handle
  undocumented responses in different ways.

- Extended prelude includes newly genetated `client_api` types and traits.

[Full diff](https://github.com/a1akris/simploxide/compare/v0.5.0...v0.6.0)

# v0.5.0 - New type for undocumented fields

- The type of undocumented fields was changed from `HashMap<Key, Value>` to
  `BTreeMap<Key, Value>` because `BTreeMap`s can be default-constructed in
  `const` contexts enabling const definitions of requests and request parts.

[Full diff](https://github.com/a1akris/simploxide/compare/v0.4.0...v0.5.0)


# v0.4.0 - Critical API fixes and automaintenance mode

- Fixes for APIs that were barely usable due to missing trait implementations
  or unergonomic return types.

- A new script that maintains this repository by automatically generating and
  submitting new type definitions whenever SimpleX-Chat changes its API
  specifications

[Full diff](https://github.com/a1akris/simploxide/compare/v0.3.0...v0.4.0)

# v0.3.0 - Upgrade to the new SimpleX API

- Regenerate types to support API definitions for SimpleX-Chat `v6.4.5+`

[Full diff](https://github.com/a1akris/simploxide/compare/v0.2.0...v0.3.0)

# v0.2.0

- Hello, world.
