use crate::types::Memory;
use sysinfo::System;

impl Memory {
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
        let mut _my_memory = Memory::default();
        // Refresh memory
        _running_system.refresh_memory();

        let mut _temp_total = _running_system.total_memory() as f64 / _BYTES_TO_GB;
        let _temp_free = _running_system.available_memory() as f64 / _BYTES_TO_GB;
        let _temp_used = _temp_total - _temp_free;

        // Pack the struct
        _my_memory.total = format!("{:.2} GB", _temp_total).into();
        _my_memory.used = format!("{:.2} GB", _temp_used).into();
        _my_memory.free = format!("{:.2} GB", _temp_free).into();

        // Return Memory Info
        _my_memory
    }
}
