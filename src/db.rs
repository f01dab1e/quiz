use rusqlite::{params, Connection, Result};

use crate::ir;

pub(crate) fn init() -> Result<Database> {
    let db = Database { conn: Connection::open(crate::path::db())? };
    db.migrations()?;
    Ok(db)
}

pub(crate) struct Database {
    conn: Connection,
}

impl Database {
    pub(crate) fn add_question(&self, question: ir::Question) -> Result<()> {
        let conn = &self.conn;

        let distractors = serde_json::to_string(&question.distractors).unwrap();
        conn.execute(
            "INSERT INTO questions (description, answer, distractors) VALUES (?, ?, ?)",
            params![question.description, question.answer, distractors],
        )?;

        let question_id = conn.last_insert_rowid();
        for tag in question.tags.iter() {
            conn.execute("INSERT OR IGNORE INTO tags (text) VALUES (?)", [tag])?;

            let tag_id =
                conn.query_row("SELECT id FROM tags WHERE text = ?", [tag], |row| row.get(0))?;

            conn.execute(
                "INSERT INTO question_tags (question_id, tag_id) VALUES (?, ?)",
                [question_id, tag_id],
            )?;
        }

        Ok(())
    }

    pub(crate) fn find_questions(
        &self,
        has_tags: Vec<String>,
        no_tags: Vec<String>,
    ) -> Result<Vec<ir::Question>> {
        use std::fmt::Write as _;

        let conn = &self.conn;
        let mut query = "SELECT q.id, q.description, q.answer, q.distractors
            FROM questions AS q
            INNER JOIN question_tags AS qt ON q.id = qt.question_id
            INNER JOIN tags AS t ON qt.tag_id = t.id\n"
            .to_owned();

        if !has_tags.is_empty() {
            writeln!(query, "WHERE t.text IN ({})", placeholders(has_tags.len())).unwrap();
        }

        if !no_tags.is_empty() {
            writeln!(query, "AND t.text NOT IN ({})", placeholders(no_tags.len())).unwrap();
        }

        let mut stmt = conn.prepare(&query)?;

        let mut tags = has_tags;
        tags.extend(no_tags);

        let rows = stmt.query(rusqlite::params_from_iter(tags))?;
        rows.mapped(|row| {
            let id = row.get(0)?;
            let description = row.get(1)?;
            let answer = row.get(2)?;
            let distractors = {
                let json: String = row.get(3)?;
                serde_json::from_str(&json).unwrap()
            };

            Ok(ir::Question {
                id: Some(id),
                description,
                answer,
                distractors,
                tags: <_>::default(),
            })
        })
        .collect()
    }

    pub(crate) fn migrations(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS questions (
            id INTEGER PRIMARY KEY,
            description TEXT,
            answer TEXT,
            distractors TEXT
        )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY,
            text TEXT UNIQUE
        )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS question_tags (
            question_id INTEGER,
            tag_id INTEGER,
            FOREIGN KEY (question_id) REFERENCES questions(id),
            FOREIGN KEY (tag_id) REFERENCES tags(id)
        )",
            [],
        )?;

        Ok(())
    }
}

fn placeholders(n: usize) -> String {
    itertools::join(std::iter::repeat("?").take(n), ",")
}
