// Lesson 3.1 — CLI architecture with clap
// Demonstrates: multi-subcommand CLI with clap derive API, DB path resolution.

use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result, params};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mydb", about = "Example multi-subcommand SQLite CLI")]
struct Cli {
    /// Path to the SQLite database file
    #[arg(long, env = "MYDB_PATH", default_value = "mydb.sqlite")]
    db: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the database schema
    Init,
    /// Insert a key-value pair
    Set {
        /// Key name
        key: String,
        /// Value to store
        value: String,
    },
    /// Retrieve a value by key
    Get {
        /// Key name
        key: String,
    },
    /// List all stored key-value pairs
    List,
}

fn open_db(path: &PathBuf) -> Result<Connection> {
    let conn = Connection::open(path)?;
    Ok(conn)
}

fn init(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS kv (
            key   TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );",
    )?;
    println!("Database initialized.");
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let conn = open_db(&cli.db)?;

    match cli.command {
        Commands::Init => init(&conn)?,
        Commands::Set { key, value } => {
            conn.execute(
                "INSERT INTO kv (key, value) VALUES (?1, ?2)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value",
                params![key, value],
            )?;
            println!("Set {key} = {value}");
        }
        Commands::Get { key } => {
            let result: rusqlite::Result<String> =
                conn.query_row("SELECT value FROM kv WHERE key = ?1", params![key], |r| {
                    r.get(0)
                });
            match result {
                Ok(v) => println!("{v}"),
                Err(_) => println!("Key '{key}' not found."),
            }
        }
        Commands::List => {
            let mut stmt = conn.prepare("SELECT key, value FROM kv ORDER BY key")?;
            let pairs = stmt.query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;
            for pair in pairs {
                let (k, v) = pair?;
                println!("{k} = {v}");
            }
        }
    }

    Ok(())
}
