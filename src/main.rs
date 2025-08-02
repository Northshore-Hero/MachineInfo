#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{Connection, Result as SqliteResult};
use std::env;
use std::error::Error;
use std::path::PathBuf;
use sysinfo::System;
use std::sync::Arc;  // Add this import

slint::include_modules!();
use MachineInfo::{get_cpu_info, get_if_dev, get_memory_info};

struct Dimension {
    x_position: String,
    y_position: String
}

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
        
        // Fallback to the home directory if we can't access the app bundle
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

fn init_db() -> SqliteResult<Connection> {
    let db_path = get_db_path();
    println!("Using database at: {}", db_path.display());
    let conn = Connection::open(db_path)?;

    // Create the table only if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS UserSettings (
            id INTEGER PRIMARY KEY,
            item_name TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    // Insert default values ONLY if they don't exist (using INSERT OR IGNORE)
    conn.execute(
        "INSERT OR IGNORE INTO UserSettings (id, item_name, content)
         VALUES 
            (1, 'Default Entry', 'Initial Setting'),
            (2, 'WindowWidth', '600'),
            (3, 'WindowHeight', '300')",
        [],
    )?;

    Ok(conn)
}

// Retrieves the currently stored entry from the database
// Returns: Result containing the stored content string
fn get_saved_entry(conn: &Connection) -> SqliteResult<String> {
    conn.query_row("SELECT content FROM UserSettings WHERE id = 1", [], |row| {
        row.get(0)
    })
}

fn set_saved_entry(conn: &Connection, entry: &str) {
    conn.execute(
        "UPDATE UserSettings SET content = ?1 WHERE id = 1",
        [entry],
    ).expect("Unable to Save Entry");
}

fn set_window_position(conn: &Connection, width: i32, height: i32) {
    conn.execute(
        "REPLACE INTO UserSettings (id, item_name, content)
         VALUES
            (2, 'WindowWidth', ?1),
            (3, 'WindowHeight', ?2)",
        [width, height],
    ).expect("Unable to Save Window Position");
}

fn get_window_position(conn: &Connection) -> Dimension {
    let mut my_dimension: Dimension = Dimension { x_position: "".to_string(), y_position: "".to_string() };
    my_dimension.x_position = conn.query_row(
        "SELECT content FROM UserSettings WHERE item_name = 'WindowWidth'",
        [],
        |row| row.get(0)
    ).expect("Read Failure");

    my_dimension.y_position = conn.query_row(
        "SELECT content FROM UserSettings WHERE item_name = 'WindowHeight'",
        [],
        |row| row.get(0)
    ).expect("Read Failure");
    // Return the dimensions
    my_dimension
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize connection to computer
    let mut _running_system = System::new_all();
    _running_system.refresh_all();

    // Get system information
    let _cpuid = get_cpu_info(&mut _running_system);
    let _memory = get_memory_info(&mut _running_system);

    // Initialize the database connection and wrap it in Arc (allows multiple conn.executes)
    let conn = Arc::new(init_db()?);

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
        let conn = Arc::clone(&conn);
        move || {
            let ui = ui_handle.unwrap();
            let input_text = ui.get_input_text().to_string();
            println!("Saving input: {}", input_text);
            set_saved_entry(&conn, &input_text);
        }
    });

    // Configure application termination handler
    ui.on_file_close({
        let ui_handle = ui.as_weak();
        let conn = Arc::clone(&conn);
        move || {
            println!("{:?}", ui_handle.unwrap().window().position());
            set_window_position(&conn, ui_handle.unwrap().window().position().x, ui_handle.unwrap().window().position().y);
            std::process::exit(0);
        }
    });
    
    ui.window().on_close_requested({
        let ui_handle = ui.as_weak();
        let conn = Arc::clone(&conn);
        move || {
            println!("{:?}", ui_handle.unwrap().window().position());
            set_window_position(&conn, ui_handle.unwrap().window().position().x, ui_handle.unwrap().window().position().y);
            std::process::exit(0);
        }
    });

    // Configure launching of application
    let weak_app = ui.as_weak();
    let dimensions = get_window_position(&conn);
    slint::invoke_from_event_loop(move || {
        // Set the window position to specific x, y coordinates
        weak_app.unwrap().window().set_position(slint::PhysicalPosition::new(dimensions.x_position.parse().unwrap(), dimensions.y_position.parse().unwrap()));
    }).unwrap();

    // Launch application event loop
    ui.run()?;
    Ok(())
}