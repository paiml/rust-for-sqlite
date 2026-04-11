# Rust for SQLite: Data Engineering Foundations

This repository contains hands-on examples and labs for using SQLite with Rust for data engineering tasks. A Coursera course from Pragmatic AI Labs.

## Contents

This repository has example projects in [./examples](./examples) and hands-on labs in [./labs](./labs). Make sure you have the [Rust toolchain](https://rustup.rs) installed.

This repository is *Codespaces ready* and set as a template repository. You can open it directly in a GitHub Codespace — Rust, rust-analyzer, and all extensions are pre-installed.

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new?hide_repo_select=true&ref=main)

## Labs

Complete these hands-on labs to reinforce your learning:

| Lab | Topic | Example |
|-----|-------|---------|
| [Lab 1: What is SQLite and Why Use It with Rust?](./labs/lab-1.md) | Connections, setup, SQLite vs. client-server | [examples/1-sqlite-intro](./examples/1-sqlite-intro/) |
| [Lab 2: Core SQL Operations in Rust](./labs/lab-2.md) | CREATE TABLE, INSERT, SELECT, UPDATE, DELETE | [examples/2-core-sql](./examples/2-core-sql/) |
| [Lab 3: Error Handling and Transactions](./labs/lab-3.md) | Result, ?, transactions, in-memory test DBs | [examples/3-error-handling](./examples/3-error-handling/) |
| [Lab 4: Loading Data from CSV](./labs/lab-4.md) | csv crate, bulk inserts, type coercion | [examples/4-csv-ingestion](./examples/4-csv-ingestion/) |
| [Lab 5: Loading and Exporting JSON](./labs/lab-5.md) | serde_json, JSON blobs, exporting results | [examples/5-json](./examples/5-json/) |
| [Lab 6: Dumping and Migrating Databases](./labs/lab-6.md) | SQL dump, restore, ALTER TABLE migrations | [examples/6-db-dump](./examples/6-db-dump/) |
| [Lab 7: CLI Architecture with clap](./labs/lab-7.md) | clap derive API, subcommands, env vars | [examples/7-cli-architecture](./examples/7-cli-architecture/) |
| [Lab 8: Crawling the Filesystem and Persisting Metadata](./labs/lab-8.md) | walkdir, file metadata, upserts | [examples/8-filesystem-crawl](./examples/8-filesystem-crawl/) |
| [Lab 9: Querying, Reporting, and Exporting Results](./labs/lab-9.md) | Aggregate queries, CSV/JSON export | [examples/9-query-export](./examples/9-query-export/) |

## Course Outline

### Module 1: SQLite Foundations

#### Lesson 1.1 — What is SQLite and why use it with Rust?
- [SQLite connections in Rust](./examples/1-sqlite-intro/)
- SQLite vs. client-server databases
- The rusqlite crate: setup and first connection

#### Lesson 1.2 — Core SQL operations in Rust
- [CREATE TABLE, INSERT, SELECT, UPDATE, DELETE](./examples/2-core-sql/)
- Schema design for data pipelines
- Parameterized queries with rusqlite

#### Lesson 1.3 — Error handling and transactions
- [Transactions and in-memory test databases](./examples/3-error-handling/)
- Mapping SQLite errors to Rust's Result
- Batching writes for performance and consistency

### Module 2: Data Ingestion and Export

#### Lesson 2.1 — Loading data from CSV
- [Bulk-loading CSV rows into SQLite](./examples/4-csv-ingestion/)
- Parsing CSV with the csv crate
- Handling missing values and schema mismatches

#### Lesson 2.2 — Loading and exporting JSON
- [serde_json ingestion and export](./examples/5-json/)
- Deserializing JSON into Rust structs
- Storing JSON blobs and exporting query results

#### Lesson 2.3 — Dumping and migrating databases
- [SQL dump generation and schema migrations](./examples/6-db-dump/)
- Generating and restoring SQL dump files
- Adding columns and evolving tables safely

### Module 3: Building the CLI Project

#### Lesson 3.1 — CLI architecture with clap
- [Multi-subcommand CLI with clap](./examples/7-cli-architecture/)
- clap derive API and subcommand dispatch
- DB path via flags, env vars, and defaults

#### Lesson 3.2 — Crawling the filesystem and persisting metadata
- [walkdir and file metadata upserts](./examples/8-filesystem-crawl/)
- Walking directory trees and reading metadata
- Incremental crawls with upserts

#### Lesson 3.3 — Querying, reporting, and exporting results
- [Aggregate queries and CSV/JSON export](./examples/9-query-export/)
- Largest files, totals by extension, recent changes
- Exporting results from the CLI

## Graded Project: fscrawl

Build **fscrawl** — a Rust CLI tool that walks a directory, stores file metadata in SQLite, and supports these subcommands:

- `crawl` — walk a directory tree and insert/upsert file records (path, size, extension, modified timestamp)
- `query` — report largest files, totals by extension, and recently changed files
- `export` — write results to CSV or JSON
- `db-dump` — export the full database as a SQL dump file
- `summary` — print aggregate statistics to stdout

A starter implementation is in [fscrawl/](./fscrawl/).

```bash
# Build
cargo build -p fscrawl

# Crawl the current directory
cargo run -p fscrawl -- crawl .

# Report stats
cargo run -p fscrawl -- query --top 5

# Export to JSON
cargo run -p fscrawl -- export --format json --output results.json

# Dump the database
cargo run -p fscrawl -- db-dump --output backup.sql

# Aggregate summary
cargo run -p fscrawl -- summary
```

## Local Setup

1. Install the Rust toolchain:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone this repository:
   ```bash
   git clone https://github.com/alfredodeza/rust-for-sqlite.git
   cd rust-for-sqlite
   ```

3. Build the entire workspace:
   ```bash
   cargo build --workspace
   ```

4. Run an example:
   ```bash
   cargo run -p sqlite-intro
   ```

5. Run tests:
   ```bash
   cargo test --workspace
   ```

## Key Crates

| Crate | Purpose |
|---|---|
| [rusqlite](https://crates.io/crates/rusqlite) | SQLite bindings for Rust |
| [clap](https://crates.io/crates/clap) | CLI argument parsing |
| [serde](https://crates.io/crates/serde) / [serde_json](https://crates.io/crates/serde_json) | JSON serialization and deserialization |
| [csv](https://crates.io/crates/csv) | CSV reading and writing |
| [walkdir](https://crates.io/crates/walkdir) | Recursive directory traversal |
| [anyhow](https://crates.io/crates/anyhow) | Ergonomic error handling |

## Resources

- [rusqlite documentation](https://docs.rs/rusqlite)
- [SQLite documentation](https://www.sqlite.org/docs.html)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [clap documentation](https://docs.rs/clap)

**Coursera Courses**

- [Rust for Data Engineering Specialization](https://www.coursera.org/specializations/rust-for-data-engineering)
- [MLOps Machine Learning Operations Specialization](https://www.coursera.org/specializations/mlops-machine-learning-duke)
- [Linux and Bash for Data Engineering](https://www.coursera.org/learn/linux-and-bash-for-data-engineering-duke)
