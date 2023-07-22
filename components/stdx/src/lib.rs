//! The missing batteries of WCA.

mod macros;
/// This module provides functionality for parsing and rendering Markdown.
pub mod markdown;
/// Various utilities for working with files and paths.
pub mod paths;

/// A type alias for `miette::Result<T, E>`.
pub type Result<T = (), E = miette::Report> = miette::Result<T, E>;
