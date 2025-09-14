# simploxide-bindgen

`cargo run` this crate to generate all source files for `simploxide-api-types`.
The files are placed into the `./generated` directory and should be moved
manually into the `simploxide-api-types/src` by some external script.

### As a library

It wasn't planned initially, but this crate also comes as a library that allows
to generate bindings for other languages. The default impls are hardcoded for
`simploxide-api-types` but they're easily overridable with a new type pattern.

Check out the docs for further guidance: [`simploxide-bindgen`](#link)

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

Licensed under either of [Apache License, Version 2.0](../LICENSE-APACHE) or [MIT
license](../LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

