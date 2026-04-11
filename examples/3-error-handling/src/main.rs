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
fn insert_batch(conn: &Connection, events: &[(&str, &str)]) -> Result<()> {
    let tx = conn.unchecked_transaction()?;

    for (name, ts) in events {
        tx.execute(
            "INSERT INTO events (name, timestamp) VALUES (?1, ?2)",
            params![name, ts],
        )?;
    }

    tx.commit()?;
    println!("Batch of {} events committed.", events.len());
    Ok(())
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    create_schema(&conn)?;

    let events = [
        ("start", "2024-01-01T00:00:00"),
        ("process", "2024-01-01T00:00:01"),
        ("end", "2024-01-01T00:00:02"),
    ];

    insert_batch(&conn, &events)?;

    let count: i64 =
        conn.query_row("SELECT COUNT(*) FROM events", [], |row| row.get(0))?;
    println!("Total events stored: {count}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().expect("in-memory DB");
        create_schema(&conn).expect("schema");
        conn
    }

    #[test]
    fn test_batch_insert() {
        let conn = setup();
        let events = [("click", "2024-06-01T10:00:00")];
        insert_batch(&conn, &events).expect("insert");

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM events", [], |row| row.get(0))
            .expect("count");
        assert_eq!(count, 1);
    }

    #[test]
    fn test_empty_batch() {
        let conn = setup();
        insert_batch(&conn, &[]).expect("empty batch ok");

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM events", [], |row| row.get(0))
            .expect("count");
        assert_eq!(count, 0);
    }
}
