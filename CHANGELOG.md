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
