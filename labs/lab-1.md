# Lab 1: What is SQLite and Why Use It with Rust?

In this lab you will set up a Rust project with `rusqlite`, open your first database connection, and explore how SQLite compares to client-server databases for data engineering workloads.

## Learning Objectives

By the end of this lab, you will be able to:

- Add `rusqlite` as a dependency in `Cargo.toml`
- Open and close a SQLite database from Rust
- Query the SQLite version and basic metadata
- Explain when SQLite is the right choice for a data pipeline

## Prerequisites

- Rust toolchain installed (`rustup`, `cargo`)
- A C compiler available (required by `rusqlite` for the bundled SQLite feature)

## Key Concepts

- **Embedded database**: SQLite runs in-process — no server, no network, no configuration
- **`rusqlite`**: A safe Rust wrapper around the SQLite C library
- **`Connection`**: The main entry point; represents an open database file or in-memory DB
- **Result and `?`**: Rust's idiomatic error propagation used throughout `rusqlite`'s API

## Lab Exercises

### Exercise 1: Add the Dependency

In a new Cargo project, add `rusqlite` to `Cargo.toml`:

```toml
[dependencies]
rusqlite = "0.31"
```

For a self-contained binary that bundles SQLite (no system library required):

```toml
rusqlite = { version = "0.31", features = ["bundled"] }
```

### Exercise 2: Open a Connection

Open a connection to a file-based database:

```rust
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("my.db")?;
    println!("Database opened.");
    Ok(())
}
```

Run it:

```bash
cargo run
ls -lh my.db
```

### Exercise 3: Query the SQLite Version

```rust
let version: String = conn.query_row(
    "SELECT sqlite_version()",
    [],
    |row| row.get(0),
)?;
println!("SQLite version: {version}");
```

### Exercise 4: Open an In-Memory Database

In-memory databases are destroyed when the connection closes — perfect for tests:

```rust
let conn = Connection::open_in_memory()?;
```

### Exercise 5: Study the Example

Navigate to [examples/1-sqlite-intro](../examples/1-sqlite-intro/) and run it:

```bash
cargo run -p sqlite-intro
```

Review `src/main.rs` and note how `Connection::open` and `Connection::open_in_memory` differ.

## Challenge

1. Modify the example to print the page size and journal mode pragmas:
   ```sql
   PRAGMA page_size;
   PRAGMA journal_mode;
   ```
2. Open the same database file twice in two separate `Connection` objects. What happens?
3. Write a function `fn db_version(conn: &Connection) -> Result<String>` and call it from `main`.

## Summary

In this lab, you learned how to:
- Add `rusqlite` to a Rust project
- Open file-based and in-memory SQLite databases
- Run a simple query with `query_row`
- Understand when embedded SQLite is the right tool

## Next Steps

Continue to [Lab 2: Core SQL Operations in Rust](./lab-2.md) to learn INSERT, SELECT, UPDATE, and DELETE.
