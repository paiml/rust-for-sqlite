// Lesson 2.2 — Loading and exporting JSON
// Demonstrates: serde_json deserialization, inserting records, exporting to JSON.

use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

fn default_unit() -> String {
    "celsius".to_string()
}

#[derive(Debug, Deserialize, Serialize)]
struct Sensor {
    device_id: String,
    temperature: f64,
    humidity: Option<f64>,
    timestamp: String,
    // missing key → empty string ""
    #[serde(default)]
    location: String,
    // missing key → "celsius" via function
    #[serde(default = "default_unit")]
    unit: String,
}

fn main() -> Result<()> {
    //let conn = Connection::open_in_memory()?;
    let conn = Connection::open("sensors.db")?;

    conn.execute_batch(
        "CREATE TABLE sensor_readings (
            id          INTEGER PRIMARY KEY,
            device_id   TEXT    NOT NULL,
            temperature REAL    NOT NULL,
            humidity    REAL,
            timestamp   TEXT    NOT NULL,
            raw_json    TEXT
        );",
    )?;

    let path = std::env::args().nth(1).unwrap_or_else(|| "sensors.json".to_string());
    let json_input = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read {path}: {e}"));

    let readings: Vec<Sensor> =
        serde_json::from_str(&json_input).expect("valid JSON");

    let tx = conn.unchecked_transaction()?;
    for r in &readings {
        let raw = serde_json::to_string(r).ok();
        tx.execute(
            "INSERT INTO sensor_readings (device_id, temperature, humidity, timestamp, raw_json)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![r.device_id, r.temperature, r.humidity, r.timestamp, raw],
        )?;
    }
    tx.commit()?;
    println!("Inserted {} readings.", readings.len());

    // Export query results as JSON
    let mut stmt = conn.prepare(
        "SELECT device_id, temperature, humidity, timestamp FROM sensor_readings ORDER BY timestamp",
    )?;

    let results: Vec<Value> = stmt
        .query_map([], |row| {
            let device_id: String = row.get(0)?;
            let temperature: f64 = row.get(1)?;
            let humidity: Option<f64> = row.get(2)?;
            let timestamp: String = row.get(3)?;
            Ok((device_id, temperature, humidity, timestamp))
        })?
        .filter_map(|r| r.ok())
        .map(|(device_id, temperature, humidity, timestamp)| {
            serde_json::json!({
                "device_id": device_id,
                "temperature": temperature,
                "humidity": humidity,
                "timestamp": timestamp,
            })
        })
        .collect();

    println!("\nExported JSON (pretty):");
    println!("{}", serde_json::to_string_pretty(&results).unwrap());

    Ok(())
}
