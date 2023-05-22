use std::sync::Arc;

pub(crate) use expect_test::{expect, Expect};

use crate::db::{Database, Memory};
use crate::{ir, State};

#[allow(dead_code)]
pub(crate) struct World {
    state: State,
    args: wca::Args,
    props: wca::Props,
}

impl Default for World {
    fn default() -> Self {
        Self {
            state: State {
                config: <_>::default(),
                db: Memory::default().into(),
                cache: anymap::AnyMap::new().into(),
            },
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
            .add_question(ir::Question {
                id: None,
                description: description.into(),
                answer: answer.into(),
                distractors: distractors.iter().copied().map(Box::from).collect(),
                tags: <_>::default(),
            })
            .unwrap();

        self
    }

    pub(crate) fn assert(
        self,
        handler: impl Fn(&State, wca::Args, wca::Props) -> crate::Result,
        expect: Expect,
    ) {
        std::io::set_output_capture(Some(Default::default()));

        handler(&self.state, self.args, self.props).unwrap();

        let captured = std::io::set_output_capture(None).unwrap();
        let captured = Arc::try_unwrap(captured).unwrap().into_inner().unwrap();
        let captured = String::from_utf8(captured).unwrap();

        expect.assert_eq(&captured)
    }
}
