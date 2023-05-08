#![deny(clippy::use_self, unused_qualifications)]

mod ir;
mod traits;

use std::iter::zip;

use miette::{bail, Context, IntoDiagnostic, Result};
use silicon::assets::HighlightingAssets;
use silicon::formatter::ImageFormatterBuilder;
use syntect::easy::HighlightLines;

use crate::traits::Highlight as _;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);

    match args.next() {
        // FIXME: replace with `ExactSizeIterator::is_empty` once stabilized
        Some(path) if args.len() == 0 => {
            let file: ir::File = {
                let input = std::fs::read_to_string(&path)
                    .into_diagnostic()
                    .with_context(|| format!("reading `{path}`"))?;

                toml::from_str(&input).into_diagnostic()?
            };

            let mut formatter = {
                ImageFormatterBuilder::new()
                    // fallback 'Hack; SimSun=31'
                    .font(Vec::<(String, f32)>::new())
                    .build()
                    .into_diagnostic()?
            };

            let HighlightingAssets { syntax_set, theme_set } = HighlightingAssets::new();

            let rust_syntax = syntax_set.find_syntax_by_extension("rs").unwrap();
            let theme = theme_set
                .themes
                .get(&file.theme)
                .ok_or_else(|| miette::miette!("Canot load the theme: {}", file.theme))?;

            let mut highlight_lines = HighlightLines::new(rust_syntax, theme);
            for (question, question_id) in zip(file.questions, 0_u32..) {
                let lines =
                    highlight_lines.highlight0(&question.program, &syntax_set).into_diagnostic()?;

                let image = formatter.format(&lines, theme);
                let path = format!("./images/{question_id}.png");

                image
                    .save(&path)
                    .into_diagnostic()
                    .with_context(|| format!("Failed to save image {path}"))?;
            }

            Ok(())
        }
        Some(_) => bail!("you must specify exactly one input file"),
        None => bail!("you must specify an input file"),
    }
}
