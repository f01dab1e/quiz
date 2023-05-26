#![allow(missing_docs)]

pub mod path;
pub mod toml;

pub trait Database {
    fn add_question(&self, question: toml::Question) -> rusqlite::Result<()>;
    fn add_questions(&self, questions: toml::Questions) -> rusqlite::Result<()> {
        for question in questions {
            self.add_question(question)?;
        }
        Ok(())
    }
    fn find_questions(
        &self,
        has_tags: Vec<String>,
        no_tags: Vec<String>,
    ) -> rusqlite::Result<Vec<toml::Question>>;
    fn migrations(&self) -> rusqlite::Result<()>;
}
