#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{Connection, Result as SqliteResult};
use std::env;
use std::error::Error;
use std::path::PathBuf;
slint::include_modules!();
use MachineInfo::{get_cpu_info, get_if_dev, get_memory_info};

// Determines the appropriate database file path
// In development: Displays relevant environment information
// Returns: PathBuf containing the database location
fn get_db_path() -> PathBuf {
    let _if_dev = get_if_dev();

    if _if_dev == Some(true) {
        println!("Running in dev mode");
        let mut _dev_path = PathBuf::new();
        _dev_path.push(env::current_exe().unwrap());
        _dev_path = _dev_path.parent().unwrap().join("app.db");
        _dev_path
    }
    else {
        println!("Running in release mode");
        let mut _release_path = PathBuf::new();
        _release_path.push(env::home_dir().unwrap());
        //This code is really customizable
        _release_path.push("Documents");
        _release_path.push("Rust Projects");
        _release_path.push("appconfig");
        _release_path.push(env!("CARGO_PKG_NAME"));
        _release_path.push("app.db");
        _release_path
    }
}

// Initializes the SQLite database with required schema
// Creates a single-row table for storing application input
fn init_db() -> SqliteResult<Connection> {
    let db_path = get_db_path();
    println!("Using database at: {}", db_path.display());
    let conn = Connection::open(db_path)?;

    // Initialize the inputs table with id constraint
    conn.execute(
        "CREATE TABLE IF NOT EXISTS inputs (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            content TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    // Ensure default record exists
    conn.execute(
        "INSERT OR IGNORE INTO inputs (id, content) VALUES (1, 'Initial Setting')",
        [],
    )?;

    Ok(conn)
}

// Retrieves the currently stored entry from the database
// Returns: Result containing the stored content string
fn get_saved_entry(conn: &Connection) -> SqliteResult<String> {
    conn.query_row("SELECT content FROM inputs WHERE id = 1", [], |row| {
        row.get(0)
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize database connection
    let conn = init_db()?;

    // Initialize UI components
    let ui = AppWindow::new()?;

    // Restore previous session state
    if let Ok(saved_entry) = get_saved_entry(&conn) {
        ui.set_input_text(saved_entry.into());
    }

    // Get CPU information
    let _cpuid = get_cpu_info();
    // Pass it through to the UI
    ui.set_cpu_id(_cpuid.name.into());
    ui.set_cpu_vendor(_cpuid.vendor.into());
    ui.set_cpu_speed(_cpuid.speed.into());
    ui.set_cpu_cores(_cpuid.cores.into());
    ui.set_cpu_usage(_cpuid.usage.into());
    ui.set_cpu_family(_cpuid.family.into());

    // Get memory information
    let _memory = get_memory_info();
    ui.set_memory_total(_memory.total.into());
    ui.set_memory_used(_memory.used.into());
    ui.set_memory_free(_memory.free.into());

    // Create a weak reference for use in callbacks
    let weak = ui.as_weak();

    // Configure save input handler
    ui.on_save_input(move || {
        let ui = weak.unwrap();
        let input_text = ui.get_input_text().to_string();

        // Persist current input state
        match conn.execute(
            "UPDATE inputs SET content = ?1, created_at = CURRENT_TIMESTAMP WHERE id = 1",
            [&input_text],
        ) {
            Ok(_) => {
                println!("Successfully saved to database!");
            }
            Err(e) => {
                eprintln!("Error saving to database: {}", e);
            }
        }
    });

    // Configure application termination handler
    ui.on_file_close(|| {
        println!("Closing Application");
        std::process::exit(0);
    });

    // Launch application event loop
    ui.run()?;
    Ok(())
}