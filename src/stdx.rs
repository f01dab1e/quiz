pub fn cli() -> CommandBuilder<0> {
    CommandBuilder::new()
}

pub struct CommandBuilder<const N: usize> {
    commands: [wca::Command; N],
    handlers: [(String, wca::Routine); N],
}

impl CommandBuilder<0> {
    fn new() -> Self {
        Self { handlers: [], commands: [] }
    }
}

impl<const N: usize> CommandBuilder<N> {
    pub fn command(
        self,
        name: impl ToString,
        callback: impl Fn(wca::Args, wca::Props) -> crate::Result + 'static,
    ) -> CommandBuilder<{ N + 1 }> {
        let name = name.to_string();
        let handler = wca::Routine::new(move |(args, props)| {
            callback(args, props).map_err(|report| wca::BasicError::new(format!("{report:?}")))
        });

        CommandBuilder {
            handlers: array_push(self.handlers, (name.clone(), handler)),
            commands: array_push(self.commands, wca::Command::former().phrase(name).form()),
        }
    }

    pub fn arg(mut self, hint: &str, ty: wca::Type, optional: bool) -> Self
    where
        Bool<{ N > 0 }>: True,
    {
        let command = &mut self.commands[N - 1];

        command.subjects.push(wca::grammar::settings::ValueDescription {
            hint: hint.into(),
            kind: ty,
            optional,
        });

        self
    }

    pub fn build(self) -> wca::CommandsAggregator {
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

pub struct Bool<const B: bool> {}

pub trait True {}

impl True for Bool<true> {}

pub trait False {}

impl False for Bool<false> {}

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
