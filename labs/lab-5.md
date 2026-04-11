# Lab 5: Loading and Exporting JSON

In this lab you will use `serde_json` to deserialize JSON documents into Rust structs, insert them into SQLite, and serialize query results back to JSON.

## Learning Objectives

By the end of this lab, you will be able to:

- Derive `Deserialize` and `Serialize` on Rust structs
- Parse JSON arrays and objects with `serde_json`
- Store JSON-sourced records in SQLite
- Export query results as pretty-printed or streaming JSON

## Prerequisites

- Completed Lab 4
- Add to `Cargo.toml`:
  ```toml
  serde = { version = "1", features = ["derive"] }
  serde_json = "1"
  ```

## Key Concepts

- **`serde::Deserialize`**: Derive macro that lets `serde_json` fill a struct from JSON
- **`serde::Serialize`**: Derive macro that lets `serde_json` convert a struct to JSON
- **`serde_json::Value`**: Dynamically typed JSON value — useful when the schema is unknown
- **`serde_json::json!`**: Macro for building JSON values inline

## Lab Exercises

### Exercise 1: Deserialize JSON into a Struct

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Event {
    id: String,
    kind: String,
    value: f64,
    tags: Option<Vec<String>>,
}

let json = r#"{"id":"e1","kind":"click","value":1.0}"#;
let event: Event = serde_json::from_str(json).expect("valid JSON");
println!("{event:?}");
```

### Exercise 2: Deserialize a JSON Array

```rust
let json = r#"[
    {"id":"e1","kind":"click","value":1.0},
    {"id":"e2","kind":"view","value":3.0}
]"#;
let events: Vec<Event> = serde_json::from_str(json).expect("array");
```

### Exercise 3: Insert Deserialized Records

```rust
let tx = conn.unchecked_transaction()?;
for e in &events {
    tx.execute(
        "INSERT INTO events (id, kind, value) VALUES (?1, ?2, ?3)",
        params![e.id, e.kind, e.value],
    )?;
}
tx.commit()?;
```

### Exercise 4: Store a Raw JSON Blob

SQLite can store JSON as `TEXT`. This is useful for schemaless attributes:

```rust
let raw = serde_json::to_string(&event).unwrap();
conn.execute(
    "INSERT INTO events_raw (id, payload) VALUES (?1, ?2)",
    params![event.id, raw],
)?;
```

### Exercise 5: Export Query Results as JSON

```rust
use serde_json::json;

let mut stmt = conn.prepare("SELECT id, kind, value FROM events ORDER BY id")?;
let results: Vec<_> = stmt
    .query_map([], |row| {
        Ok(json!({
            "id":    row.get::<_, String>(0)?,
            "kind":  row.get::<_, String>(1)?,
            "value": row.get::<_, f64>(2)?,
        }))
    })?
    .filter_map(|r| r.ok())
    .collect();

println!("{}", serde_json::to_string_pretty(&results).unwrap());
```

### Exercise 6: Run the Example

```bash
cargo run -p json-ingestion
```

## Challenge

1. Add an optional `metadata` column that stores an arbitrary JSON blob for each sensor reading.
2. Write a query that uses SQLite's `json_extract()` function to filter rows by a field inside the stored JSON blob.
3. Implement a streaming JSON export that writes one record per line (newline-delimited JSON / NDJSON) to avoid building the full array in memory.

## Summary

In this lab, you learned how to:
- Use `serde` derive macros for JSON ↔ struct conversion
- Deserialize JSON arrays into `Vec<T>`
- Insert records from JSON into SQLite
- Export query results using `serde_json::json!` and `to_string_pretty`

## Next Steps

Continue to [Lab 6: Dumping and Migrating Databases](./lab-6.md).
