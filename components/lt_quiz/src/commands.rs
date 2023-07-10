use std::path::PathBuf;

use itertools::Itertools as _;
use lt_quiz_core::traits::Database as _;
use miette::{IntoDiagnostic as _, WrapErr as _};
use wca::{Args, Props};

use crate::state::State;
use crate::{toml, Result};

pub(crate) fn import_from(state: State, args: Args, _properties: Props) -> Result {
    let mut args = args.0.into_iter();
    wca::parse_args!(args, path: PathBuf);

    let questions: toml::Questions = {
        let input = std::fs::read_to_string(&path)
            .into_diagnostic()
            .with_context(|| format!("reading `{}`", path.display()))?;

        ::toml::from_str(&input).into_diagnostic()?
    };

    state.db.add_questions(questions)
}

pub(crate) fn questions_list(state: State, _args: Args, properties: Props) -> Result {
    let has_tags = properties.get_owned("has_tags").unwrap_or_default();
    let no_tags = properties.get_owned("no_tags").unwrap_or_default();

    let questions = state.db.find_questions(has_tags, no_tags)?;

    for toml::Question { id, description, answer, distractors, .. } in questions {
        let id = id.unwrap();
        let description: String = description.chars().take(60).collect();
        let distractors = distractors.join("\n");

        println!("{id}. {description}\nAnswer:\n{answer}\nDistractors:\n{distractors}");
    }

    Ok(())
}

pub(crate) fn questions_about(state: State, _args: Args, properties: Props) -> Result {
    use prettytable::{row, Table};

    let has_tags = properties.get_owned("has_tags").unwrap_or_default();
    let no_tags = properties.get_owned("no_tags").unwrap_or_default();

    let mut table = Table::new();
    let mut rows = Vec::new();

    let questions = state.db.find_questions(has_tags, no_tags)?;
    for toml::Question { id, description, answer, distractors, .. } in questions {
        let distractors = distractors.iter().join("\n");
        rows.push(row![id.unwrap(), description, answer, distractors]);
    }

    table.add_row(row!["ID", "Description", "Answer", "Distractors"]);
    table.extend(rows);
    println!("{}", table);

    Ok(())
}

pub(crate) fn questions(state: State, _args: Args, properties: Props) -> Result {
    let has_tags = properties.get_owned("has_tags").unwrap_or_default();
    let no_tags = properties.get_owned("no_tags").unwrap_or_default();

    let questions = state.questions(has_tags, no_tags)?;
    println!("{:?}", questions);

    Ok(())
}

pub(crate) fn export(state: State, args: Args, properties: Props) -> Result {
    use std::io::Write as _;
    use std::iter::zip;

    use silicon::assets::HighlightingAssets;

    let mut args = args.0.into_iter();
    wca::parse_args!(args, path: PathBuf);

    let mut writer = std::fs::File::create(path).into_diagnostic()?;
    writer.write_all(b"# Rust Quiz").into_diagnostic()?;

    let has_tags = properties.get_owned("has_tags").unwrap_or_default();
    let no_tags = properties.get_owned("no_tags").unwrap_or_default();

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
        .get(state.config.theme.value())
        .ok_or_else(|| miette::miette!("Canot load the theme: {}", state.config.theme.value()))?;

    let mut highlight_lines = {
        let rust_syntax = syntax_set.find_syntax_by_extension("rs").unwrap();

        syntect::easy::HighlightLines::new(rust_syntax, theme)
    };

    let questions = state.db.find_questions(has_tags, no_tags)?;
    for question in questions {
        for (code, index) in
            zip(stdx::markdown::find_rust_code_blocks(&question.description), 0_usize..)
        {
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

pub(crate) fn config(state: State, _args: Args, _properties: Props) -> Result {
    let config = &state.config;

    println!("[{}] Theme: {}", config.theme.kind(), config.theme.value());

    Ok(())
}

fn write_question(writer: &mut impl std::io::Write, question: toml::Question) -> Result {
    use itertools::Itertools as _;

    let toml::Question { id, description, answer, distractors, .. } = question;
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

    fn empty() -> World {
        World::default()
    }

    fn world() -> World {
        World::default().question("Memory safety in Rust", "Unsafe", &["Safe"])
    }

    #[test]
    fn empty_list() {
        empty().assert(commands::questions_list, expect![]);
    }

    #[test]
    fn question_list() {
        world().assert(
            commands::questions_list,
            expect![[r#"
            1. Memory safety in Rust
            Answer:
            Unsafe
            Distractors:
            Safe
        "#]],
        );
    }

    #[test]
    fn empty_table() {
        empty().assert(
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
        world().assert(
            commands::questions_about,
            expect![[r#"
                +----+-----------------------+--------+-------------+
                | ID | Description           | Answer | Distractors |
                +----+-----------------------+--------+-------------+
                | 1  | Memory safety in Rust | Unsafe | Safe        |
                +----+-----------------------+--------+-------------+

            "#]],
        );
    }

    #[test]
    fn config() {
        empty().assert(
            commands::config,
            expect![[r#"
                [default] Theme: GitHub
            "#]],
        );
    }
}
