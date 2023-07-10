//! lt_quiz

#![allow(incomplete_features)]
#![feature(internal_output_capture)]

mod commands;
mod db;
mod path;
mod state;
#[cfg(test)]
mod test;

use lt_quiz_core::toml;
pub(crate) use stdx::Result;

fn mk_aggregator(state: state::State) -> wca::CommandsAggregator {
    use wca::stdx::{cli, CommandExt as _, Property};
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

    let state = state::State::new(lt_quiz_core::ir::Config::from_dir(path::config())?, db::init()?);

    let args = std::env::args().skip(1).join(" ");
    mk_aggregator(state).perform(args).into_diagnostic()?;

    Ok(())
}
