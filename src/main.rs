#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{Connection, Result as SqliteResult};
use std::env;
use std::error::Error;
use std::path::PathBuf;
use sysinfo::System;

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
        // Get the path to the app bundle's Resources directory
        let mut _release_path = PathBuf::new();
        if let Ok(exe_path) = env::current_exe() {
            // On macOS, the executable is typically at MyApp.app/Contents/MacOS/executable
            // We want to store the database in MyApp.app/Contents/Resources/
            if let Some(exe_dir) = exe_path.parent() {
                if let Some(macos_dir) = exe_dir.parent() {
                    if let Some(contents_dir) = macos_dir.parent() {
                        _release_path = contents_dir.join("Resources").join("app.db");
                    }
                }
            }
        }
        
        // Fallback to home directory if we can't access the app bundle
        if _release_path.parent().is_none() {
            _release_path = env::home_dir()
                .unwrap()
                .join("Library")
                .join("Application Support")
                .join(env!("CARGO_PKG_NAME"))
                .join("app.db");
        }
        
        // Ensure the parent directory exists
        if let Some(parent) = _release_path.parent() {
            std::fs::create_dir_all(parent).unwrap_or_else(|e| {
                eprintln!("Failed to create database directory: {}", e);
            });
        }
        
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
    // Initialize connection to computer
    let mut _running_system = System::new_all();
    _running_system.refresh_all();

    // Get system information
    let _cpuid = get_cpu_info(&mut _running_system);
    let _memory = get_memory_info(&mut _running_system);

    // Initialize database connection
    let conn = init_db()?;

    // Initialize UI components
    let ui = AppWindow::new()?;

    // Restore previous session state
    if let Ok(saved_entry) = get_saved_entry(&conn) {
        ui.set_input_text(saved_entry.into());
    }

    // Pass CPU to UI
    ui.set_cpu_id(_cpuid.name.into());
    ui.set_cpu_vendor(_cpuid.vendor.into());
    ui.set_cpu_speed(_cpuid.speed.into());
    ui.set_cpu_cores(_cpuid.cores.into());
    ui.set_cpu_usage(_cpuid.usage.into());
    ui.set_cpu_family(_cpuid.family.into());

    // Pass Memory to UI
    ui.set_memory_total(_memory.total.into());
    ui.set_memory_used(_memory.used.into());
    ui.set_memory_free(_memory.free.into());

    // Refresh
    ui.on_file_refresh({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            // Get system information
            let _cpuid = get_cpu_info(&mut _running_system);
            let _memory = get_memory_info(&mut _running_system);
            // Pass CPU to UI
            ui.set_cpu_id(_cpuid.name.into());
            ui.set_cpu_vendor(_cpuid.vendor.into());
            ui.set_cpu_speed(_cpuid.speed.into());
            ui.set_cpu_cores(_cpuid.cores.into());
            ui.set_cpu_usage(_cpuid.usage.into());
            ui.set_cpu_family(_cpuid.family.into());
            // Pass Memory to UI
            ui.set_memory_total(_memory.total.into());
            ui.set_memory_used(_memory.used.into());
            ui.set_memory_free(_memory.free.into());
        }
    });

    ui.on_save_input({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let input_text = ui.get_input_text().to_string();
            println!("Saving input: {}", input_text);
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
        }
    });

    // Configure application termination handler
    ui.on_file_close(|| {
        println!("Closing Application");
        std::process::exit(0);
    });

    // Configure launching of application
    let weak_app = ui.as_weak();
    slint::invoke_from_event_loop(move || {
        println!("{:?}", weak_app.unwrap().window().position());
        // Set the window position to specific x, y coordinates
        weak_app.unwrap().window().set_position(slint::PhysicalPosition::new(600, 300));
    }).unwrap();

    // Launch application event loop
    ui.run()?;
    Ok(())
}