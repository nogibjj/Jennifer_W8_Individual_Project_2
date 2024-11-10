use csv::Writer;
use jemalloc_ctl::{epoch, stats};
use std::error::Error;
use std::fs;
use std::time::Instant;
use tokio;
use Rust_SQL::{extract, query, transform_load};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[derive(Debug)]
struct PerformanceMetrics {
    operation: String,
    execution_time: f64,
    memory_used: f64,
}

fn get_memory_usage() -> f64 {
    epoch::advance().unwrap();

    // Get allocated memory in bytes
    let allocated = stats::allocated::read().unwrap() as f64;

    // Convert bytes to KB (1 KB = 1,024 bytes)
    allocated / 1024.0 // Changed from 1_048_576.0 (MB) to 1024.0 (KB)
}

async fn measure_performance<F, T>(
    operation: &str,
    func: F,
) -> Result<PerformanceMetrics, Box<dyn Error>>
where
    F: FnOnce() -> Result<T, Box<dyn Error>>,
{
    // Get initial memory
    let start_memory = get_memory_usage();

    // Measure time
    let start = Instant::now();
    let result = func()?;
    let duration = start.elapsed().as_secs_f64();

    // Force cleanup and wait a bit
    drop(result);
    std::thread::sleep(std::time::Duration::from_millis(10));

    // Get final memory
    let end_memory = get_memory_usage();

    // Calculate memory difference
    let memory_used = (end_memory - start_memory).max(0.0);

    Ok(PerformanceMetrics {
        operation: operation.to_string(),
        execution_time: duration,
        memory_used,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Clean up old performance files
    let current_dir = std::env::current_dir()?;
    let parent_dir = current_dir
        .parent()
        .ok_or("Failed to get parent directory")?;

    // Define paths
    let rust_csv_path = parent_dir.join("rust_benchmarks.csv");
    let python_csv_path = parent_dir.join("python_benchmarks.csv");
    let comparison_csv_path = parent_dir.join("performance_comparison.csv");
    let comparison_png_path = parent_dir.join("performance_comparison.png");

    // Remove old files if they exist
    for path in &[
        &rust_csv_path,
        &python_csv_path,
        &comparison_csv_path,
        &comparison_png_path,
    ] {
        if path.exists() {
            println!("Removing old file: {:?}", path);
            fs::remove_file(path)?;
        }
    }

    println!("Starting performance measurements...");
    let mut metrics = Vec::new();

    // Define operations
    let operations = vec![
        (
            "Extract",
            Box::new(|| {
                futures::executor::block_on(extract::extract(
                "https://github.com/fivethirtyeight/data/raw/refs/heads/master/goose/goose_rawdata.csv",
                "data/goose_rawdata.csv",
            ))
            }) as Box<dyn FnOnce() -> Result<(), Box<dyn Error>>>,
        ),
        (
            "Transform/Load",
            Box::new(|| transform_load::load("data/goose_rawdata.csv")),
        ),
        ("Query", Box::new(|| query::db_query())),
        ("CRUD-Create", Box::new(|| query::crud_create())),
        ("CRUD-Read", Box::new(|| query::crud_read())),
        ("CRUD-Update", Box::new(|| query::crud_update())),
        ("CRUD-Delete", Box::new(|| query::crud_delete())),
    ];

    // Run measurements
    for (op_name, op_func) in operations {
        println!("Measuring {}...", op_name);

        // Wait a bit for memory to settle
        std::thread::sleep(std::time::Duration::from_millis(100));

        let op_metrics = measure_performance(op_name, op_func).await?;
        println!(
            "{}: Time = {:.3}s, Memory = {:.2}MB",
            op_metrics.operation, op_metrics.execution_time, op_metrics.memory_used
        );
        metrics.push(op_metrics);
    }

    // Write new results to CSV
    let mut wtr = Writer::from_path(&rust_csv_path)?;
    wtr.write_record(&["operation", "language", "execution_time", "memory_used"])?;

    let mut total_time = 0.0;
    let mut total_memory = 0.0;

    for metric in metrics {
        total_time += metric.execution_time;
        total_memory += metric.memory_used;

        wtr.write_record(&[
            &metric.operation,
            "Rust",
            &format!("{:.3}", metric.execution_time),
            &format!("{:.2}", metric.memory_used), // Memory now in KB
        ])?;
    }
    wtr.flush()?;

    println!("\nTotal execution time: {:.3}s", total_time);
    println!("Peak memory usage: {:.2}KB", total_memory); // Changed to KB
    println!(
        "Performance metrics have been written to: {:?}",
        rust_csv_path
    );
    println!("Note: Old performance files have been removed. Please run main.py to generate new comparison.");
    Ok(())
}

// [Tests remain the same]

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use std::sync::Mutex;
    use std::sync::Once;

    static INIT: Once = Once::new();

    lazy_static! {
        static ref DB_MUTEX: Mutex<()> = Mutex::new(());
    }

    async fn init_database() -> Result<(), Box<dyn Error>> {
        // Download and load test data
        extract::extract(
            "https://github.com/fivethirtyeight/data/raw/refs/heads/master/goose/goose_rawdata.csv",
            "data/goose_rawdata.csv",
        )
        .await?;
        transform_load::load("data/goose_rawdata.csv")?;
        Ok(())
    }

    fn cleanup() {
        let _ = std::fs::remove_file("GooseDB.db");
        let _ = std::fs::remove_file("data/goose_rawdata.csv");
    }

    #[tokio::test]
    async fn test_extract() {
        let _lock = DB_MUTEX.lock().unwrap();
        cleanup();

        let result = extract::extract(
            "https://github.com/fivethirtyeight/data/raw/refs/heads/master/goose/goose_rawdata.csv",
            "data/goose_rawdata.csv",
        )
        .await;
        assert!(result.is_ok(), "Extract test failed!");
    }

    #[tokio::test]
    async fn test_transform() {
        let _lock = DB_MUTEX.lock().unwrap();
        cleanup();

        // First extract
        let extract_result = extract::extract(
            "https://github.com/fivethirtyeight/data/raw/refs/heads/master/goose/goose_rawdata.csv",
            "data/goose_rawdata.csv",
        )
        .await;
        assert!(
            extract_result.is_ok(),
            "Extract failed during transform test"
        );

        // Then transform
        let result = transform_load::load("data/goose_rawdata.csv");
        assert!(result.is_ok(), "Transform test failed!");
    }

    #[tokio::test]
    async fn test_db_query() {
        let _lock = DB_MUTEX.lock().unwrap();
        cleanup();

        // Initialize database first
        init_database()
            .await
            .expect("Database initialization failed");

        let result = query::db_query();
        assert!(result.is_ok(), "DB Query test failed!");
    }

    #[tokio::test]
    async fn test_crud_create() {
        let _lock = DB_MUTEX.lock().unwrap();
        cleanup();

        // Initialize database first
        init_database()
            .await
            .expect("Database initialization failed");

        let result = query::crud_create();
        assert!(result.is_ok(), "CRUD Create test failed!");
    }

    #[tokio::test]
    async fn test_crud_read() {
        let _lock = DB_MUTEX.lock().unwrap();
        cleanup();

        // Initialize database and create test data
        init_database()
            .await
            .expect("Database initialization failed");
        query::crud_create().expect("Create failed during read test");

        let result = query::crud_read();
        assert!(result.is_ok(), "CRUD Read test failed!");
    }

    #[tokio::test]
    async fn test_crud_update() {
        let _lock = DB_MUTEX.lock().unwrap();
        cleanup();

        // Initialize database and create test data
        init_database()
            .await
            .expect("Database initialization failed");
        query::crud_create().expect("Create failed during update test");

        let result = query::crud_update();
        assert!(result.is_ok(), "CRUD Update test failed!");
    }

    #[tokio::test]
    async fn test_crud_delete() {
        let _lock = DB_MUTEX.lock().unwrap();
        cleanup();

        // Initialize database and create test data
        init_database()
            .await
            .expect("Database initialization failed");
        query::crud_create().expect("Create failed during delete test");

        let result = query::crud_delete();
        assert!(result.is_ok(), "CRUD Delete test failed!");
    }
}
