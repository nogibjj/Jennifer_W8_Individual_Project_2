use csv::ReaderBuilder;
use futures_util::StreamExt;
use reqwest;
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

pub mod extract {
    use super::*;

    pub async fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
        if let Some(parent) = Path::new(file_path).parent() {
            fs::create_dir_all(parent)?;
        }

        // Create a reusable client with custom configuration
        let client = reqwest::Client::builder()
            .pool_idle_timeout(std::time::Duration::from_secs(15))
            .build()?;

        let response = client.get(url).send().await?;
        if response.status().is_success() {
            let file = fs::File::create(file_path)?;
            let mut writer = BufWriter::new(file);
            let mut stream = response.bytes_stream();

            // Stream the download in chunks
            while let Some(chunk) = stream.next().await {
                writer.write_all(&chunk?)?;
            }
            writer.flush()?;
            println!("File successfully downloaded to {}", file_path);
        } else {
            println!(
                "Failed to retrieve the file. HTTP Status Code: {}",
                response.status()
            );
        }

        Ok(())
    }
}

pub mod transform_load {
    use super::*;

    const BATCH_SIZE: usize = 1000;

    fn parse_int(value: Option<&str>) -> i32 {
        value.and_then(|v| v.parse().ok()).unwrap_or(0)
    }

    fn parse_float(value: Option<&str>) -> f64 {
        value.and_then(|v| v.parse().ok()).unwrap_or(0.0)
    }

    pub fn load(dataset: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(dataset)?;
        let reader = BufReader::new(file);
        let mut rdr = ReaderBuilder::new()
            .flexible(true)
            .trim(csv::Trim::All)
            .from_reader(reader);

        let mut conn = Connection::open("GooseDB.db")?;

        // Begin transaction for better performance
        conn.execute_batch(
            "
            PRAGMA synchronous = OFF;
            PRAGMA journal_mode = MEMORY;
            DROP TABLE IF EXISTS GooseDB;
            CREATE TABLE GooseDB (
                name TEXT,
                year INTEGER,
                team TEXT,
                league TEXT,
                goose_eggs INTEGER,
                broken_eggs INTEGER,
                mehs INTEGER,
                league_average_gpct REAL,
                ppf REAL,
                replacement_gpct REAL,
                gwar REAL,
                key_retro TEXT
            )
        ",
        )?;

        {
            let tx = conn.transaction()?;

            // Process records in batches
            let mut batch = Vec::with_capacity(BATCH_SIZE);
            let mut stmt =
                tx.prepare("INSERT INTO GooseDB VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")?;

            for result in rdr.records() {
                let record = result?;

                batch.push((
                    record.get(0).unwrap_or("").to_string(),
                    parse_int(record.get(1)),
                    record.get(2).unwrap_or("").to_string(),
                    record.get(3).unwrap_or("").to_string(),
                    parse_int(record.get(4)),
                    parse_int(record.get(5)),
                    parse_int(record.get(6)),
                    parse_float(record.get(7)),
                    parse_float(record.get(8)),
                    parse_float(record.get(9)),
                    parse_float(record.get(10)),
                    record.get(11).unwrap_or("").to_string(),
                ));

                if batch.len() >= BATCH_SIZE {
                    for record in &batch {
                        stmt.execute(params![
                            &record.0, &record.1, &record.2, &record.3, &record.4, &record.5,
                            &record.6, &record.7, &record.8, &record.9, &record.10, &record.11
                        ])?;
                    }
                    batch.clear();
                }
            }

            // Insert remaining records
            for record in &batch {
                stmt.execute(params![
                    &record.0, &record.1, &record.2, &record.3, &record.4, &record.5, &record.6,
                    &record.7, &record.8, &record.9, &record.10, &record.11
                ])?;
            }

            // Drop the statement before committing
            drop(stmt);
            tx.commit()?;
        }

        println!("Data successfully loaded into GooseDB.db");
        Ok(())
    }
}

pub mod query {
    use super::*;

    fn get_connection() -> Result<Connection, Box<dyn Error>> {
        let conn = Connection::open("GooseDB.db")?;
        conn.execute_batch("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")?;
        Ok(conn)
    }

    pub fn db_query() -> Result<(), Box<dyn Error>> {
        let conn = get_connection()?;
        let mut stmt = conn.prepare_cached("SELECT * FROM GooseDB LIMIT 10")?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i32>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })?;

        println!("Top 10 rows of the GooseDB table:");
        for row in rows {
            println!("{:?}", row?);
        }

        Ok(())
    }

    pub fn crud_create() -> Result<(), Box<dyn Error>> {
        let mut conn = get_connection()?;
        let tx = conn.transaction()?;

        tx.execute(
            "INSERT INTO GooseDB VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                "Jennifer Li",
                2024,
                "DKU",
                "AL",
                0,
                0,
                0,
                0.0,
                0.0,
                0.0,
                0.0,
                "jennifer101"
            ],
        )?;

        tx.commit()?;
        println!("Create successfully");
        Ok(())
    }

    pub fn crud_read() -> Result<(), Box<dyn Error>> {
        let conn = get_connection()?;
        let mut stmt = conn.prepare_cached("SELECT * FROM GooseDB LIMIT 10")?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i32>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })?;

        println!("Read 10 from GooseDB table:");
        for row in rows {
            println!("{:?}", row?);
        }

        Ok(())
    }

    pub fn crud_update() -> Result<(), Box<dyn Error>> {
        let mut conn = get_connection()?;
        let tx = conn.transaction()?;

        tx.execute(
            "UPDATE GooseDB SET year = 2024 WHERE key_retro = 'luqud101'",
            [],
        )?;

        tx.commit()?;
        println!("Update successfully");
        Ok(())
    }

    pub fn crud_delete() -> Result<(), Box<dyn Error>> {
        let mut conn = get_connection()?;
        let tx = conn.transaction()?;

        tx.execute("DELETE FROM GooseDB WHERE key_retro = 'kircm101'", [])?;

        tx.commit()?;
        println!("Delete successfully");
        Ok(())
    }
}
