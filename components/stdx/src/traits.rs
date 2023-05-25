use crate::ir::{Builder, Property};
use crate::Result;

#[allow(missing_docs)]
pub trait CommandExt<T>: Sized {
    fn arg(self, hint: &str, tag: wca::Type) -> Builder<Self> {
        Builder::new(self).arg(hint, tag)
    }

    fn properties<const N: usize>(self, properties: [Property; N]) -> Builder<Self> {
        Builder::new(self).properties(properties)
    }
}

impl<F: Fn(T, wca::Args, wca::Props) -> Result, T> CommandExt<T> for F {}

#[allow(missing_docs)]
pub trait IntoBuilder<F, T>: Sized {
    fn into_builder(self) -> Builder<F>;
}

impl<F, T> IntoBuilder<F, T> for Builder<F> {
    fn into_builder(self) -> Self {
        self
    }
}

impl<F: Fn(T, wca::Args, wca::Props) -> Result, T> IntoBuilder<F, T> for F {
    fn into_builder(self) -> Builder<F> {
        Builder::new(self)
    }
}
