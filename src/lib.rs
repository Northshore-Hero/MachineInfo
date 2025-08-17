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
    let disk_size = unwrapped_disk_size.to_string();
    let unwrapped_disk_space = _running_disks.first().unwrap().available_space() as f64 / _BYTES_TO_GB;
    let used_space = unwrapped_disk_size - unwrapped_disk_space;
    let unwrapped_used_space = used_space.to_string();
    // Pack the Struct
    _my_storage.name = String::from(disk_name.unwrap());
    _my_storage.total_space = String::from(disk_size);
    _my_storage.free_space = String::from(unwrapped_disk_space.to_string());
    _my_storage.used_space = String::from(unwrapped_used_space);
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
    let mut _temp_free = _running_system.available_memory() as f64 / _BYTES_TO_GB;
    let mut _temp_used = _temp_total - _temp_free;

    // Pack the struct
    _my_memory.total = String::from(_temp_total.to_string());
    _my_memory.total.push_str(" GB");
    _my_memory.used = String::from(_temp_used.to_string());
    _my_memory.used.push_str(" GB");
    _my_memory.free = String::from(_temp_free.to_string());
    _my_memory.free.push_str(" GB");

    // Return Memory Info
    _my_memory
}

pub fn get_cpu_info(_passed_system: &mut System) -> Processor {
    let _running_system = _passed_system;
    // Declare Constants
    const _MHZ_TO_GHZ: f32 = 1000.0;
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

    // Count the number of cores
    for _cpu in _running_system.cpus() {
        _cpu_count += 1;
    }

    // Get speed in Ghz
    // cast frequency into a float
    let _temp_freq: f32 = _my_cpu.frequency() as f32;
    let _my_speed: f32 = _temp_freq / _MHZ_TO_GHZ;
    let _my_speed = (_my_speed * 100.0).round() / 100.0;

    // Fix the usage to a certain length
    let _temp_usage: f32 = _my_cpu.cpu_usage() as f32;
    let _my_usage = (_temp_usage * 100.0).round() / 100.0;

    // Pack Struct
    _my_processor.name = String::from(_my_cpu.brand());
    _my_processor.vendor = String::from(_my_cpu.vendor_id());
    _my_processor.cores = String::from(_cpu_count.to_string());
    //Overwrite the string to concatenate the units
    _my_processor.speed = String::from(_my_speed.to_string());
    _my_processor.speed.push_str(" GHz");
    _my_processor.usage = String::from(_my_usage.to_string());
    _my_processor.usage.push_str(" %");
    _my_processor.family = get_cpu_architecture();

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
