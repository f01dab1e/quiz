/// Macro for parsing WCA arguments.
///
/// # Examples
/// ```rust
/// use wca::Value;
///
/// let mut args = vec![Value::Number(42.), Value::String("Rust".into())].into_iter();
/// stdx::parse_args!(args, n: f64, name: String);
///
/// assert_eq!(n, 42.);
/// assert_eq!(name, "Rust");
/// ```
#[macro_export]
macro_rules! parse_args {
    ($args:ident, mut $b:ident: $ty:ident $( $rest:tt )* ) => {
        let mut $b: $ty = std::convert::TryFrom::try_from($args.next().unwrap()).unwrap();
        $crate::parse_args!($args $( $rest )* )
    };
    ($args:ident, $b:ident: $ty:ident $( $rest:tt )* ) => {
        let $b: $ty = std::convert::TryFrom::try_from($args.next().unwrap()).unwrap();
        $crate::parse_args!($args $( $rest )* )
    };
    ($args:ident, $b:ident $( $rest:tt )* ) => {
        let $b = $args.next().unwrap();
        $crate::parse_args!($args $( $rest )* )
    };
    ($args:ident, mut $b:ident $( $rest:tt )* ) => {
        let mut $b = $args.next().unwrap();
        $crate::parse_args!($args $( $rest )* )
    };
    ($args:ident) => {
        assert!($args.next().is_none());
    };
    ($args:ident,) => {
        $crate::parse_args!($args)
    };
}

/// Type size assertion. The first argument is a type and the second argument is
/// its expected size.
#[macro_export]
macro_rules! static_assert_size {
    ($ty:ty, $size:expr) => {
        const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    };
}
