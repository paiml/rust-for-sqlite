// fscrawl — Graded Project
//
// A CLI tool that walks a directory, stores file metadata in SQLite, and
// supports the following subcommands:
//   crawl    — walk a directory and upsert file records
//   query    — report largest files, totals by extension, recent changes
//   export   — write results to CSV or JSON
//   db-dump  — export the full database as a SQL dump file

mod db;

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "fscrawl",
    about = "Filesystem crawler that stores file metadata in SQLite",
    version
)]
struct Cli {
    /// Path to the SQLite database file
    #[arg(long, env = "FSCRAWL_DB", default_value = "fscrawl.sqlite")]
    db: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Walk a directory tree and insert/upsert file records
    Crawl {
        /// Root directory to crawl
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Only include files larger than this size in bytes
        #[arg(long)]
        min_size: Option<u64>,
    },
    /// Report file statistics
    Query {
        /// Show the N largest files
        #[arg(long, default_value = "10")]
        top: usize,
    },
    /// Export results to CSV or JSON
    Export {
        /// Output format
        #[arg(long, value_enum, default_value = "csv")]
        format: ExportFormat,

        /// Output file path (defaults to stdout)
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Export the full database as a SQL dump file
    DbDump {
        /// Output file path (defaults to stdout)
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Print aggregate statistics to stdout
    Summary,
}

#[derive(ValueEnum, Clone)]
enum ExportFormat {
    Csv,
    Json,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let conn = db::open(&cli.db)?;
    db::init(&conn)?;

    match cli.command {
        Commands::Crawl { path, min_size } => {
            let n = db::crawl(&conn, &path, min_size)?;
            println!("Crawled and upserted {n} file(s) from {}.", path.display());
        }
        Commands::Query { top } => {
            db::query_report(&conn, top)?;
        }
        Commands::Export { format, output } => match format {
            ExportFormat::Csv => db::export_csv(&conn, output.as_deref())?,
            ExportFormat::Json => db::export_json(&conn, output.as_deref())?,
        },
        Commands::DbDump { output } => {
            db::dump(&conn, output.as_deref())?;
        }
        Commands::Summary => {
            db::summary(&conn)?;
        }
    }

    Ok(())
}
