// Lesson 3.2 — Crawling the filesystem and persisting metadata
// Demonstrates: walkdir, reading file metadata, bulk upsert into SQLite.

use rusqlite::{Connection, Result, params};
use walkdir::WalkDir;
use std::path::Path;

fn create_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS files (
            path        TEXT PRIMARY KEY,
            size        INTEGER NOT NULL,
            extension   TEXT,
            modified_at TEXT
        );",
    )?;
    Ok(())
}

fn crawl(conn: &Connection, root: &Path) -> Result<usize> {
    let tx = conn.unchecked_transaction()?;
    let mut count = 0usize;

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path().to_string_lossy().to_string();
        let meta = entry.metadata().ok();
        let size = meta.as_ref().map(|m| m.len() as i64).unwrap_or(0);
        let extension = entry
            .path()
            .extension()
            .map(|e| e.to_string_lossy().to_string());
        let modified_at = meta
            .and_then(|m| m.modified().ok())
            .and_then(|t| {
                t.duration_since(std::time::UNIX_EPOCH)
                    .ok()
                    .map(|d| d.as_secs().to_string())
            });

        // UPSERT: update size/modified if the file changed
        tx.execute(
            "INSERT INTO files (path, size, extension, modified_at)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(path) DO UPDATE SET
                 size        = excluded.size,
                 modified_at = excluded.modified_at",
            params![path, size, extension, modified_at],
        )?;
        count += 1;
    }

    tx.commit()?;
    Ok(count)
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    create_schema(&conn)?;

    // Crawl the current directory as a demo
    let root = std::env::current_dir().unwrap();
    println!("Crawling: {}", root.display());

    let n = crawl(&conn, &root)?;
    println!("Upserted {n} file(s).");

    // Quick summary by extension
    let mut stmt = conn.prepare(
        "SELECT COALESCE(extension, '(none)'), COUNT(*), SUM(size)
         FROM files GROUP BY extension ORDER BY COUNT(*) DESC LIMIT 10",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?, row.get::<_, i64>(2)?))
    })?;

    println!("\nTop extensions:");
    println!("{:<15} {:>6} {:>12}", "ext", "count", "total bytes");
    for row in rows {
        let (ext, cnt, bytes) = row?;
        println!("{:<15} {:>6} {:>12}", ext, cnt, bytes);
    }

    Ok(())
}
