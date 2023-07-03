//! The missing batteries of WCA.

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod ir;
mod macros;
/// This module provides functionality for parsing and rendering Markdown.
pub mod markdown;
mod traits;

pub use ir::{CommandBuilder, Property};
pub use traits::{CommandExt, IntoBuilder};

/// A type alias for `miette::Result<T, E>`.
pub type Result<T = (), E = miette::Report> = miette::Result<T, E>;

/// Creates a command-line interface (CLI) builder with the given initial state.
///
/// This function initializes a `CommandBuilder` with the provided `state` and
/// returns it for further configuration of the CLI.
pub fn cli<T>(state: T) -> CommandBuilder<T, 0> {
    CommandBuilder::with_state(state)
}
