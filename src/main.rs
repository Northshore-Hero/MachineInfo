#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{Connection, Result as SqliteResult};
use std::env;
use std::error::Error;
use std::path::PathBuf;
slint::include_modules!();
use sysinfo::{Components, Disks, Networks, System};

struct Processor {
    name: String,
    vendor: String,
    family: String,
    speed: String,
    cores: String,
    usage: String,
}

fn get_cpu_architecture() -> String {
    #[cfg(any(target_arch = "x86"))]
    {
        return "x86".to_string();
    }
    #[cfg(any(target_arch = "x86_64"))]
    {
        return "x86_64".to_string();
    }
    #[cfg(any(target_arch = "aarch64"))]
    {
        return "aarch64".to_string();
    }
    #[cfg(any(target_arch = "arm"))]
    {
        return "arm".to_string();
    }
    #[cfg(any(target_arch = "riscv32"))]
    {
        return "riscv32".to_string();
    }
    #[cfg(any(target_arch = "riscv64"))]
    {
        return "riscv64".to_string();
    }
    #[cfg(any(target_arch = "powerpc"))]
    {
        return "powerpc".to_string();
    }
    #[cfg(any(target_arch = "powerpc64"))]
    {
        return "powerpc64".to_string();
    }
    #[cfg(any(target_arch = "mips"))]
    {
        return "mips".to_string();
    }
    #[cfg(any(target_arch = "mips64"))]
    {
        return "mips64".to_string();
    }
    //TODO add more architectures
    return "Unknown Architecture".to_string();
}

fn get_cpu_info() -> Processor {
    // Declare Constants
    const _MHZ_TO_GHZ: f32 = 1000.0;
    // Declare Variables
    let mut _running_system = System::new_all();
    let mut _cpu_count = 0;
    let mut _my_processor = Processor {
        name: "".to_string(),
        vendor: "".to_string(),
        family: "".to_string(),
        speed: "".to_string(),
        cores: "".to_string(),
        usage: "".to_string(),
    };
    
    // Define CPU Info
    _running_system.refresh_cpu_all();
    let _my_cpu = _running_system.cpus().first().unwrap();
    
    // Count the number of cores
    for cpu in _running_system.cpus() {
        _cpu_count += 1;
    }

    // Get speed in Ghz
    // cast frequency into a float
    let _temp_freq: f32 = _my_cpu.frequency() as f32;
    let _my_speed: f32 = _temp_freq / _MHZ_TO_GHZ;
    
    // Pack Struct
    _my_processor.name = String::from(_my_cpu.brand());
    _my_processor.vendor = String::from(_my_cpu.vendor_id());
    _my_processor.cores = String::from(_cpu_count.to_string());
    //Overwrite the string to concatenate the units
    _my_processor.speed = String::from(_my_speed.to_string());
    _my_processor.speed.push_str(" GHz");
    _my_processor.usage = String::from(_my_cpu.cpu_usage().to_string());
    _my_processor.family = get_cpu_architecture();
    
    // Return Processor Info
    _my_processor
}

// Determines the execution environment based on debug assertions
// Returns Some(true) for development mode, Some(false) for release mode
fn get_if_dev() -> Option<bool> {
    if cfg!(debug_assertions) {
        Some(true)
    } else {
        Some(false)
    }
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
        // Attempts to locate the database alongside the executable
        // Fall back to the local directory if an executable path is unavailable
        //env::current_exe()
        //    .map(|exe_path| exe_path.parent().unwrap_or(&exe_path).join("app.db"))
        //    .unwrap_or_else(|_| {
        //        eprintln!("Warning: Couldn't determine executable path, using local path");
        //        PathBuf::from("app.db")
        //    })
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