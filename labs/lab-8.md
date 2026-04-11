# Lab 8: Crawling the Filesystem and Persisting Metadata

In this lab you will walk a directory tree with `walkdir`, read file metadata, and store the results in SQLite using upserts to support incremental crawls.

## Learning Objectives

By the end of this lab, you will be able to:

- Traverse a directory tree with `walkdir::WalkDir`
- Read file size, extension, and modification timestamp from `std::fs::Metadata`
- Design a `files` table that uniquely identifies each file by path
- Use `INSERT … ON CONFLICT … DO UPDATE` for incremental upserts

## Prerequisites

- Completed Lab 7
- Add `walkdir = "2"` to `Cargo.toml`

## Key Concepts

- **`WalkDir`**: Recursively yields every entry under a root path, handles symlinks and permissions gracefully
- **`DirEntry::metadata()`**: Returns size, timestamps, and file type without a second syscall
- **Upsert (`INSERT OR REPLACE` / `ON CONFLICT … DO UPDATE`)**: Insert a new row or update in place if the path already exists
- **`modified()`**: Returns a `SystemTime` — convert to Unix seconds for easy storage

## Lab Exercises

### Exercise 1: Walk a Directory

```rust
use walkdir::WalkDir;

for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
    if entry.file_type().is_file() {
        println!("{}", entry.path().display());
    }
}
```

### Exercise 2: Read File Metadata

```rust
let meta = entry.metadata()?;
let size = meta.len();                          // bytes
let ext  = entry.path().extension()
    .map(|e| e.to_string_lossy().to_string());
let mtime = meta.modified()
    .ok()
    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
    .map(|d| d.as_secs() as i64);
```

### Exercise 3: Design the Files Table

```sql
CREATE TABLE IF NOT EXISTS files (
    path        TEXT    PRIMARY KEY,
    size        INTEGER NOT NULL,
    extension   TEXT,
    modified_at INTEGER
);
```

Using `path` as the primary key means each file appears exactly once regardless of how many times you crawl.

### Exercise 4: Upsert File Records

```rust
tx.execute(
    "INSERT INTO files (path, size, extension, modified_at)
     VALUES (?1, ?2, ?3, ?4)
     ON CONFLICT(path) DO UPDATE SET
         size        = excluded.size,
         modified_at = excluded.modified_at",
    params![path, size as i64, ext, mtime],
)?;
```

### Exercise 5: Wrap in a Transaction

```rust
let tx = conn.unchecked_transaction()?;
// ... all upserts ...
tx.commit()?;
println!("Crawl complete.");
```

### Exercise 6: Run the Example

```bash
cargo run -p filesystem-crawl
```

## Challenge

1. Add a `--min-size <bytes>` flag (using `clap`) to skip files smaller than the given threshold.
2. Track the total number of new vs. updated rows in the crawl (hint: use `changes()` on the connection).
3. Write a test that creates a temporary directory with known files, crawls it, and asserts the DB contains the expected rows.

## Summary

In this lab, you learned how to:
- Use `WalkDir` to recursively list files
- Extract size, extension, and modification time from `Metadata`
- Store file metadata with a path-keyed upsert
- Batch all writes in a single transaction for performance

## Next Steps

Continue to [Lab 9: Querying, Reporting, and Exporting Results](./lab-9.md).
