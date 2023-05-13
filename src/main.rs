#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![deny(clippy::use_self, unused_qualifications)]

#[macro_use]
mod helpers;
mod commands;
mod config;
mod questions;

pub use miette::Result;

fn mk_aggregator() -> wca::CommandsAggregator {
    use wca::{Command, CommandsAggregator, Type};

    use crate::helpers::routines;

    let commands = [
        Command::former().phrase("import.from").subject("file", Type::Path, true).form(),
        Command::former().phrase("questions.list").form(),
        Command::former().phrase("questions.about").form(),
        Command::former().phrase("questions").property("export", "lol", Type::Number, true).form(), // TODO: .export
        Command::former().phrase("export").form(),
    ];

    CommandsAggregator::former()
        .grammar(commands)
        .executor(
            routines()
                .routine("import.from", commands::import_from)
                .routine("questions.list", commands::questions_list)
                .routine("questions.about", commands::questions_about)
                .routine("questions", commands::questions_export)
                .build(),
        )
        .build()
}

fn main() -> Result<()> {
    use itertools::Itertools as _;
    use miette::IntoDiagnostic as _;

    let args = std::env::args().skip(1).join(" ");
    mk_aggregator().perform(args).into_diagnostic()?;

    Ok(())
}
