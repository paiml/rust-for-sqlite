# Lab 6: Dumping and Migrating Databases

In this lab you will generate a SQL dump of a live SQLite database from Rust, restore it into a fresh database, and apply schema migrations safely.

## Learning Objectives

By the end of this lab, you will be able to:

- Read the schema from `sqlite_schema`
- Generate `INSERT` statements for all rows programmatically
- Restore a database by replaying a SQL dump
- Add columns with `ALTER TABLE` without losing data

## Prerequisites

- Completed Lab 5

## Key Concepts

- **`sqlite_schema`**: The internal table that stores all DDL statements for the database
- **SQL dump**: A plain-text file of `CREATE TABLE` + `INSERT` statements that can recreate the database
- **`ALTER TABLE ... ADD COLUMN`**: SQLite supports adding columns but not removing or renaming them without recreating the table
- **`execute_batch`**: Runs multiple semicolon-separated SQL statements — useful for replaying a dump

## Lab Exercises

### Exercise 1: Read the Schema

```rust
let mut stmt = conn.prepare(
    "SELECT sql FROM sqlite_schema
     WHERE type='table' AND sql IS NOT NULL
     ORDER BY rootpage",
)?;
let schemas: Vec<String> = stmt
    .query_map([], |row| row.get(0))?
    .filter_map(|r| r.ok())
    .collect();

for s in &schemas {
    println!("{s};");
}
```

### Exercise 2: Generate INSERT Statements

For each table, query all rows and format values as SQL literals:

```rust
let row_value = match val {
    rusqlite::types::Value::Null        => "NULL".to_string(),
    rusqlite::types::Value::Integer(n)  => n.to_string(),
    rusqlite::types::Value::Real(f)     => f.to_string(),
    rusqlite::types::Value::Text(t)     => format!("'{}'", t.replace('\'', "''")),
    rusqlite::types::Value::Blob(_)     => "X''".to_string(),
};
```

### Exercise 3: Write the Dump to a File

```rust
use std::fs;
let dump = generate_dump(&conn)?;
fs::write("backup.sql", &dump)?;
println!("Dump written to backup.sql ({} bytes)", dump.len());
```

### Exercise 4: Restore from a Dump

```rust
let dump = fs::read_to_string("backup.sql")?;
let fresh = Connection::open_in_memory()?;
fresh.execute_batch(&dump)?;
println!("Restored successfully.");
```

### Exercise 5: Apply a Schema Migration

```rust
// Add a new column — existing rows get NULL for the new column
conn.execute_batch("ALTER TABLE users ADD COLUMN verified INTEGER DEFAULT 0;")?;
println!("Migration applied.");
```

To verify no data was lost:

```rust
let count: i64 = conn.query_row("SELECT COUNT(*) FROM users", [], |r| r.get(0))?;
println!("Rows after migration: {count}");
```

### Exercise 6: Run the Example

```bash
cargo run -p db-dump
```

## Challenge

1. Extend the dump function to also include `CREATE INDEX` statements from `sqlite_schema`.
2. Implement a `migrate_v2` function that uses a temporary table to rename a column (SQLite's multi-step rename workaround).
3. Write a test that dumps a database, restores it into a fresh connection, and asserts row counts match.

## Summary

In this lab, you learned how to:
- Read DDL from `sqlite_schema`
- Generate and write a SQL dump file
- Restore a database with `execute_batch`
- Add columns with `ALTER TABLE` while preserving data

## Next Steps

Continue to [Lab 7: CLI Architecture with clap](./lab-7.md).
