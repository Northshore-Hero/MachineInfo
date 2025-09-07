use crate::types::Processor;
use sysinfo::System;
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
        _my_processor.family = Self::get_cpu_architecture();
        _my_processor.speed = format!("{:.2} GHz", _temp_freq);
        _my_processor.usage = format!("{:.2} %", _temp_usage);

        // Return Processor Info
        _my_processor
    }
    #[allow(unreachable_code)]
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
        "Unknown Architecture".to_string()
    }
}
