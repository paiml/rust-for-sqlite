// Lesson 1.1 — What is SQLite and why use it with Rust?
// Demonstrates: opening a connection, creating a database file, and closing it.

use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    // Open (or create) a database file on disk
    //let conn = Connection::open("lesson1.db")?;

    let conn = Connection::open_in_memory()?;

    println!("Opened database successfully.");

    // SQLite stores the schema version in a pragma — a quick sanity check
    let version: String = conn.query_row(
        "SELECT sqlite_version()",
        [],
        |row| row.get(0),
    )?;
    println!("SQLite version: {version}");

    // Connection is closed automatically when `conn` is dropped
    println!("Connection closed.");
    Ok(())
}
