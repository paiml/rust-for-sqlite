# Lab 9: Querying, Reporting, and Exporting Results

In this final module lab you will add reporting queries to the file metadata database and implement CSV and JSON export subcommands.

## Learning Objectives

By the end of this lab, you will be able to:

- Write aggregate SQL queries (largest files, totals by extension, recent changes)
- Export query results to CSV with the `csv` crate
- Export query results to JSON with `serde_json`
- Wire export logic to CLI subcommands

## Prerequisites

- Completed Labs 7 and 8
- Add `csv = "1"`, `serde = { version = "1", features = ["derive"] }`, `serde_json = "1"` to `Cargo.toml`

## Key Concepts

- **Aggregate queries**: `COUNT`, `SUM`, `MAX` with `GROUP BY` and `ORDER BY`
- **`csv::Writer`**: Streams rows to any `impl Write` (file, stdout, buffer)
- **`serde_json::json!`**: Build JSON objects dynamically from query columns
- **`Option<PathBuf>`**: Write to a file when a path is given; fall back to stdout

## Lab Exercises

### Exercise 1: Largest Files Query

```rust
let mut stmt = conn.prepare(
    "SELECT path, size FROM files ORDER BY size DESC LIMIT ?1",
)?;
let rows = stmt.query_map(params![10i64], |row| {
    Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
})?;
for row in rows {
    let (path, size) = row?;
    println!("{path}  ({size} bytes)");
}
```

### Exercise 2: Totals by Extension

```rust
let mut stmt = conn.prepare(
    "SELECT COALESCE(extension,'(none)'), COUNT(*), SUM(size)
     FROM files GROUP BY extension ORDER BY SUM(size) DESC",
)?;
```

### Exercise 3: Export to CSV

```rust
use csv::Writer;
use std::io;

let mut wtr = Writer::from_writer(io::stdout());
wtr.write_record(["path", "size", "extension"])?;

let mut stmt = conn.prepare("SELECT path, size, COALESCE(extension,'') FROM files")?;
for row in stmt.query_map([], |row| {
    Ok((row.get::<_,String>(0)?, row.get::<_,i64>(1)?, row.get::<_,String>(2)?))
})? {
    let (path, size, ext) = row?;
    wtr.write_record([path, size.to_string(), ext])?;
}
wtr.flush()?;
```

To write to a file instead of stdout:

```rust
let wtr = Writer::from_path("output.csv")?;
```

### Exercise 4: Export to JSON

```rust
use serde_json::json;

let mut stmt = conn.prepare("SELECT path, size, extension FROM files")?;
let records: Vec<_> = stmt
    .query_map([], |row| {
        Ok(json!({
            "path":      row.get::<_,String>(0)?,
            "size":      row.get::<_,i64>(1)?,
            "extension": row.get::<_,Option<String>>(2)?,
        }))
    })?
    .filter_map(|r| r.ok())
    .collect();

println!("{}", serde_json::to_string_pretty(&records)?);
```

### Exercise 5: Wire to a CLI Subcommand

Add an `export` subcommand to the CLI from Lab 7:

```rust
Export {
    #[arg(long, value_enum, default_value = "csv")]
    format: ExportFormat,

    #[arg(long)]
    output: Option<PathBuf>,
},
```

### Exercise 6: Run the Example

```bash
cargo run -p query-export
```

## Challenge

1. Add a `--since <unix-timestamp>` filter to the `query` subcommand to show only files modified after the given time.
2. Implement a `summary` subcommand that prints total file count, total size, and number of distinct extensions in one pass.
3. Add a `--format ndjson` option to the `export` subcommand that writes one JSON object per line.

## Summary

In this lab, you learned how to:
- Write aggregate queries with `COUNT`, `SUM`, `GROUP BY`
- Stream CSV output with `csv::Writer`
- Build and pretty-print JSON from query results
- Connect export logic to CLI subcommands

## Next Steps

You're ready for the **Graded Project: fscrawl**. See the project description in the [README](../README.md#graded-project-fscrawl) and the starter code in [fscrawl/](../fscrawl/).
