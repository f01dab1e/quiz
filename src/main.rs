#![deny(clippy::use_self, unused_qualifications)]

use miette::{bail, Context, IntoDiagnostic, Result};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);

    match args.next() {
        // FIXME: replace with `ExactSizeIterator::is_empty` once stabilized
        Some(path) if args.len() == 0 => {
            let input = std::fs::read_to_string(&path)
                .into_diagnostic()
                .with_context(|| format!("reading `{path}`"))?;

            let toml: toml::Value = toml::from_str(&input).into_diagnostic()?;
            dbg!(toml);

            Ok(())
        }
        Some(_) => bail!("you must specify exactly one input file"),
        None => bail!("you must specify an input file"),
    }
}
