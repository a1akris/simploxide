![Build](https://github.com/a1akris/simploxide/actions/workflows/rust.yml/badge.svg)

# SimplOxide

A fast and robust SimpleX-Chat bot SDK that doesn't exist yet. Meanwhile there
are:

- [simploxide-core](./simploxide-core): A fully asynchronous raw SimpleX Chat
  client with strong graceful shutdown guarantees.

- [simploxide-api-types](./simploxide-api-types): All API types from SimpleX
  bot API docs with serialization, command syntax interpretation, and even some
  syntax sugar support.

- [simploxide-client](./simploxide-client): A crate that merges
  `simploxide-core` with `simploxide-api-types` providing a higher level client
  API. Supports all requests described [in SimpleX docs](https://github.com/simplex-chat/simplex-chat/tree/stable/bots). The API is
  still cumbersome to work with, mainly because of the deeply nested API types,
  but it's possible to implement everything with it. Right now **this is the
  main(root) crate you should use**, it re-exports everything else.

- [simploxide](./simploxide): An empty crate reserved for the future bot SDK
  that will provide a lot of utilities on top of the `simploxide-client` to
  cover common bot development tasks.

- [simploxide-bindgen](./simploxide-bindgen): A crate that produces
  `simploxide-api-types` files and keeps them up to date by parsing the
  upstream documentation and generating the code from it.


### Where to look next?

Start with [simploxide-client](https://docs.rs/simploxide-client) crate docs.
If you need to know how async querying is implemented under the hood check the
[simploxide-core/README.md](./simploxide-core)


### Version compatability table

| SimplOxide Version | Simplex Chat Min Version | Simplex Chat Max Version |
| ------------------ | ------------------------ | ------------------------ |
| 0.2.0              | 6.4.4.0                  | 6.4.4.2                  |


### LICENSE

#### DISCLAIMER

SIMPLEX CLI HAS AN OPEN WEBSOCKET API, AND IT CAN BE ACCESSED FROM ANY LIBRARY
OR APPLICATION, WHETHER OPEN-SOURCE OR NOT. THE SIMPLOXIDE LIBRARIES PROVIDE A
WEBSOCKET CLIENT AND API CODECS FOR SIMPLEX CLI THAT DON'T DEPEND ON ANY CODE
FROM THE SIMPLEX PROJECT AND THEREFORE ARE DUAL LICENSED UNDER APACHE-2.0/MIT
AS STATED BELOW. APACHE-2.0/MIT TERMS AND CONDITIONS ARE APPLICABLE TO YOUR
PROJECTS AS LONG AS THEY DON'T:

- SHIP SIMPLEX-CLI OR ANY OTHER SIMPLEX COMPONENTS AS PART OF AN APPLICATION
- USE FFI BINDINGS TO THE SIMPLEX-CORE OR OTHER SIMPLEX LIBRARIES
- DEPEND ON OR INCLUDE ANYTHING ELSE LICENSED UNDER AGPL

OTHERWISE, YOUR PROJECTS MUST ADHERE TO [SIMPLEX
AGPL-3.0](https://github.com/simplex-chat/simplex-chat/blob/stable/LICENSE).


THIS IS NOT A LEGAL ADVICE BUT RATHER A FRIENDLY REMAINDER.

SIMPLOXIDE LIBRARIES AUTHORS DISCLAIM ALL RESPONSIBILITY AND LIABILITY FOR ANY
FAILURE BY SIMPLOXIDE USERS TO COMPLY WITH THE AGPL-3.0.

---

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT
license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

