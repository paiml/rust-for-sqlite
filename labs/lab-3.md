# Lab 3: Error Handling and Transactions

In this lab you will learn to handle database errors with Rust's `Result` type and use transactions to batch writes for performance and consistency.

## Learning Objectives

By the end of this lab, you will be able to:

- Propagate `rusqlite::Error` with the `?` operator
- Wrap multiple inserts in a single transaction
- Roll back a transaction on failure
- Write unit tests using an in-memory SQLite database

## Prerequisites

- Completed Lab 2
- Familiarity with Rust's `Result` and `?` operator

## Key Concepts

- **`rusqlite::Error`**: The crate's unified error type; implements `std::error::Error`
- **`Transaction`**: Groups multiple statements into an atomic unit — all commit or all roll back
- **In-memory DB for tests**: `Connection::open_in_memory()` gives each test an isolated, temporary database

## Lab Exercises

### Exercise 1: Propagate Errors with `?`

Any `rusqlite` function returns `Result<T, rusqlite::Error>`. Use `?` to propagate:

```rust
use rusqlite::{Connection, Result};

fn count_rows(conn: &Connection) -> Result<i64> {
    conn.query_row("SELECT COUNT(*) FROM events", [], |r| r.get(0))
}
```

Return `Result<()>` from `main` so `?` works at the top level.

### Exercise 2: Open a Transaction

```rust
let tx = conn.unchecked_transaction()?;

tx.execute("INSERT INTO events (name) VALUES (?1)", ["login"])?;
tx.execute("INSERT INTO events (name) VALUES (?1)", ["logout"])?;

tx.commit()?; // both inserts are durable
```

If `commit()` is never called — or if the transaction is dropped — the writes are rolled back automatically.

### Exercise 3: Intentional Rollback

```rust
let tx = conn.unchecked_transaction()?;
tx.execute("INSERT INTO events (name) VALUES ('partial')", [])?;
// tx is dropped here without commit — rolled back
drop(tx);

let count: i64 = conn.query_row("SELECT COUNT(*) FROM events", [], |r| r.get(0))?;
assert_eq!(count, 0); // nothing was saved
```

### Exercise 4: Batch Inserts with a Transaction

Wrapping N inserts in one transaction is dramatically faster than N autocommit inserts:

```rust
fn insert_batch(conn: &Connection, names: &[&str]) -> Result<()> {
    let tx = conn.unchecked_transaction()?;
    for name in names {
        tx.execute("INSERT INTO events (name) VALUES (?1)", [name])?;
    }
    tx.commit()
}
```

### Exercise 5: Testing with In-Memory Databases

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_batch() {
        let conn = Connection::open_in_memory().unwrap();
        create_schema(&conn).unwrap();
        insert_batch(&conn, &["a", "b", "c"]).unwrap();

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM events", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 3);
    }
}
```

Run tests with:

```bash
cargo test -p error-handling
```

### Exercise 6: Run the Example

```bash
cargo run -p error-handling
```

## Challenge

1. Modify `insert_batch` to return `Err` if any name is empty, and verify the whole batch is rolled back.
2. Benchmark inserting 10 000 rows with and without a transaction. Use `std::time::Instant`.
3. Write a test that verifies a failed mid-batch insert leaves zero rows.

## Summary

In this lab, you learned how to:
- Use `?` to propagate `rusqlite::Error`
- Wrap writes in transactions for atomicity and performance
- Roll back a transaction by not calling `commit`
- Use in-memory databases for fast, isolated unit tests

## Next Steps

Continue to [Lab 4: Loading Data from CSV](./lab-4.md).
