use std::sync::Arc;

pub(crate) use expect_test::{expect, Expect};
use lt_quiz_core::traits::Database as _;

use crate::state::State;
use crate::toml;

#[allow(dead_code)]
pub(crate) struct World {
    state: State,
    args: wca::Args,
    props: wca::Props,
}

impl Default for World {
    fn default() -> Self {
        Self {
            state: State::new(<_>::default(), crate::db::Sqlite::memory().into()),
            args: wca::Args(<_>::default()),
            props: wca::Props(<_>::default()),
        }
    }
}

impl World {
    #[track_caller]
    pub(crate) fn question(self, description: &str, answer: &str, distractors: &[&str]) -> Self {
        self.state
            .db
            .add_question(toml::Question {
                id: None,
                description: description.into(),
                answer: answer.into(),
                distractors: distractors.iter().copied().map(Box::from).collect(),
                tags: vec!["tag".into()].into_boxed_slice(),
            })
            .unwrap();

        self
    }

    pub(crate) fn assert(
        self,
        handler: impl Fn(State, wca::Args, wca::Props) -> crate::Result,
        expect: Expect,
    ) {
        std::io::set_output_capture(Some(Default::default()));

        handler(self.state, self.args, self.props).unwrap();

        let captured = std::io::set_output_capture(None).unwrap();
        let captured = Arc::try_unwrap(captured).unwrap().into_inner().unwrap();
        let captured = String::from_utf8(captured).unwrap();

        expect.assert_eq(&captured)
    }
}
