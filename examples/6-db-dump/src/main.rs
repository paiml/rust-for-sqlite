// Lesson 2.3 — Dumping and migrating databases
// Demonstrates: generating a SQL dump and schema migrations (ALTER TABLE).

use rusqlite::{Connection, Result, params};

/// Produce a minimal SQL dump: schema + INSERT statements.
fn dump_database(conn: &Connection) -> Result<String> {
    let mut out = String::new();

    // Dump all table schemas
    let mut schema_stmt = conn.prepare(
        "SELECT sql FROM sqlite_schema WHERE type='table' AND sql IS NOT NULL ORDER BY rootpage",
    )?;
    let schemas: Vec<String> = schema_stmt
        .query_map([], |row| row.get(0))?
        .filter_map(|r| r.ok())
        .collect();

    for sql in &schemas {
        out.push_str(sql);
        out.push_str(";\n");
    }
    out.push('\n');

    // Dump all rows from each user table
    let table_names: Vec<String> = conn
        .prepare("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY rootpage")?
        .query_map([], |row| row.get(0))?
        .filter_map(|r| r.ok())
        .collect();

    for table in &table_names {
        let mut row_stmt = conn.prepare(&format!("SELECT * FROM \"{table}\""))?;
        let col_count = row_stmt.column_count();

        let rows_result: Vec<Vec<String>> = row_stmt
            .query_map([], |row| {
                let mut cols = Vec::new();
                for i in 0..col_count {
                    let val: rusqlite::types::Value = row.get(i)?;
                    let s = match val {
                        rusqlite::types::Value::Null => "NULL".to_string(),
                        rusqlite::types::Value::Integer(n) => n.to_string(),
                        rusqlite::types::Value::Real(f) => f.to_string(),
                        rusqlite::types::Value::Text(t) => format!("'{}'", t.replace('\'', "''")),
                        rusqlite::types::Value::Blob(_) => "X''".to_string(),
                    };
                    cols.push(s);
                }
                Ok(cols)
            })?
            .filter_map(|r| r.ok())
            .collect();

        for cols in rows_result {
            out.push_str(&format!(
                "INSERT INTO \"{table}\" VALUES ({});\n",
                cols.join(", ")
            ));
        }
    }

    Ok(out)
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute_batch(
        "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL, email TEXT);",
    )?;
    conn.execute("INSERT INTO users (name, email) VALUES (?1, ?2)", params!["Alice", "alice@example.com"])?;
    conn.execute("INSERT INTO users (name, email) VALUES (?1, ?2)", params!["Bob", "bob@example.com"])?;

    let dump = dump_database(&conn)?;
    println!("=== SQL Dump ===\n{dump}");

    // Schema migration: add a new column
    conn.execute_batch("ALTER TABLE users ADD COLUMN created_at TEXT;")?;
    println!("Migration applied: added 'created_at' column.");

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM users", [], |r| r.get(0))?;
    println!("Rows still intact after migration: {count}");

    Ok(())
}
