//! The `simploxide-bindgen` library crate contains everythinig that's needed to parse SimpleX bot
//! API docs and generate a Rust code from them.
//!
//! The actual complete code generators are not included here though because there are some
//! decisions to be made about the resulting file hierarchy and the type shapes of certain types.
//! The `simploxide-bindgen` binary crate generates only one specific layout for the `simploxide`
//! project needs.
//!
//! The library is also generic enough to allow implementing code generation for other languages,
//! the implementor just needs to define the rendering functions for `simploxide-bindgen::types`
//! and implement syntax binders that generate command syntax interpreters for types in the target
//! language.
//!

/// A COMMANDS.MD parser and utilities
pub mod commands;
/// An EVENTS.MD parser and utilities
pub mod events;
/// A TYPES.MD parser as well as data types representing structs, enums and discriminated unions.
pub mod types;

/// A syntax parser and a generator of syntax interpreters
pub mod syntax;

/// Common parsing utilities
mod parse_utils;
