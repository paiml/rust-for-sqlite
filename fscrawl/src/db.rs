use csv::Writer;
use rusqlite::{params, Connection, Result};
use serde_json::json;
use std::{
    io,
    path::Path,
};
use walkdir::WalkDir;

pub fn open(path: &Path) -> Result<Connection> {
    Connection::open(path)
}

pub fn init(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS files (
            path        TEXT    PRIMARY KEY,
            size        INTEGER NOT NULL,
            extension   TEXT,
            modified_at INTEGER
        );",
    )?;
    Ok(())
}

pub fn crawl(conn: &Connection, root: &Path, min_size: Option<u64>) -> Result<usize> {
    let tx = conn.unchecked_transaction()?;
    let mut count = 0usize;

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let size = meta.len();
        if let Some(min) = min_size {
            if size < min {
                continue;
            }
        }

        let path = entry.path().to_string_lossy().to_string();
        let extension = entry
            .path()
            .extension()
            .map(|e| e.to_string_lossy().to_string());
        let modified_at = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64);

        tx.execute(
            "INSERT INTO files (path, size, extension, modified_at)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(path) DO UPDATE SET
                 size        = excluded.size,
                 modified_at = excluded.modified_at",
            params![path, size as i64, extension, modified_at],
        )?;
        count += 1;
    }

    tx.commit()?;
    Ok(count)
}

pub fn query_report(conn: &Connection, top: usize) -> Result<()> {
    println!("=== Top {top} Largest Files ===");
    let mut stmt = conn.prepare(
        "SELECT path, size FROM files ORDER BY size DESC LIMIT ?1",
    )?;
    let rows = stmt.query_map(params![top as i64], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
    })?;
    for row in rows {
        let (path, size) = row?;
        println!("  {path}  ({size} bytes)");
    }

    println!("\n=== Totals by Extension ===");
    let mut stmt = conn.prepare(
        "SELECT COALESCE(extension,'(none)'), COUNT(*), SUM(size)
         FROM files GROUP BY extension ORDER BY SUM(size) DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, i64>(2)?,
        ))
    })?;
    for row in rows {
        let (ext, cnt, total) = row?;
        println!("  .{ext}  files={cnt}  total={total} bytes");
    }

    println!("\n=== 10 Most Recently Changed Files ===");
    let mut stmt = conn.prepare(
        "SELECT path, modified_at FROM files
         WHERE modified_at IS NOT NULL
         ORDER BY modified_at DESC LIMIT 10",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
    })?;
    for row in rows {
        let (path, mtime) = row?;
        println!("  {path}  (mtime={mtime})");
    }

    Ok(())
}

pub fn export_csv(conn: &Connection, output: Option<&Path>) -> anyhow::Result<()> {
    let mut stmt =
        conn.prepare("SELECT path, size, COALESCE(extension,''), modified_at FROM files ORDER BY path")?;

    let write: Box<dyn io::Write> = match output {
        Some(p) => Box::new(std::fs::File::create(p)?),
        None => Box::new(io::stdout()),
    };
    let mut wtr = Writer::from_writer(write);
    wtr.write_record(["path", "size", "extension", "modified_at"])?;

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, Option<i64>>(3)?,
        ))
    })?;
    for row in rows {
        let (path, size, ext, mtime) = row?;
        wtr.write_record([
            path,
            size.to_string(),
            ext,
            mtime.map(|t| t.to_string()).unwrap_or_default(),
        ])?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn export_json(conn: &Connection, output: Option<&Path>) -> anyhow::Result<()> {
    let mut stmt =
        conn.prepare("SELECT path, size, extension, modified_at FROM files ORDER BY path")?;

    let records: Vec<_> = stmt
        .query_map([], |row| {
            Ok(json!({
                "path":        row.get::<_, String>(0)?,
                "size":        row.get::<_, i64>(1)?,
                "extension":   row.get::<_, Option<String>>(2)?,
                "modified_at": row.get::<_, Option<i64>>(3)?,
            }))
        })?
        .filter_map(|r| r.ok())
        .collect();

    let json_str = serde_json::to_string_pretty(&records)?;

    match output {
        Some(p) => std::fs::write(p, json_str)?,
        None => println!("{json_str}"),
    }
    Ok(())
}

pub fn dump(conn: &Connection, output: Option<&Path>) -> anyhow::Result<()> {
    let mut out = String::new();

    let mut schema_stmt = conn.prepare(
        "SELECT sql FROM sqlite_schema WHERE type='table' AND sql IS NOT NULL ORDER BY rootpage",
    )?;
    for sql in schema_stmt
        .query_map([], |row| row.get::<_, String>(0))?
        .filter_map(|r| r.ok())
    {
        out.push_str(&sql);
        out.push_str(";\n");
    }
    out.push('\n');

    let table_names: Vec<String> = conn
        .prepare("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY rootpage")?
        .query_map([], |row| row.get(0))?
        .filter_map(|r| r.ok())
        .collect();

    for table in &table_names {
        let mut row_stmt = conn.prepare(&format!("SELECT * FROM \"{table}\""))?;
        let col_count = row_stmt.column_count();

        let rows_result: Vec<Vec<String>> = row_stmt
            .query_map([], |row| {
                let mut cols = Vec::new();
                for i in 0..col_count {
                    let val: rusqlite::types::Value = row.get(i)?;
                    let s = match val {
                        rusqlite::types::Value::Null => "NULL".to_string(),
                        rusqlite::types::Value::Integer(n) => n.to_string(),
                        rusqlite::types::Value::Real(f) => f.to_string(),
                        rusqlite::types::Value::Text(t) => {
                            format!("'{}'", t.replace('\'', "''"))
                        }
                        rusqlite::types::Value::Blob(_) => "X''".to_string(),
                    };
                    cols.push(s);
                }
                Ok(cols)
            })?
            .filter_map(|r| r.ok())
            .collect();

        for cols in rows_result {
            out.push_str(&format!(
                "INSERT INTO \"{table}\" VALUES ({});\n",
                cols.join(", ")
            ));
        }
    }

    match output {
        Some(p) => std::fs::write(p, out)?,
        None => print!("{out}"),
    }
    Ok(())
}

pub fn summary(conn: &Connection) -> Result<()> {
    let total_files: i64 =
        conn.query_row("SELECT COUNT(*) FROM files", [], |r| r.get(0))?;
    let total_size: i64 =
        conn.query_row("SELECT COALESCE(SUM(size),0) FROM files", [], |r| r.get(0))?;
    let distinct_ext: i64 =
        conn.query_row("SELECT COUNT(DISTINCT extension) FROM files", [], |r| r.get(0))?;

    println!("Files indexed : {total_files}");
    println!("Total size    : {total_size} bytes");
    println!("Distinct exts : {distinct_ext}");
    Ok(())
}
