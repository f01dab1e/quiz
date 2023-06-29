//! The missing batteries of WCA.

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod ir;
mod macros;
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

/// Finds rust code blocks in the given markdown text.
pub fn find_rust_code_blocks(text: &str) -> Vec<Box<str>> {
    lazy_regex::lazy_regex!(r"(?s)```rust\s*\n(.*?)\n\s*```")
        .captures_iter(text)
        .map(|item| item[1].into())
        .collect()
}

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use itertools::Itertools as _;

    #[test]
    fn find_rust_code_blocks() {
        let markdown = r#"
```rust
fn main() {
    println!("Hello, world!");
}
```

```

```

```zig
const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("Hello, {s}!\n", .{"world"});
}
```
"#;

        let blocks = super::find_rust_code_blocks(markdown)
            .into_iter()
            .enumerate()
            .map(|(index, text)| lazy_format::lazy_format!("{index}: {text}"))
            .join("\n");

        expect![[r#"
            0: fn main() {
                println!("Hello, world!");
            }"#]]
        .assert_eq(&blocks);
    }
}
