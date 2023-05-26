use miette::{IntoDiagnostic as _, WrapErr as _};
use serde::{Deserialize, Serialize};
use stdx::Result;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    pub theme: Option<String>,
}

impl Config {
    pub fn from_home_dir() -> Result<Self> {
        use std::io::ErrorKind;

        let path = crate::path::config();
        let input = match std::fs::read_to_string(&path) {
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(String::new()),
            input => {
                input.into_diagnostic().with_context(|| format!("reading `{}`", path.display()))
            }
        }?;

        ::toml::from_str(&input).into_diagnostic()
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Questions {
    pub(crate) questions: Vec<Question>,
}

impl IntoIterator for Questions {
    type Item = Question;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.questions.into_iter()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Question {
    pub id: Option<i64>,
    pub description: Box<str>,
    pub answer: Box<str>,
    pub distractors: Box<[Box<str>]>,
    pub tags: Box<[Box<str>]>,
}

#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
mod size_asserts {
    use stdx::static_assert_size;

    use super::*;

    static_assert_size!(Config, 24);
    static_assert_size!(Question, 80);
    static_assert_size!(Questions, 24);
}
