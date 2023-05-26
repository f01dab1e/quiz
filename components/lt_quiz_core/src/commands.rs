use miette::IntoDiagnostic as _;

use crate::traits::Database;

pub fn import_from(db: &impl Database, questions: crate::toml::Questions) -> stdx::Result {
    db.add_questions(questions).into_diagnostic()
}
