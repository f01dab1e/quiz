use stdx::Result;

use crate::toml;

/// Database trait for managing questions.
pub trait Database {
    /// Adds a single question to the database.
    fn add_question(&self, question: toml::Question) -> Result;
    /// Adds multiple questions to the database.
    fn add_questions(&self, questions: toml::Questions) -> Result {
        for question in questions {
            self.add_question(question)?;
        }
        Ok(())
    }

    /// Finds questions based on specified tags.
    fn find_questions(
        &self,
        has_tags: Vec<String>,
        no_tags: Vec<String>,
    ) -> Result<Vec<toml::Question>>;

    /// Performs any necessary database migrations.
    fn migrations(&self) -> Result;
}
