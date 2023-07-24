use std::io::ErrorKind;
use std::path::Path;

use miette::{IntoDiagnostic as _, WrapErr as _};

/// Reads a file to a string.
///
/// Equivalent to [`std::fs::read_to_string`] with better error messages.
pub fn read(path: impl AsRef<Path>) -> crate::Result<String> {
    let path = path.as_ref();
    std::fs::read_to_string(path)
        .into_diagnostic()
        .with_context(|| format!("reading `{}`", path.display()))
}

/// Reads a file to a string.
///
/// Equivalent to [`std::fs::read_to_string`] with better error messages.
#[allow(clippy::empty_loop)]
pub fn maybe_read(path: impl AsRef<Path>) -> crate::Result<Option<String>> {
    let path = path.as_ref();

    match std::fs::read_to_string(path) {
        Ok(s) => Ok(Some(s)),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
        err @ Err(_) => err
            .map(|_| loop {})
            .into_diagnostic()
            .with_context(|| format!("reading `{}`", path.display())),
    }
}
