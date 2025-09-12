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

_TBD_
