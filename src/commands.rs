use std::path::PathBuf;

use itertools::Itertools as _;
use miette::{IntoDiagnostic as _, WrapErr as _};
use wca::{Args, Props};

use crate::db::Database as _;
use crate::{ir, stdx, Result, State};

pub(crate) fn import_from(State { db, .. }: &State, args: Args, _props: Props) -> Result {
    let mut args = args.0.into_iter();
    parse_args!(args, path: PathBuf);

    let questions: ir::Questions = {
        let input = std::fs::read_to_string(&path)
            .into_diagnostic()
            .with_context(|| format!("reading `{}`", path.display()))?;

        toml::from_str(&input).into_diagnostic()?
    };

    for question in questions {
        db.add_question(question).into_diagnostic()?;
    }

    Ok(())
}

pub(crate) fn questions_list(State { db, .. }: &State, _args: Args, props: Props) -> Result {
    let has_tags = props.get_owned("has_tags").unwrap_or_default();
    let no_tags = props.get_owned("no_tags").unwrap_or_default();

    let questions = db.find_questions(has_tags, no_tags).into_diagnostic()?;

    for ir::Question { id, description, answer, distractors, .. } in questions {
        let id = id.unwrap();
        let description: String = description.chars().take(60).collect();
        let distractors = distractors.join("\n");

        println!("{id}. {description}\nAnswer:\n{answer}\nDistractors:\n{distractors}");
    }

    Ok(())
}

pub(crate) fn questions_about(State { db, .. }: &State, _args: Args, props: Props) -> Result {
    use prettytable::{row, Table};

    let has_tags = props.get_owned("has_tags").unwrap_or_default();
    let no_tags = props.get_owned("no_tags").unwrap_or_default();

    let mut table = Table::new();
    let mut rows = Vec::new();

    let questions = db.find_questions(has_tags, no_tags).into_diagnostic()?;
    for ir::Question { id, description, answer, distractors, .. } in questions {
        let distractors = distractors.iter().join("\n");
        rows.push(row![id.unwrap(), description, answer, distractors]);
    }

    table.add_row(row!["ID", "Description", "Answer", "Distractors"]);
    table.extend(rows);
    println!("{}", table);

    Ok(())
}

pub(crate) fn questions(state: &State, _args: Args, props: Props) -> Result {
    let has_tags = props.get_owned("has_tags").unwrap_or_default();
    let no_tags = props.get_owned("no_tags").unwrap_or_default();

    let questions = state.questions(has_tags, no_tags)?;
    println!("{:?}", questions);

    Ok(())
}

pub(crate) fn export(state: &State, args: Args, props: Props) -> Result {
    use std::io::Write as _;
    use std::iter::zip;

    use silicon::assets::HighlightingAssets;

    let mut args = args.0.into_iter();
    parse_args!(args, path: PathBuf);

    let mut writer = std::fs::File::create(path).into_diagnostic()?;
    writer.write_all(b"# Rust Quiz").into_diagnostic()?;

    let has_tags = props.get_owned("has_tags").unwrap_or_default();
    let no_tags = props.get_owned("no_tags").unwrap_or_default();

    let mut formatter = {
        silicon::formatter::ImageFormatterBuilder::new()
            // fallback 'Hack; SimSun=31'
            .font(Vec::<(String, f32)>::new())
            .build()
            .into_diagnostic()?
    };

    let HighlightingAssets { syntax_set, theme_set } = HighlightingAssets::new();

    let theme = theme_set
        .themes
        .get(&state.config.theme)
        .ok_or_else(|| miette::miette!("Canot load the theme: {}", state.config.theme))?;

    let mut highlight_lines = {
        let rust_syntax = syntax_set.find_syntax_by_extension("rs").unwrap();

        syntect::easy::HighlightLines::new(rust_syntax, theme)
    };

    let questions = state.db.find_questions(has_tags, no_tags).into_diagnostic()?;
    for question in questions {
        for (code, index) in zip(stdx::find_rust_code_blocks(&question.description), 0_usize..) {
            let lines = syntect::util::LinesWithEndings::from(&code)
                .map(|line| highlight_lines.highlight_line(line, &syntax_set))
                .collect::<Result<Vec<_>, _>>()
                .into_diagnostic()?;

            let image = formatter.format(&lines, theme);
            let image_path = format!("{index}.png");
            image.save(&image_path).into_diagnostic()?;
        }

        write_question(&mut writer, question)?;
    }

    Ok(())
}

pub(crate) fn config(state: &State, _args: Args, _props: Props) -> Result {
    println!("Theme: {}", state.config.theme);

    Ok(())
}

fn write_question(writer: &mut impl std::io::Write, question: ir::Question) -> Result {
    use itertools::Itertools as _;

    let ir::Question { id, description, answer, distractors, .. } = question;
    let id = id.unwrap();
    let distractors =
        distractors.iter().map(|distractor| lazy_format::lazy_format!("* {distractor}")).join("\n");

    write!(
        writer,
        r#"

## {id} 

{description}

* {answer} :heavy_check_mark:
{distractors}"#
    )
    .into_diagnostic()
}

#[cfg(test)]
mod tests {
    use crate::commands;
    use crate::test::{expect, World};

    #[test]
    fn empty_table() {
        World::default().assert(
            commands::questions_about,
            expect![[r#"
            +----+-------------+--------+-------------+
            | ID | Description | Answer | Distractors |
            +----+-------------+--------+-------------+

        "#]],
        );
    }

    #[test]
    fn question_table() {
        World::default().question("Memory safety in Rust", "Unsafe", &["Safe"]).assert(
            commands::questions_about,
            expect![[r#"
                +----+-----------------------+--------+-------------+
                | ID | Description           | Answer | Distractors |
                +----+-----------------------+--------+-------------+
                | 0  | Memory safety in Rust | Unsafe | Safe        |
                +----+-----------------------+--------+-------------+

            "#]],
        );
    }

    #[test]
    fn config() {
        World::default().assert(
            commands::config,
            expect![[r#"
                Theme: 
            "#]],
        );
    }
}
