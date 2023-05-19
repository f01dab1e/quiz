use rusqlite::Result;

use crate::ir;

#[derive(Default)]
pub(crate) struct MockDb {}

impl super::Database for MockDb {
    fn add_question(&self, _question: ir::Question) -> Result<()> {
        Ok(())
    }

    fn find_questions(
        &self,
        _has_tags: Vec<String>,
        _no_tags: Vec<String>,
    ) -> Result<Vec<ir::Question>> {
        Ok(Vec::new())
    }

    fn migrations(&self) -> Result<()> {
        Ok(())
    }
}
