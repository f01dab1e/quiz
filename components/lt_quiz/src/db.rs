mod sqlite;

use miette::IntoDiagnostic as _;
use rusqlite::Connection;

pub(crate) use self::sqlite::Sqlite;

pub(crate) fn init() -> stdx::Result<Sqlite> {
    let db = Sqlite { conn: Connection::open(lt_quiz_core::path::db()).into_diagnostic()? };
    lt_quiz_core::traits::Database::migrations(&db)?;
    Ok(db)
}
