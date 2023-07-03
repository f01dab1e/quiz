use std::path::PathBuf;

use miette::{IntoDiagnostic as _, WrapErr as _};
use serde::{Deserialize, Serialize};
use stdx::Result;

/// Configuration structure for the application.
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    /// Represents the selected theme for code photos.
    pub theme: Option<String>,
}

impl Config {
    /// Loads the configuration from the home directory.
    ///
    /// This function reads the configuration file from the home directory using
    /// the `config` path from the `path` module. It attempts to read the
    /// file and parse its contents as TOML format. If the file does not
    /// exist, an empty string is used as the input. The function returns the
    /// deserialized configuration as a `Result`, transformed into a diagnostic
    /// error if necessary.
    pub fn from_home_dir(path: PathBuf) -> Result<Self> {
        use std::io::ErrorKind;

        let input = match std::fs::read_to_string(&path) {
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(String::new()),
            input => {
                input.into_diagnostic().with_context(|| format!("reading `{}`", path.display()))
            }
        }?;

        ::toml::from_str(&input).into_diagnostic()
    }
}

/// Represents a collection of questions.
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

/// Represents a question.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Question {
    /// The optional ID of the question.
    pub id: Option<i64>,
    /// The description of the question.
    pub description: Box<str>,
    /// The answer to the question.
    pub answer: Box<str>,
    /// The distractor options for the question.
    pub distractors: Box<[Box<str>]>,
    /// The tags associated with the question.
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
