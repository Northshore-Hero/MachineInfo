use sysinfo::System;

pub struct Processor {
    pub name: String,
    pub vendor: String,
    pub family: String,
    pub speed: String,
    pub cores: String,
    pub usage: String,
}

pub fn get_cpu_info() -> Processor {
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