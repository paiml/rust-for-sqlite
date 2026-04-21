use csv::ReaderBuilder;
use rusqlite::{Connection, Result, params};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Product {
    name: String,
    category: Option<String>,
    price: Option<f64>,  // Serde automatically handles empty strings as None
}

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

    for result in rdr.deserialize() {
        let product: Product = match result {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Skipping invalid row: {}", e);
                skipped += 1;
                continue;
            }
        };

        if product.name.trim().is_empty() {
            skipped += 1;
            continue;
        }

        tx.execute(
            "INSERT INTO products (name, category, price) VALUES (?1, ?2, ?3)",
            params![product.name, product.category, product.price],
        )?;
        inserted += 1;
    }

    tx.commit()?;
    println!("Inserted: {inserted}  Skipped: {skipped}");

    // Verify logic same as above...
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
