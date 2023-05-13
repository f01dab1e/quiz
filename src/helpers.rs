use std::mem::MaybeUninit;

use wca::{Args, BasicError, Props, Routine};

pub fn routines() -> Routines<0> {
    Routines::new()
}

pub struct Routines<const N: usize> {
    routines: [(String, Routine); N],
}

impl Routines<0> {
    fn new() -> Self {
        Self { routines: [] }
    }
}

impl<const N: usize> Routines<N> {
    pub fn routine(
        self,
        name: impl ToString,
        callback: impl Fn(Args, Props) -> miette::Result<()> + 'static,
    ) -> Routines<{ N + 1 }> {
        Routines {
            routines: array_push(
                self.routines,
                (
                    name.to_string(),
                    Routine::new(move |(args, props)| {
                        callback(args, props)
                            .map_err(|report| BasicError::new(format!("{report:?}")))
                    }),
                ),
            ),
        }
    }

    pub fn build(self) -> [(String, Routine); N] {
        self.routines
    }
}

fn array_push<const N: usize, T>(this: [T; N], item: T) -> [T; N + 1] {
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
