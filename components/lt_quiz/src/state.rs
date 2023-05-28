use lt_quiz_core::ir;

use crate::{db, toml, Result};

pub(crate) struct State {
    pub(crate) config: ir::Config,
    pub(crate) db: db::Sqlite,
    pub(crate) cache: std::cell::RefCell<anymap::AnyMap>,
}

impl State {
    pub(crate) fn questions(
        &self,
        has_tags: Vec<String>,
        no_tags: Vec<String>,
    ) -> Result<Vec<toml::Question>> {
        use lt_quiz_core::traits::Database as _;

        let mut cache = self.cache.borrow_mut();
        match cache.get::<Vec<toml::Question>>() {
            Some(questions) => Ok(questions.clone()),
            None => {
                let questions = self.db.find_questions(has_tags, no_tags)?;
                cache.insert(questions.clone());
                Ok(questions)
            }
        }
    }
}
