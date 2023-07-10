//! The missing batteries of WCA.

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod macros;
/// This module provides functionality for parsing and rendering Markdown.
pub mod markdown;

/// A type alias for `miette::Result<T, E>`.
pub type Result<T = (), E = miette::Report> = miette::Result<T, E>;
