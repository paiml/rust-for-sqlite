# Lab 4: Loading Data from CSV

In this lab you will parse CSV files with the `csv` crate and bulk-load rows into SQLite, handling missing values and type coercion along the way.

## Learning Objectives

By the end of this lab, you will be able to:

- Read CSV files with the `csv` crate
- Handle missing and malformed values gracefully
- Bulk-insert CSV rows into SQLite inside a transaction
- Deal with schema mismatches between the CSV and the table

## Prerequisites

- Completed Lab 3
- Add `csv = "1"` to your `Cargo.toml`

## Key Concepts

- **`csv::Reader`**: Iterates over records from a file or byte slice
- **`StringRecord`**: A single CSV row as a collection of string fields
- **Type coercion**: Parsing strings into `f64`, `i64`, etc., and deciding what to do on failure
- **`Option<T>` for nullable columns**: Use `Option` to map missing CSV values to SQL `NULL`

## Lab Exercises

### Exercise 1: Parse a CSV String

```rust
use csv::ReaderBuilder;

let data = "name,price\nWidget,9.99\nGadget,";

let mut rdr = ReaderBuilder::new()
    .has_headers(true)
    .from_reader(data.as_bytes());

for result in rdr.records() {
    let record = result.expect("CSV record");
    println!("{:?}", record);
}
```

### Exercise 2: Coerce Types and Handle Missing Values

```rust
let price: Option<f64> = record
    .get(1)
    .filter(|s| !s.is_empty())   // empty string → None
    .and_then(|s| s.parse().ok()); // parse failure → None
```

### Exercise 3: Bulk Insert with a Transaction

```rust
let tx = conn.unchecked_transaction()?;

for result in rdr.records() {
    let record = result?;
    let name = record.get(0).unwrap_or("").trim();
    if name.is_empty() { continue; }

    let price: Option<f64> = record.get(1)
        .filter(|s| !s.is_empty())
        .and_then(|s| s.parse().ok());

    tx.execute(
        "INSERT INTO products (name, price) VALUES (?1, ?2)",
        params![name, price],
    )?;
}

tx.commit()?;
```

### Exercise 4: Read from a Real File

```rust
let mut rdr = csv::Reader::from_path("data/products.csv")?;
```

Create a sample `data/products.csv` file and load it into your database.

### Exercise 5: Run the Example

```bash
cargo run -p csv-ingestion
```

## Challenge

1. Add a column `category TEXT` to the table and parse it from the CSV. Insert `NULL` if the column is absent in the file.
2. Count and report skipped rows (empty name, unparseable price) without panicking.
3. Write a test that loads a CSV string with intentional bad rows and verifies only valid rows land in the DB.

## Summary

In this lab, you learned how to:
- Use `csv::ReaderBuilder` to parse CSV data
- Coerce string fields to typed values with `Option`
- Use transactions for efficient bulk inserts
- Skip or log invalid rows without crashing

## Next Steps

Continue to [Lab 5: Loading and Exporting JSON](./lab-5.md).
