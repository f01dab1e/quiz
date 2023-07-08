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
