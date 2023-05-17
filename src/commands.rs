use std::path::PathBuf;

use miette::{IntoDiagnostic as _, WrapErr as _};
use wca::{Args, Props};

use crate::config::Config;
use crate::questions::{Question, Questions};
use crate::Result;

pub fn import_from(args: Args, _props: Props) -> Result {
    let mut conf = Config::from_home_dir()?;

    let mut args = args.0.into_iter();
    parse_args!(args, path: PathBuf);

    let path = path
        .canonicalize()
        .into_diagnostic()
        .with_context(|| format!("process file `{}`", path.display()))?;

    conf.paths.insert(path);
    conf.save()
}

pub fn questions_list(_args: Args, _props: Props) -> Result {
    let conf = Config::from_home_dir()?;
    dbg!(Questions::read(conf.paths)?);
    Ok(())
}

pub fn questions_about(_args: Args, _props: Props) -> Result {
    use prettytable::{row, Cell, Table};

    let conf = Config::from_home_dir()?;
    let questions = Questions::read(conf.paths)?;

    let rows: Vec<_> = questions
        .into_iter()
        .map(|question| row![Cell::from(&question.title), Cell::from(&question.program)])
        .collect();

    let mut table = Table::new();
    table.add_row(row!["Question", "Code"]);
    table.extend(rows);
    table.printstd();

    Ok(())
}

pub fn questions_export(_args: Args, _props: Props) -> Result {
    use std::io::Write as _;

    use silicon::assets::HighlightingAssets;

    let config = Config::from_home_dir()?;
    let questions = Questions::read(config.paths)?;

    let mut formatter = {
        silicon::formatter::ImageFormatterBuilder::new()
            // fallback 'Hack; SimSun=31'
            .font(Vec::<(String, f32)>::new())
            .build()
            .into_diagnostic()?
    };

    let HighlightingAssets { syntax_set, theme_set } = HighlightingAssets::new();

    let rust_syntax = syntax_set.find_syntax_by_extension("rs").unwrap();
    let theme = theme_set
        .themes
        .get(&config.theme)
        .ok_or_else(|| miette::miette!("Canot load the theme: {}", config.theme))?;

    let mut writer = std::fs::File::create("output.md").into_diagnostic()?;
    writer.write_all(b"# Rust Quiz").into_diagnostic()?;

    let mut highlight_lines = syntect::easy::HighlightLines::new(rust_syntax, theme);
    for (question, question_id) in std::iter::zip(questions, 0_usize..) {
        let lines = syntect::util::LinesWithEndings::from(&question.program)
            .map(|line| highlight_lines.highlight_line(line, &syntax_set))
            .collect::<Result<Vec<_>, _>>()
            .into_diagnostic()?;

        let image = formatter.format(&lines, theme);
        let image_path = format!("{question_id}.png");
        image.save(&image_path).into_diagnostic()?;
        write_question(&mut writer, question, &image_path)?;
    }

    Ok(())
}

fn write_question(
    writer: &mut impl std::io::Write,
    question: Question,
    image_path: &str,
) -> Result {
    use itertools::Itertools as _;

    let Question { title, answer, distractors, .. } = question;
    let distractors =
        distractors.iter().map(|distractor| lazy_format::lazy_format!("* {distractor}")).join("\n");

    write!(
        writer,
        r#"

## {title}

![]({image_path})

* {answer} :heavy_check_mark:
{distractors}"#
    )
    .into_diagnostic()
}
