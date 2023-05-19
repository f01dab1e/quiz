use std::path::PathBuf;

use itertools::Itertools as _;
use miette::{IntoDiagnostic as _, WrapErr as _};
use wca::{Args, Props};

use crate::{ir, Result, State};

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
    table.printstd();

    Ok(())
}

pub(crate) fn questions_export(State { db, .. }: &State, _args: Args, props: Props) -> Result {
    use std::io::Write as _;

    let mut writer = std::fs::File::create("output.md").into_diagnostic()?;
    writer.write_all(b"# Rust Quiz").into_diagnostic()?;

    let has_tags = props.get_owned("has_tags").unwrap_or_default();
    let no_tags = props.get_owned("no_tags").unwrap_or_default();

    let questions = db.find_questions(has_tags, no_tags).into_diagnostic()?;
    for question in questions {
        write_question(&mut writer, question)?;
    }

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
