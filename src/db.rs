mod memory;
mod sqlite;

use enum_dispatch::enum_dispatch;
use rusqlite::{Connection, Result};

pub(crate) use self::memory::Memory;
pub(crate) use self::sqlite::Sqlite;
use crate::ir;

pub(crate) fn init() -> Result<DatabaseImpl> {
    let db = Sqlite { conn: Connection::open(crate::path::db())? };
    db.migrations()?;
    Ok(db.into())
}

#[enum_dispatch]
pub(crate) enum DatabaseImpl {
    Sqlite(Sqlite),
    Memory(Memory),
}

#[enum_dispatch(DatabaseImpl)]
pub(crate) trait Database {
    fn add_question(&self, question: ir::Question) -> Result<()>;
    fn find_questions(
        &self,
        has_tags: Vec<String>,
        no_tags: Vec<String>,
    ) -> Result<Vec<ir::Question>>;
    fn migrations(&self) -> Result<()>;
}
