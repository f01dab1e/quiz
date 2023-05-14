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

    crate::stdx::cli()
        .command("import.from", commands::import_from)
        .arg("file", Type::Path, false)
        .command("questions.list", commands::questions_list)
        .command("questions.about", commands::questions_about)
        .command("questions", commands::questions_export)
        .build()
}

fn main() -> Result {
    use itertools::Itertools as _;
    use miette::IntoDiagnostic as _;

    let args = std::env::args().skip(1).join(" ");
    mk_aggregator().perform(args).into_diagnostic()?;

    Ok(())
}
