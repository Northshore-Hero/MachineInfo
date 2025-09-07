use std::env;
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;

pub fn set_db_path() -> PathBuf {
    let mut path = PathBuf::new();

    let if_dev = get_if_dev();
    if if_dev == Some(true) {
        if let Ok(exe_path) = env::current_exe() {
            fs::create_dir_all(exe_path.parent().unwrap().join("Resources")).expect("Unable to create directory -> Aborting!");
            let dev_path = exe_path.parent().unwrap().join("Resources/app.db");
            path = dev_path;
        }
    } else {
        if let Some(project_dirs) = ProjectDirs::from("io", "github.northshorehero", "MachineInfo") {
            fs::create_dir_all(project_dirs.config_dir()).expect("Unable to create directory -> Aborting!");
            let prod_path = project_dirs.config_dir().join("app.db");
            path = prod_path
        }
    }

    // Return the pathway
    path
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
