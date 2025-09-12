# SimplOxide

A fast and robust SimpleX Chat bots SDK that doesn't exist yet. Meanwhile there are:

- [simploxide-core](./simploxide-core): A fully asynchronous raw SimpleX Chat client with strong
  graceful shutdown guarantees. See its docs for details.

- [simploxide-api-types](./simploxide-api-types): All API types from SimpleX bot API docs with
  serialization, command syntax interpretation, and even some syntax sugar
  support.

- [simploxide-bindgen](./simploxide-bindgen): A crate that produces
  `simploxide-api-types` files and keeps them up to date by parsing the
  upstream documentation and generating the code from it.

- [simploxide-client](./simploxide-client): A crate that merges
  `simploxide-core` with `simploxide-api-types` providing a higher level client
  API. Supports all requests described [in SimpleX docs](#link). The API is
  still cumbersome to work with, mainly because of the deeply nested API types,
  but it's possible to implement everything with it. Right now **this is the
  main(root) crate you should use**, it re-exports everything else.

- [simploxide](./simploxide): An empty crate reserved for the future bot SDK
  that will provide a lot of utilities on top of the `simploxide-client` to
  cover common bot development tasks.

### Where to look next?

Start with [simploxide-client](#ReferenceHere) crate docs. If you need to
know how async querying is implemented under the hood check the
[simploxide-core/README.md](./simploxide-core)


### Version compatability table

| SimplOxide Version | Simplex Chat Min Version | Simplex Chat Max Version |
| ------------------ | ------------------------ | ------------------------ |
| 0.1.0              | 6.4.4.0                  | 6.4.4.2                  |


##### LICENSE

_TODO_
