use std::collections::HashSet;
use std::path::PathBuf;

use miette::{IntoDiagnostic as _, WrapErr as _};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

use crate::Result;

#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
static_assert_size!(Config, 72);

#[serde_inline_default]
#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde_inline_default("GitHub".into())]
    pub theme: String,
    #[serde(default = "HashSet::new")]
    pub paths: HashSet<PathBuf>,
}

impl Config {
    fn path() -> PathBuf {
        let config =
            home::home_dir().unwrap_or_default().join(concat!(".", env!("CARGO_PKG_NAME")));
        let _ = std::fs::create_dir(&config);

        config.join("config.toml")
    }

    pub fn from_home_dir() -> Result<Self> {
        use std::io::ErrorKind;

        let path = Self::path();
        let input = match std::fs::read_to_string(&path) {
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(String::new()),
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
