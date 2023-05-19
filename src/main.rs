#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![deny(clippy::use_self, unused_qualifications, unreachable_pub)]

#[macro_use]
mod stdx;
mod commands;
mod db;
mod ir;
mod path;

pub(crate) type Result<T = (), E = miette::Report> = miette::Result<T, E>;

pub(crate) struct State {
    #[allow(dead_code)]
    pub(crate) config: ir::Config,
    pub(crate) db: db::Database,
}

fn mk_aggregator(state: &'static State) -> wca::CommandsAggregator {
    use wca::Type;

    use crate::stdx::{cli, CommandExt as _, Property};

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
        .command(commands::questions_export.properties(filter))
        .build()
}

fn main() -> Result {
    use itertools::Itertools as _;
    use miette::IntoDiagnostic as _;

    let state = {
        let state =
            State { config: ir::Config::from_home_dir()?, db: db::init().into_diagnostic()? };
        Box::leak(Box::new(state))
    };

    let args = std::env::args().skip(1).join(" ");
    mk_aggregator(state).perform(args).into_diagnostic()?;

    Ok(())
}
