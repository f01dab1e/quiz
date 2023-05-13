use std::collections::HashSet;
use std::path::PathBuf;

use miette::{Context, IntoDiagnostic};
use serde::{Deserialize, Serialize};

use crate::Result;

type Str = Box<str>;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Questions {
    pub questions: Vec<Question>,
}

impl Questions {
    fn extend(&mut self, other: Self) {
        self.questions.extend(other);
    }
}

impl IntoIterator for Questions {
    type Item = Question;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.questions.into_iter()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Question {
    pub title: Str,
    pub program: Str,
    pub answer: Str,
    pub distractors: Vec<Str>,
}

pub fn read(paths: HashSet<PathBuf>) -> Result<Questions> {
    let mut questions = Questions::default();

    for path in paths {
        questions.extend(read0(path)?);
    }

    Ok(questions)
}

fn read0(path: PathBuf) -> Result<Questions> {
    let input = std::fs::read_to_string(&path)
        .into_diagnostic()
        .with_context(|| format!("reading `{}`", path.display()))?;

    toml::from_str(&input).into_diagnostic()
}
