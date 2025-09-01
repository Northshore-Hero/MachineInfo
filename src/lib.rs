use sysinfo::{System, Disks};

pub struct Storage {
    pub name: String,
    pub usage: String,
    pub mount_point: String,
    pub file_system: String,
    pub type_: String,
    pub total_space: String,
    pub free_space: String,
    pub used_space: String,
    pub percent_used: String,
}

pub struct Processor {
    pub name: String,
    pub vendor: String,
    pub family: String,
    pub speed: String,
    pub cores: String,
    pub usage: String,
}

pub struct Memory {
    pub total: String,
    pub used: String,
    pub free: String,
}

pub fn get_storage_info(_passed_disks: &mut Disks) -> Storage {
    // Declare Variables
    const _BYTES_TO_GB: f64 = 1000000000.0;
    let _running_disks = _passed_disks;
    let mut _my_storage = Storage {
        name: "".to_string(),
        usage: "".to_string(),
        mount_point: "".to_string(),
        file_system: "".to_string(),
        type_: "".to_string(),
        total_space: "".to_string(),
        free_space: "".to_string(),
        used_space: "".to_string(),
        percent_used: "".to_string(),
    };
    // Define disk Info
    let unwrapped_disk_name = _running_disks.first().unwrap().name();
    let disk_name = unwrapped_disk_name.to_str();
    let unwrapped_disk_size = _running_disks.first().unwrap().total_space() as f64 / _BYTES_TO_GB;
    let unwrapped_disk_space = _running_disks.first().unwrap().available_space() as f64 / _BYTES_TO_GB;
    let used_space = unwrapped_disk_size - unwrapped_disk_space;
    // Pack the Struct
    _my_storage.name = String::from(disk_name.unwrap());
    _my_storage.total_space = format!("{:.2} GB", unwrapped_disk_size);
    _my_storage.free_space = format!("{:.2} GB", unwrapped_disk_space);
    _my_storage.used_space = format!("{:.2} GB", used_space);
    // Return a packed struct
    _my_storage
}

pub fn get_memory_info(_passed_system: &mut System) -> Memory {
    let _running_system = _passed_system;
    // Define Constants
    const _BYTES_TO_GB: f64 = 1024.0 * 1024.0 * 1024.0;
    // Declare Variables.0
    let mut _my_memory = Memory {
        total: "".to_string(),
        used: "".to_string(),
        free: "".to_string(),
    };
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

pub fn get_cpu_info(_passed_system: &mut System) -> Processor {
    // Create a vector to store the percentages of each core
    let mut _core_percents: Vec<f32> = Vec::new();
    // Create a reference to the passed system
    let _running_system = _passed_system;
    // Declare Constants
    const _MHZ_TO_GHZ: f64 = 1000.0;
    // Declare Variables
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
    return "Unknown Architecture".to_string();
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
