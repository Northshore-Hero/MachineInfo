use crate::types::Storage;
use sysinfo::Disks;

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
        if let Some(disk) = passed_disks.first() {
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
