//! The `simploxide-bindgen` library crate contains everythinig that's needed to parse SimpleX bot
//! API docs and generate Rust code from them.
//!
//! The functions that generate full Rust modules or a full crate are not included here though.
//! There are some decisions to be made about the resulting file hierarchy and the type shapes of
//! certain types, so full code generation is left to the client.
//!
//! The `simploxide-bindgen` binary crate generates only one specific crate layout for the
//! `simploxide` project needs.
//!
//! The library is generic enough to allow implementing bindings for other languages. The
//! implementor just needs to override `std::fmt::Display` for [`crate::types`] using a new type
//! pattern and implement a [`syntax::binding::SyntaxInterpreter`] to generate a command syntax
//! interpreter in the target language.

/// A COMMANDS.MD parser and utilities.
pub mod commands;
/// An EVENTS.MD parser and utilities.
pub mod events;
/// A TYPES.MD parser and utilities.
pub mod types;

/// A syntax parser and a generator of syntax interpreters.
pub mod syntax;

/// Common parsing utilities.
mod parse_utils;
