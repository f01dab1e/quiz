//! lt_quiz

#![allow(incomplete_features)]
#![feature(generic_const_exprs, internal_output_capture)]

mod commands;
mod db;
mod path;
mod state;
#[cfg(test)]
mod test;

use lt_quiz_core::toml;
pub(crate) use stdx::Result;

fn mk_aggregator(state: &'static state::State) -> wca::CommandsAggregator {
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
        let state = state::State {
            config: lt_quiz_core::ir::Config::from_dir(path::config())?,
            db: db::init()?,
            cache: anymap::AnyMap::new().into(),
        };
        Box::leak(Box::new(state))
    };

    let args = std::env::args().skip(1).join(" ");
    mk_aggregator(state).perform(args).into_diagnostic()?;

    Ok(())
}
