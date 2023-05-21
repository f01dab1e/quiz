use std::cell::RefCell;

use rusqlite::Result;

use crate::ir;

#[derive(Default)]
pub(crate) struct Memory {
    questions: RefCell<Vec<ir::Question>>,
}

impl super::Database for Memory {
    fn add_question(&self, mut question: ir::Question) -> Result<()> {
        let mut questions = self.questions.borrow_mut();

        question.id = Some(questions.len() as i64);
        questions.push(question);

        Ok(())
    }

    fn find_questions(
        &self,
        _has_tags: Vec<String>,
        _no_tags: Vec<String>,
    ) -> Result<Vec<ir::Question>> {
        Ok(self.questions.borrow().to_vec())
    }

    fn migrations(&self) -> Result<()> {
        Ok(())
    }
}
