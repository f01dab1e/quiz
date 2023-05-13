use std::fs::File;
use std::iter::zip;
use std::path::PathBuf;

use itertools::Itertools;
use miette::{Context, IntoDiagnostic};
use syntect::easy::HighlightLines;
use wca::{Args, Props, Value};

use crate::config::Config;
use crate::questions::Question;
use crate::{questions, Result};

pub fn import_from(args: Args, _props: Props) -> Result<()> {
    let mut conf = Config::from_home_dir()?;

    let mut args = args.0.into_iter();
    parse_args!(args, path: PathBuf);

    let path = path
        .canonicalize()
        .into_diagnostic()
        .with_context(|| format!("reading `{}`", path.display()))?;

    conf.paths.insert(path);
    conf.save()
}

pub fn questions_list(_args: Args, _props: Props) -> Result<()> {
    let conf = Config::from_home_dir()?;
    dbg!(crate::questions::read(conf.paths)?);
    Ok(())
}

pub fn questions_about(_args: Args, _props: Props) -> Result<()> {
    use prettytable::{row, Row, Table};

    let conf = Config::from_home_dir()?;
    let questions = questions::read(conf.paths)?;

    let rows: Vec<_> = questions
        .into_iter()
        .map(|question| Row::new(vec![(&question.title).into(), (&question.program).into()]))
        .collect();

    let mut table = Table::new();

    table.add_row(row!["Question", "Code"]);
    table.extend(rows);
    table.printstd();

    Ok(())
}

pub fn questions_export(_args: Args, props: Props) -> Result<()> {
    use std::io::Write as _;

    use silicon::assets::HighlightingAssets;
    use silicon::formatter::ImageFormatterBuilder;
    use syntect::util::LinesWithEndings;

    let config = Config::from_home_dir()?;
    let questions = questions::read(config.paths)?;
    let export = props.get("export").map_or(true, |n| n != &Value::Number(0.0));

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
        .get(&config.theme)
        .ok_or_else(|| miette::miette!("Canot load the theme: {}", config.theme))?;

    let mut highlight_lines = HighlightLines::new(rust_syntax, theme);

    if export {
        let mut writer = File::create("output.md").into_diagnostic()?;
        writer.write_all(b"# Rust Quiz").unwrap();

        for (question, question_id) in zip(questions, 0_usize..) {
            let lines = LinesWithEndings::from(&question.program)
                .map(|line| highlight_lines.highlight_line(line, &syntax_set))
                .collect::<Result<Vec<_>, _>>()
                .into_diagnostic()?;

            let image = formatter.format(&lines, theme);
            let image_path = format!("{question_id}.png");
            image.save(&image_path).into_diagnostic()?;

            write_question(&mut writer, question, &image_path)?;
        }
    }

    Ok(())
}

fn write_question(
    writer: &mut impl std::io::Write,
    question: Question,
    image_path: &str,
) -> Result<()> {
    let Question { title, program, answer, distractors } = question;

    let distractors = distractors
        .into_iter()
        .map(|distractor| lazy_format::lazy_format!("* {distractor}"))
        .join("\n");

    writer
        .write_fmt(format_args!(
            r#"

## {title}

![]({image_path})

```rust
{program}```

* {answer} :heavy_check_mark:
{distractors}"#
        ))
        .into_diagnostic()
}
