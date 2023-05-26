use rusqlite::Result;

use crate::toml;

pub trait Database {
    fn add_question(&self, question: toml::Question) -> Result<()>;
    fn add_questions(&self, questions: toml::Questions) -> Result<()> {
        for question in questions {
            self.add_question(question)?;
        }
        Ok(())
    }
    fn find_questions(
        &self,
        has_tags: Vec<String>,
        no_tags: Vec<String>,
    ) -> Result<Vec<toml::Question>>;
    fn migrations(&self) -> Result<()>;
}
