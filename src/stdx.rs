use crate::ir::Symbol;

pub(crate) fn cli<T>(state: T) -> CommandBuilder<T, 0> {
    CommandBuilder::with_state(state)
}

pub(crate) struct CommandBuilder<T, const N: usize> {
    state: T,
    commands: [wca::Command; N],
    handlers: [(String, wca::Routine); N],
}

#[derive(Clone)]
pub(crate) struct Property<'a> {
    pub(crate) name: &'a str,
    pub(crate) hint: &'a str,
    pub(crate) tag: wca::Type,
}

impl<T> CommandBuilder<T, 0> {
    fn with_state(state: T) -> Self {
        Self { state, handlers: [], commands: [] }
    }
}

pub(crate) trait CommandExt<T>: Sized {
    fn arg(self, hint: &str, tag: wca::Type) -> Builder<Self> {
        Builder::new(self).arg(hint, tag)
    }

    fn properties<const N: usize>(self, properties: [Property; N]) -> Builder<Self> {
        Builder::new(self).properties(properties)
    }
}

pub(crate) struct Builder<F> {
    handler: F,
    command: wca::Command,
}

impl<F> Builder<F> {
    fn new(handler: F) -> Self {
        let name = itertools::join(name::<F>().split('_'), ".");

        Self { handler, command: wca::Command::former().phrase(name).form() }
    }

    pub(crate) fn arg(mut self, hint: &str, tag: wca::Type) -> Self {
        self.command.subjects.push(wca::grammar::settings::ValueDescription {
            hint: hint.into(),
            kind: tag,
            optional: false,
        });
        self
    }

    pub(crate) fn properties<const N: usize>(mut self, properties: [Property; N]) -> Self {
        for property in properties {
            self.command.properties.insert(
                property.name.to_owned(),
                wca::grammar::settings::ValueDescription {
                    hint: property.hint.to_owned(),
                    kind: property.tag,
                    optional: true,
                },
            );
        }
        self
    }
}

impl<F: Fn(T, wca::Args, wca::Props) -> crate::Result, T> CommandExt<T> for F {}

pub(crate) trait IntoBuilder<F, T>: Sized {
    fn into_builder(self) -> Builder<F>;
}

impl<F, T> IntoBuilder<F, T> for Builder<F> {
    fn into_builder(self) -> Self {
        self
    }
}

impl<F: Fn(T, wca::Args, wca::Props) -> crate::Result, T> IntoBuilder<F, T> for F {
    fn into_builder(self) -> Builder<F> {
        Builder::new(self)
    }
}

impl<T: Copy + 'static, const LEN: usize> CommandBuilder<T, LEN> {
    pub(crate) fn command<F: Fn(T, wca::Args, wca::Props) -> crate::Result + 'static>(
        self,
        command: impl IntoBuilder<F, T>,
    ) -> CommandBuilder<T, { LEN + 1 }> {
        let Builder { handler, command } = command.into_builder();

        let handler = wca::Routine::new(move |(args, props)| {
            handler(self.state, args, props)
                .map_err(|report| wca::BasicError::new(format!("{report:?}")))
        });

        CommandBuilder {
            state: self.state,
            handlers: array_push(self.handlers, (command.phrase.clone(), handler)),
            commands: array_push(self.commands, command),
        }
    }

    pub(crate) fn build(self) -> wca::CommandsAggregator {
        wca::CommandsAggregator::former().grammar(self.commands).executor(self.handlers).build()
    }
}

fn array_push<const N: usize, T>(this: [T; N], item: T) -> [T; N + 1] {
    use std::mem::MaybeUninit;

    unsafe {
        let mut uninit = MaybeUninit::<[T; N + 1]>::uninit();

        let ptr = uninit.as_mut_ptr() as *mut T;
        (ptr as *mut [T; N]).write(this);
        (ptr.add(N) as *mut [T; 1]).write([item]);

        uninit.assume_init()
    }
}

#[macro_export]
macro_rules! parse_args {
    ($args:ident, mut $b:ident: $ty:ident $( $rest:tt )* ) => {
        let mut $b: $ty = std::convert::TryFrom::try_from($args.next().unwrap()).unwrap();
        parse_args!($args $( $rest )* )
    };
    ($args:ident, $b:ident: $ty:ident $( $rest:tt )* ) => {
        let $b: $ty = std::convert::TryFrom::try_from($args.next().unwrap()).unwrap();
        parse_args!($args $( $rest )* )
    };
    ($args:ident, $b:ident $( $rest:tt )* ) => {
        let $b = $args.next().unwrap();
        parse_args!($args $( $rest )* )
    };
    ($args:ident, mut $b:ident $( $rest:tt )* ) => {
        let mut $b = $args.next().unwrap();
        parse_args!($args $( $rest )* )
    };
    ($args:ident) => {
        assert!($args.next().is_none());
    };
    ($args:ident,) => {
        parse_args!($args)
    };
}

#[macro_export]
macro_rules! static_assert_size {
    ($ty:ty, $size:expr) => {
        const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    };
}

fn name<T>() -> &'static str {
    let name = std::any::type_name::<T>();
    name.rfind(':').map_or(name, |tail| &name[tail + 1..])
}

pub(crate) fn find_rust_code_blocks(text: &str) -> Vec<Symbol> {
    lazy_regex::lazy_regex!(r"(?s)```rust\s*\n(.*?)\n\s*```")
        .captures_iter(text)
        .map(|item| item[1].into())
        .collect()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools as _;

    use crate::test::expect;

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
