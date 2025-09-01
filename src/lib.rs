use sysinfo::{Disks, System};

pub mod database {
    use std::env;
    use std::path::PathBuf;
    use rusqlite::{Connection, Result as SqliteResult};
    use crate::{get_if_dev, Dimension};

    // Determines the appropriate database file path
    // In development: Displays relevant environment information
    // Returns: PathBuf containing the database location
    pub fn set_db_path() -> PathBuf {
        let _if_dev = get_if_dev();

        if _if_dev == Some(true) {
            println!("Running in dev mode");
            let mut _dev_path = PathBuf::new();
            _dev_path.push(env::current_exe().unwrap());
            _dev_path = _dev_path.parent().unwrap().join("app.db");
            _dev_path
        } else {
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
    pub fn init_db() -> SqliteResult<Connection> {
        let db_path = set_db_path();
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

    pub fn get_window_position(conn: &Connection) -> Dimension {
        let mut my_dimension = Dimension::default();
        my_dimension.x_position = conn
            .query_row(
                "SELECT content FROM UserSettings WHERE item_name = 'WindowWidth'",
                [],
                |row| row.get(0),
            )
            .expect("Read Failure");

        my_dimension.y_position = conn
            .query_row(
                "SELECT content FROM UserSettings WHERE item_name = 'WindowHeight'",
                [],
                |row| row.get(0),
            )
            .expect("Read Failure");
        // Return the dimensions
        my_dimension
    }
    // Retrieves the currently stored entry from the database
    // Returns: Result containing the stored content string
    pub fn get_saved_entry(conn: &Connection) -> SqliteResult<String> {
        conn.query_row("SELECT content FROM UserSettings WHERE id = 1", [], |row| {
            row.get(0)
        })
    }
    pub fn set_saved_entry(conn: &Connection, entry: &str) {
        conn.execute("UPDATE UserSettings SET content = ?1 WHERE id = 1", [entry])
            .expect("Unable to Save Entry");
    }

    pub fn set_window_position(conn: &Connection, width: i32, height: i32) {
        conn.execute(
            "REPLACE INTO UserSettings (id, item_name, content)
         VALUES
            (2, 'WindowWidth', ?1),
            (3, 'WindowHeight', ?2)",
            [width, height],
        )
            .expect("Unable to Save Window Position");
    }
}
#[derive(Debug, Default)]
pub struct Dimension {
    pub x_position: Option<String>,
    pub y_position: Option<String>,
}
pub struct Processor {
    pub name: String,
    pub vendor: String,
    pub family: String,
    pub speed: String,
    pub cores: String,
    pub usage: String,
}
impl Processor {
    fn new() -> Self {
        Processor {
            name: "".to_string(),
            vendor: "".to_string(),
            family: "".to_string(),
            speed: "".to_string(),
            cores: "".to_string(),
            usage: "".to_string(),
        }
    }
    pub fn set_cpu_connection() -> System {
        let mut _running_system = System::new_all();
        _running_system.refresh_cpu_all();
        _running_system
    }
    pub fn get_cpu_info(_passed_system: &mut System) -> Processor {
        // Create a vector to store the percentages of each core
        let mut _core_percents: Vec<f32> = Vec::new();
        // Create a reference to the passed system
        let _running_system = _passed_system;
        // Declare Constants
        const _MHZ_TO_GHZ: f64 = 1000.0;
        // Declare Variables
        let mut _cpu_count = 0;
        let mut _my_processor = Processor::new();

        // Define CPU Info
        _running_system.refresh_cpu_all();
        let _my_cpu = _running_system.cpus().first().unwrap();

        // Count the number of cores and get the total % usage
        for _cpu in _running_system.cpus() {
            _cpu_count += 1;
            _core_percents.push(_cpu.cpu_usage())
        }
        let _temp_usage: f64 = _core_percents.iter().sum::<f32>() as f64 / _cpu_count as f64;

        // Get speed in Ghz
        // cast frequency into a float
        let _temp_freq: f64 = _my_cpu.frequency() as f64 / _MHZ_TO_GHZ;

        // Pack Struct
        _my_processor.name = String::from(_my_cpu.brand());
        _my_processor.vendor = String::from(_my_cpu.vendor_id());
        _my_processor.cores = String::from(_cpu_count.to_string());
        _my_processor.family = get_cpu_architecture();
        _my_processor.speed = format!("{:.2} GHz", _temp_freq);
        _my_processor.usage = format!("{:.2} %", _temp_usage);

        // Return Processor Info
        _my_processor
    }
}

pub struct Memory {
    pub total: String,
    pub used: String,
    pub free: String,
}
impl Memory {
    fn new() -> Self {
        Memory {
            total: "".to_string(),
            used: "".to_string(),
            free: "".to_string(),
        }
    }
    pub fn set_memory_connection() -> System {
        let mut _running_system = System::new_all();
        _running_system.refresh_memory();
        _running_system
    }
    pub fn get_memory_info(_passed_system: &mut System) -> Memory {
        let _running_system = _passed_system;
        // Define Constants
        const BASE_VALUE: u32 = 1024;
        const RAISED_VALUE: u32 = 3;
        const _BYTES_TO_GB: f64 = BASE_VALUE.pow(RAISED_VALUE) as f64;
        // Declare Variables
        let mut _my_memory = Memory::new();
        // Refresh memory
        _running_system.refresh_memory();

        let mut _temp_total = _running_system.total_memory() as f64 / _BYTES_TO_GB;
        let _temp_free = _running_system.available_memory() as f64 / _BYTES_TO_GB;
        let _temp_used = _temp_total - _temp_free;

        // Pack the struct
        _my_memory.total = format!("{:.2} GB", _temp_total);
        _my_memory.used = format!("{:.2} GB", _temp_used);
        _my_memory.free = format!("{:.2} GB", _temp_free);

        // Return Memory Info
        _my_memory
    }
}
#[derive(Debug, Default)]
pub struct Storage {
    pub name: Option<String>,
    pub usage: Option<String>,
    pub mount_point: Option<String>,
    pub file_system: Option<String>,
    pub type_: Option<String>,
    pub total_space: Option<String>, // e.g. "500.00 GB"
    pub free_space: Option<String>,  // e.g. "120.20 GB"
    pub used_space: Option<String>,  // e.g. "379.80 GB"
    pub percent_used: Option<String>, // e.g. "75.96 %"
}
impl Storage {
    pub fn get_storage_connection() -> Disks {
        let running_disks = Disks::new_with_refreshed_list();
        running_disks
    }
    pub fn get_storage_info(passed_disks: &mut Disks) -> Self {
        // Declare Constants
        const BYTES_TO_GB: f64 = 1_000_000_000.0;

        // Declare Variables
        let mut my_storage = Self::default();

        // Refresh disk info
        passed_disks.refresh(true);

        // Start Unwrapping if we find a disk
        if let Some(disk) = passed_disks.first(){
            let mut percent_used = 0.0;
            let unwrapped_disk_name = disk.name().to_str();
            let unwrapped_disk_size = disk.total_space() as f64 / BYTES_TO_GB;
            let unwrapped_disk_space = disk.available_space() as f64 / BYTES_TO_GB;
            let used_space = unwrapped_disk_size - unwrapped_disk_space;
            if unwrapped_disk_size > 0.0 && used_space > 0.0 {
                percent_used = (used_space / unwrapped_disk_size) * 100.0;
            }

            my_storage.name = Some(String::from(unwrapped_disk_name.unwrap_or_default()));
            my_storage.total_space = format!("{:.2} GB", unwrapped_disk_size).into();
            my_storage.free_space = format!("{:.2} GB", unwrapped_disk_space).into();
            my_storage.used_space = format!("{:.2} GB", used_space).into();
            my_storage.percent_used = format!("{:.2} %", percent_used).into()
        } else {
            eprintln!("Error: Data not found, returning default values");
        }
        // Return a packed struct (or default)
        my_storage
    }
}
#[allow(unreachable_code)]
pub fn get_cpu_architecture() -> String {
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
    "Unknown Architecture".to_string()
}

// Determines the execution environment based on debug assertions
// Returns Some(true) for development mode, Some(false) for release mode
pub fn get_if_dev() -> Option<bool> {
    if cfg!(debug_assertions) {
        Some(true)
    } else {
        Some(false)
    }
}