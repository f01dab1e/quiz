#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![deny(clippy::use_self, unused_qualifications)]

#[macro_use]
mod stdx;
mod commands;
mod config;
mod questions;

pub type Result<T = (), E = miette::Report> = miette::Result<T, E>;

fn mk_aggregator() -> wca::CommandsAggregator {
    use wca::Type;

    use crate::stdx::{cli, CommandExt as _, Property};

    let list = Type::List(Box::new(Type::String), ',');
    let filter = [
        Property { name: "has_tags", hint: "lol", tag: list.clone() },
        Property { name: "no_tags", hint: "lol", tag: list },
    ];

    cli()
        .command(commands::import_from.arg("file", Type::String))
        .command(commands::questions_list.properties(filter.clone()))
        .command(commands::questions_list.properties(filter.clone()))
        .command(commands::questions_about)
        .command(commands::questions_export)
        .build()
}

fn main() -> Result {
    use itertools::Itertools as _;
    use miette::IntoDiagnostic as _;

    let args = std::env::args().skip(1).join(" ");
    mk_aggregator().perform(args).into_diagnostic()?;

    Ok(())
}
