use std::env;
use std::path::PathBuf;

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

// Determines the execution environment based on debug assertions
// Returns Some(true) for development mode, Some(false) for release mode
pub fn get_if_dev() -> Option<bool> {
    if cfg!(debug_assertions) {
        Some(true)
    } else {
        Some(false)
    }
}