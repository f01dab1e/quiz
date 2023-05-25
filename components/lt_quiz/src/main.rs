//! lt-quiz

#![allow(incomplete_features)]
#![feature(generic_const_exprs, internal_output_capture)]

mod commands;
mod db;
mod ir;
mod path;
#[cfg(test)]
mod test;
mod toml;

pub(crate) type Result<T = (), E = miette::Report> = miette::Result<T, E>;

pub(crate) struct State {
    pub(crate) config: ir::Config,
    pub(crate) db: db::DatabaseImpl,
    pub(crate) cache: std::cell::RefCell<anymap::AnyMap>,
}

impl State {
    fn questions(
        &self,
        has_tags: Vec<String>,
        no_tags: Vec<String>,
    ) -> Result<Vec<toml::Question>> {
        use miette::IntoDiagnostic as _;

        use crate::db::Database as _;

        let mut cache = self.cache.borrow_mut();

        match cache.get::<Vec<toml::Question>>() {
            Some(questions) => Ok(questions.clone()),
            None => {
                let questions = self.db.find_questions(has_tags, no_tags).into_diagnostic()?;
                cache.insert(questions.clone());
                Ok(questions)
            }
        }
    }
}

fn mk_aggregator(state: &'static State) -> wca::CommandsAggregator {
    use stdx::{cli, CommandExt as _, Property};
    use wca::Type;

    let list = Type::List(Box::new(Type::String), ',');
    let filter = [
        Property { name: "has_tags", hint: "lol", tag: list.clone() },
        Property { name: "no_tags", hint: "lol", tag: list },
    ];

    cli(state)
        .command(commands::import_from.arg("file", Type::Path))
        .command(commands::questions_list.properties(filter.clone()))
        .command(commands::questions_list.properties(filter.clone()))
        .command(commands::questions_about.properties(filter.clone()))
        .command(commands::questions.properties(filter.clone()))
        .command(commands::export.arg("file", Type::Path).properties(filter))
        .command(commands::config)
        .build()
}

fn main() -> Result {
    use itertools::Itertools as _;
    use miette::IntoDiagnostic as _;

    let state = {
        let state = State {
            config: ir::Config::from_home_dir()?,
            db: db::init().into_diagnostic()?,
            cache: anymap::AnyMap::new().into(),
        };
        Box::leak(Box::new(state))
    };

    let args = std::env::args().skip(1).join(" ");
    mk_aggregator(state).perform(args).into_diagnostic()?;

    Ok(())
}
