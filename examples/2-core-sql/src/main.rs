// Lesson 1.2 — Core SQL operations in Rust
// Demonstrates: CREATE TABLE, INSERT, SELECT, UPDATE, DELETE with rusqlite.

use rusqlite::{Connection, Result, params};

#[derive(Debug)]
struct Record {
    id: i64,
    name: String,
    value: f64,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    // CREATE TABLE
    conn.execute_batch(
        "CREATE TABLE records (
            id    INTEGER PRIMARY KEY,
            name  TEXT    NOT NULL,
            value REAL    NOT NULL
        );",
    )?;
    println!("Table created.");

    // INSERT with parameterized queries — prevents SQL injection
    conn.execute(
        "INSERT INTO records (name, value) VALUES (?1, ?2)",
        params!["alpha", 1.5],
    )?;
    conn.execute(
        "INSERT INTO records (name, value) VALUES (?1, ?2)",
        params!["beta", 2.7],
    )?;
    println!("Rows inserted.");

    // SELECT
    let mut stmt = conn.prepare("SELECT id, name, value FROM records ORDER BY id")?;
    let rows = stmt.query_map([], |row| {
        Ok(Record {
            id: row.get(0)?,
            name: row.get(1)?,
            value: row.get(2)?,
        })
    })?;

    println!("\nAll records:");
    for row in rows {
        println!("  {:?}", row?);
    }

    // UPDATE
    conn.execute(
        "UPDATE records SET value = ?1 WHERE name = ?2",
        params![99.0, "alpha"],
    )?;
    println!("\nUpdated 'alpha' value to 99.0");

    // DELETE
    conn.execute("DELETE FROM records WHERE name = ?1", params!["beta"])?;
    println!("Deleted 'beta' row.");

    // Final SELECT
    let count: i64 =
        conn.query_row("SELECT COUNT(*) FROM records", [], |row| row.get(0))?;
    println!("\nRemaining rows: {count}");

    Ok(())
}
