// Lesson 3.3 — Querying, reporting, and exporting results
// Demonstrates: aggregate queries, exporting to CSV and JSON.

use csv::Writer;
use rusqlite::{Connection, Result, params};
use serde_json::json;

fn setup(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE files (
            path        TEXT PRIMARY KEY,
            size        INTEGER NOT NULL,
            extension   TEXT,
            modified_at INTEGER
        );",
    )?;
    let rows: &[(&str, i64, &str, i64)] = &[
        ("/data/report.csv",  102_400, "csv",  1_700_000_000),
        ("/data/archive.zip", 5_242_880, "zip", 1_700_000_001),
        ("/src/main.rs",       4_096,   "rs",  1_700_000_002),
        ("/src/lib.rs",        2_048,   "rs",  1_700_000_003),
        ("/data/notes.txt",      512,   "txt", 1_700_000_004),
    ];
    let tx = conn.unchecked_transaction()?;
    for (path, size, ext, mtime) in rows {
        tx.execute(
            "INSERT INTO files (path, size, extension, modified_at) VALUES (?1,?2,?3,?4)",
            params![path, size, ext, mtime],
        )?;
    }
    tx.commit()?;
    Ok(())
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;

    // --- Largest files ---
    println!("=== Top 3 Largest Files ===");
    let mut stmt =
        conn.prepare("SELECT path, size FROM files ORDER BY size DESC LIMIT 3")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
    })?;
    for row in rows {
        let (path, size) = row?;
        println!("  {path}  ({size} bytes)");
    }

    // --- Totals by extension ---
    println!("\n=== Totals by Extension ===");
    let mut stmt = conn.prepare(
        "SELECT COALESCE(extension,'(none)'), COUNT(*), SUM(size)
         FROM files GROUP BY extension ORDER BY SUM(size) DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?, row.get::<_, i64>(2)?))
    })?;
    for row in rows {
        let (ext, cnt, total) = row?;
        println!("  .{ext}  files={cnt}  total={total} bytes");
    }

    // --- Export to CSV ---
    println!("\n=== CSV Export ===");
    let mut wtr = Writer::from_writer(std::io::stdout());
    wtr.write_record(["path", "size", "extension"]).unwrap();

    let mut stmt = conn.prepare("SELECT path, size, COALESCE(extension,'') FROM files ORDER BY path")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;
    for row in rows {
        let (path, size, ext) = row?;
        wtr.write_record([path, size.to_string(), ext]).unwrap();
    }
    wtr.flush().unwrap();

    // --- Export to JSON ---
    println!("\n=== JSON Export ===");
    let mut stmt = conn.prepare("SELECT path, size, extension FROM files ORDER BY size DESC")?;
    let records: Vec<_> = stmt
        .query_map([], |row| {
            Ok(json!({
                "path": row.get::<_, String>(0)?,
                "size": row.get::<_, i64>(1)?,
                "extension": row.get::<_, Option<String>>(2)?,
            }))
        })?
        .filter_map(|r| r.ok())
        .collect();

    println!("{}", serde_json::to_string_pretty(&records).unwrap());

    Ok(())
}
