# Lab 2: Core SQL Operations in Rust

In this lab you will create tables, insert rows with parameterized queries, and read, update, and delete records using `rusqlite`.

## Learning Objectives

By the end of this lab, you will be able to:

- Create tables and design a simple schema
- Insert rows safely with parameterized queries
- Query rows and map them to Rust structs
- Update and delete records

## Prerequisites

- Completed Lab 1
- `rusqlite` dependency in `Cargo.toml`

## Key Concepts

- **`execute`**: Run a statement that does not return rows (INSERT, UPDATE, DELETE, DDL)
- **`query_row`**: Fetch exactly one row — errors if zero or more than one row found
- **`prepare` + `query_map`**: Iterate over multiple rows and map each to a Rust type
- **Parameterized queries (`?1`, `?2`, …)**: Prevent SQL injection; values are bound at runtime with the `params!` macro

## Lab Exercises

### Exercise 1: Create a Table

```rust
conn.execute_batch(
    "CREATE TABLE IF NOT EXISTS products (
        id    INTEGER PRIMARY KEY,
        name  TEXT    NOT NULL,
        price REAL
    );",
)?;
```

`execute_batch` runs one or more semicolon-separated statements — useful for DDL.

### Exercise 2: Insert Rows with Parameters

```rust
use rusqlite::params;

conn.execute(
    "INSERT INTO products (name, price) VALUES (?1, ?2)",
    params!["Widget", 9.99],
)?;
```

Never interpolate user-supplied strings directly into SQL. Always use `params!`.

### Exercise 3: Select Rows into a Struct

```rust
#[derive(Debug)]
struct Product { id: i64, name: String, price: Option<f64> }

let mut stmt = conn.prepare("SELECT id, name, price FROM products")?;
let products = stmt.query_map([], |row| {
    Ok(Product {
        id:    row.get(0)?,
        name:  row.get(1)?,
        price: row.get(2)?,
    })
})?;

for p in products {
    println!("{:?}", p?);
}
```

### Exercise 4: Update a Record

```rust
let rows_changed = conn.execute(
    "UPDATE products SET price = ?1 WHERE name = ?2",
    params![19.99, "Widget"],
)?;
println!("Updated {rows_changed} row(s).");
```

### Exercise 5: Delete a Record

```rust
conn.execute("DELETE FROM products WHERE id = ?1", params![1])?;
```

### Exercise 6: Run the Example

```bash
cargo run -p core-sql
```

## Challenge

1. Add a `quantity INTEGER NOT NULL DEFAULT 0` column to the table and insert rows with explicit quantities.
2. Write a query that returns the total inventory value: `SUM(price * quantity)`.
3. Implement a `find_by_name(conn: &Connection, name: &str) -> Result<Option<Product>>` function.

## Summary

In this lab, you learned how to:
- Create tables with `execute_batch`
- Use `params!` for safe parameterized queries
- Map query results to Rust structs with `query_map`
- Perform UPDATE and DELETE operations

## Next Steps

Continue to [Lab 3: Error Handling and Transactions](./lab-3.md).
