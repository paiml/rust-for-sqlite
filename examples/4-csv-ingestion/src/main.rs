// Lesson 2.1 — Loading data from CSV
// Demonstrates: parsing CSV with the csv crate and bulk-inserting into SQLite.

use csv::ReaderBuilder;
use rusqlite::{Connection, Result, params};

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute_batch(
        "CREATE TABLE products (
            id       INTEGER PRIMARY KEY,
            name     TEXT    NOT NULL,
            category TEXT,
            price    REAL
        );",
    )?;

    // Sample CSV data (could be read from a file with ReaderBuilder::from_path)
    let csv_data = "\
name,category,price
Widget A,hardware,9.99
Widget B,hardware,
Gadget X,software,49.00
Gadget Y,software,0
";

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_data.as_bytes());

    let tx = conn.unchecked_transaction()?;
    let mut inserted = 0usize;
    let mut skipped = 0usize;

    for result in rdr.records() {
        let record = result.map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

        let name = record.get(0).unwrap_or("").trim();
        let category = record.get(1).filter(|s| !s.is_empty());
        // Coerce price: empty string or "0" both become NULL
        let price: Option<f64> = record
            .get(2)
            .filter(|s| !s.is_empty())
            .and_then(|s| s.parse().ok())
            .filter(|&v: &f64| v > 0.0);

        if name.is_empty() {
            skipped += 1;
            continue;
        }

        tx.execute(
            "INSERT INTO products (name, category, price) VALUES (?1, ?2, ?3)",
            params![name, category, price],
        )?;
        inserted += 1;
    }

    tx.commit()?;
    println!("Inserted: {inserted}  Skipped: {skipped}");

    // Verify
    let mut stmt = conn.prepare("SELECT name, category, price FROM products")?;
    let rows = stmt.query_map([], |row| {
        let name: String = row.get(0)?;
        let cat: Option<String> = row.get(1)?;
        let price: Option<f64> = row.get(2)?;
        Ok((name, cat, price))
    })?;

    println!("\nProducts in DB:");
    for row in rows {
        println!("  {:?}", row?);
    }

    Ok(())
}
