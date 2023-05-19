use miette::{IntoDiagnostic as _, WrapErr as _};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

use crate::Result;

type Symbol = Box<str>;

#[serde_inline_default]
#[derive(Deserialize, Serialize, Default)]
pub(crate) struct Config {
    #[serde_inline_default("GitHub".into())]
    pub(crate) theme: String,
}

impl Config {
    pub(crate) fn from_home_dir() -> Result<Self> {
        use std::io::ErrorKind;

        let path = crate::path::config();
        let input = match std::fs::read_to_string(&path) {
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(String::new()),
            input => {
                input.into_diagnostic().with_context(|| format!("reading `{}`", path.display()))
            }
        }?;

        toml::from_str(&input).into_diagnostic()
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub(crate) struct Questions {
    pub(crate) questions: Vec<Question>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Question {
    pub(crate) id: Option<i64>,
    pub(crate) description: Symbol,
    pub(crate) answer: Symbol,
    pub(crate) distractors: Box<[Symbol]>,
    pub(crate) tags: Box<[Symbol]>,
}

impl IntoIterator for Questions {
    type Item = Question;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.questions.into_iter()
    }
}

#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
mod size_asserts {
    use super::*;

    static_assert_size!(Config, 24);
    static_assert_size!(Question, 80);
    static_assert_size!(Questions, 24);
}
