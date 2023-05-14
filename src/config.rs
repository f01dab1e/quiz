use std::collections::HashSet;
use std::path::PathBuf;

use miette::{IntoDiagnostic as _, WrapErr as _};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

use crate::Result;

#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
static_assert_size!(Config, 72);

#[serde_inline_default]
#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    #[serde_inline_default("GitHub".into())]
    pub theme: String,
    #[serde(default = "HashSet::new")]
    pub paths: HashSet<PathBuf>,
}

impl Config {
    fn path() -> PathBuf {
        let filename = format!(".{}.toml", env!("CARGO_PKG_NAME"));
        home::home_dir().unwrap_or_default().join(filename)
    }

    pub fn from_home_dir() -> Result<Self> {
        use std::io::ErrorKind;

        let path = Self::path();
        let input = match std::fs::read_to_string(&path) {
            Err(err) if err.kind() == ErrorKind::NotFound => return Ok(<_>::default()),
            input => {
                input.into_diagnostic().with_context(|| format!("reading `{}`", path.display()))
            }
        }?;

        toml::from_str(&input).into_diagnostic()
    }

    pub fn save(self) -> Result {
        let path = Self::path();
        let toml = toml::to_string(&self).unwrap();

        std::fs::write(&path, toml)
            .into_diagnostic()
            .with_context(|| format!("process file `{}`", path.display()))
    }
}
