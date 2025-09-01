#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::error::Error;
use std::sync::Arc;
use machine_info::{Memory, Storage, Processor, database};
slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the database connection and wrap it in Arc (allows multiple conn.executes)
    let conn = Arc::new(database::init_db()?);

    // Get connection to disks
    let mut _storage_connection = Storage::set_storage_connection();
    let _storage = Storage::get_storage_info(&mut _storage_connection);

    // Get CPU information
    let mut _cpu_connection = Processor::set_cpu_connection();
    let _cpu = Processor::get_cpu_info(&mut _cpu_connection);

    // Get memory information
    let mut _memory_connection = Memory::set_memory_connection();
    let _memory = Memory::get_memory_info(&mut _memory_connection);

    // Initialize UI components
    let ui = AppWindow::new()?;

    // Restore previous session state
    if let Ok(saved_entry) = database::get_saved_entry(&conn) {
        ui.set_input_text(saved_entry.into());
    }

    // Pass CPU to UI
    ui.set_cpu_id(_cpu.name.into());
    ui.set_cpu_vendor(_cpu.vendor.into());
    ui.set_cpu_speed(_cpu.speed.into());
    ui.set_cpu_cores(_cpu.cores.into());
    ui.set_cpu_usage(_cpu.usage.into());
    ui.set_cpu_family(_cpu.family.into());

    // Pass Memory to UI
    ui.set_memory_total(_memory.total.into());
    ui.set_memory_used(_memory.used.into());
    ui.set_memory_free(_memory.free.into());

    // Pass Storage to UI
    ui.set_storage_name(_storage.name.into());
    ui.set_storage_total(_storage.total_space.into());
    ui.set_storage_used(_storage.used_space.into());
    ui.set_storage_free(_storage.free_space.into());

    // Refresh
    ui.on_file_refresh({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            // Get system information
            let _cpu = Processor::get_cpu_info(&mut _cpu_connection);
            let _memory = Memory::get_memory_info(&mut _memory_connection);
            let _storage = Storage::get_storage_info(&mut _storage_connection);
            // Pass CPU to UI
            ui.set_cpu_id(_cpu.name.into());
            ui.set_cpu_vendor(_cpu.vendor.into());
            ui.set_cpu_speed(_cpu.speed.into());
            ui.set_cpu_cores(_cpu.cores.into());
            ui.set_cpu_usage(_cpu.usage.into());
            ui.set_cpu_family(_cpu.family.into());
            // Pass Memory to UI
            ui.set_memory_total(_memory.total.into());
            ui.set_memory_used(_memory.used.into());
            ui.set_memory_free(_memory.free.into());
            // Pass Storage to the UI.
            ui.set_storage_name(_storage.name.into());
            ui.set_storage_total(_storage.total_space.into());
            ui.set_storage_used(_storage.used_space.into());
            ui.set_storage_free(_storage.free_space.into());
        }
    });

    ui.on_save_input({
        let ui_handle = ui.as_weak();
        let conn = Arc::clone(&conn);
        move || {
            let ui = ui_handle.unwrap();
            let input_text = ui.get_input_text().to_string();
            #[cfg(debug_assertions)]
            println!("Saving input: {}", input_text);
            database::set_saved_entry(&conn, &input_text);
        }
    });

    // Configure application termination handler
    ui.on_file_close({
        let ui_handle = ui.as_weak();
        move || {
            ui_handle
                .unwrap()
                .window()
                .try_dispatch_event(slint::platform::WindowEvent::CloseRequested)
                .expect("Unable to close window");
        }
    });
    // Close file handling
    ui.window().on_close_requested({
        let ui_handle = ui.as_weak();
        let conn = Arc::clone(&conn);
        move || {
            #[cfg(debug_assertions)]
            println!("{:?}", ui_handle.unwrap().window().position());
            database::set_window_position(
                &conn,
                ui_handle.unwrap().window().position().x,
                ui_handle.unwrap().window().position().y,
            );
            std::process::exit(0);
        }
    });

    // Configure launching of application
    let weak_app = ui.as_weak();
    let dimensions = database::get_window_position(&conn);
    slint::invoke_from_event_loop(move || {
        // Set the window position to specific x, y coordinates
        weak_app
            .unwrap()
            .window()
            .set_position(slint::PhysicalPosition::new(
                dimensions.x_position.parse().unwrap(),
                dimensions.y_position.parse().unwrap(),
            ));
    })
    .unwrap();

    // Launch application event loop
    ui.run()?;
    Ok(())
}
