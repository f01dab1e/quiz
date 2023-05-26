mod sqlite;

use rusqlite::{Connection, Result};

pub(crate) use self::sqlite::Sqlite;

pub(crate) fn init() -> Result<Sqlite> {
    let db = Sqlite { conn: Connection::open(lt_quiz_core::path::db())? };
    lt_quiz_core::traits::Database::migrations(&db)?;
    Ok(db)
}
