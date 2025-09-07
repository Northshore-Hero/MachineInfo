pub mod db;
pub mod types;

use sysinfo::{Disks, System};


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
