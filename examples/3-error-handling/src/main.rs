// Lesson 1.3 — Error handling and transactions
// Demonstrates: Result + ?, transactions for batch writes, in-memory test DB.

use rusqlite::{Connection, Result, params};

fn create_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS events (
            id        INTEGER PRIMARY KEY,
            name      TEXT    NOT NULL,
            timestamp TEXT    NOT NULL
        );",
    )?;
    Ok(())
}

/// Insert a batch of rows inside a single transaction.
/// If any insert fails the whole batch is rolled back.
fn insert_batch(conn: &mut Connection, events: &[(&str, &str)]) -> Result<()> {
    let tx = conn.transaction()?;

    for (name, ts) in events {
        tx.execute(
            "INSERT INTO events (xname, timestamp) VALUES (?1, ?2)",
            params![name, ts],
        )?;
    }

    tx.commit()?;
    println!("Batch of {} events committed.", events.len());
    Ok(())
}

fn main() -> Result<()> {
    let mut conn = Connection::open_in_memory()?;
    create_schema(&conn)?;

    let events = [
        ("start", "2024-01-01T00:00:00"),
        ("process", "2024-01-01T00:00:01"),
        ("end", "2024-01-01T00:00:02"),
    ];

    if let Err(e) = insert_batch(&mut conn, &events) {
        println!("There was an error {}", e);
    }

    let count: i64 =
        conn.query_row("SELECT COUNT(*) FROM events", [], |row| row.get(0))?;
    println!("Total events stored: {count}");

    Ok(())
}

