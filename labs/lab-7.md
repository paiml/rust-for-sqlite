# Lab 7: CLI Architecture with clap

In this lab you will design a multi-subcommand command-line tool using `clap`'s derive API, wire each subcommand to a SQLite database layer, and configure the database path via flags, environment variables, and defaults.

## Learning Objectives

By the end of this lab, you will be able to:

- Define a CLI with subcommands using `#[derive(Parser)]` and `#[derive(Subcommand)]`
- Accept flags, positional arguments, and environment variables with `#[arg(...)]`
- Dispatch subcommands in a `match` block
- Separate CLI parsing from database logic

## Prerequisites

- Completed Lab 6
- Add `clap = { version = "4", features = ["derive"] }` to `Cargo.toml`

## Key Concepts

- **`clap` derive API**: Annotated structs and enums generate the full argument parser at compile time
- **`#[command(subcommand)]`**: Marks a field as the active subcommand
- **`#[arg(env = "VAR")]`**: Reads the value from an environment variable when the flag is absent
- **Separation of concerns**: Parse args in `main`, pass the connection to domain functions

## Lab Exercises

### Exercise 1: Define the Root CLI Struct

```rust
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mydb", about = "A simple SQLite CLI")]
struct Cli {
    #[arg(long, env = "MYDB_PATH", default_value = "mydb.sqlite")]
    db: PathBuf,

    #[command(subcommand)]
    command: Commands,
}
```

### Exercise 2: Define Subcommands

```rust
use clap::Subcommand;

#[derive(Subcommand)]
enum Commands {
    /// Initialize the database schema
    Init,
    /// Insert a key-value pair
    Set { key: String, value: String },
    /// Retrieve a value by key
    Get { key: String },
}
```

### Exercise 3: Parse and Dispatch

```rust
fn main() -> rusqlite::Result<()> {
    let cli = Cli::parse();
    let conn = rusqlite::Connection::open(&cli.db)?;

    match cli.command {
        Commands::Init => init(&conn)?,
        Commands::Set { key, value } => set(&conn, &key, &value)?,
        Commands::Get { key } => get(&conn, &key)?,
    }
    Ok(())
}
```

### Exercise 4: Test the CLI

```bash
# Init schema
cargo run -p cli-architecture -- init

# Set a value
cargo run -p cli-architecture -- set greeting "Hello, world"

# Get a value
cargo run -p cli-architecture -- get greeting

# Use a custom DB path via env var
MYDB_PATH=/tmp/test.db cargo run -p cli-architecture -- init
```

### Exercise 5: View Generated Help

```bash
cargo run -p cli-architecture -- --help
cargo run -p cli-architecture -- set --help
```

### Exercise 6: Run the Example

```bash
cargo run -p cli-architecture
```

## Challenge

1. Add a `delete <key>` subcommand that removes an entry from the database.
2. Add a `list` subcommand that prints all key-value pairs formatted as `key = value`.
3. Add a `--verbose` global flag that prints the SQL query being executed before each operation.

## Summary

In this lab, you learned how to:
- Use `clap`'s derive API for zero-boilerplate CLI parsing
- Define subcommands with per-command arguments
- Read configuration from flags, env vars, and defaults
- Dispatch subcommands with a `match` block

## Next Steps

Continue to [Lab 8: Crawling the Filesystem and Persisting Metadata](./lab-8.md).
