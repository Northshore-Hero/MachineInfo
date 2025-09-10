#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use machine_info::{Dimension, Memory, Processor, Storage, WindowInformation};
use std::env;
use std::error::Error;
use std::sync::Arc;
slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    // Get connection to database
    let conn = Arc::new(WindowInformation::connect_to_db()?);
    let wi = WindowInformation::load_from_db(&conn)?;

    // Get connection to disks
    let mut _storage_connection = Storage::get_storage_connection();
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
    //if let Ok(saved_entry) = db_controls::get_saved_entry(&temp_dataconnection) {
    //    ui.set_input_text(saved_entry.into());
    //}

    // Pass CPU to UI
    ui.set_cpu_id(_cpu.name.unwrap_or_default().into());
    ui.set_cpu_vendor(_cpu.vendor.unwrap_or_default().into());
    ui.set_cpu_speed(_cpu.speed.unwrap_or_default().into());
    ui.set_cpu_cores(_cpu.cores.unwrap_or_default().into());
    ui.set_cpu_usage(_cpu.usage.unwrap_or_default().into());
    ui.set_cpu_family(_cpu.family.unwrap_or_default().into());

    // Pass Memory to UI
    ui.set_memory_total(_memory.total.unwrap_or_default().into());
    ui.set_memory_used(_memory.used.unwrap_or_default().into());
    ui.set_memory_free(_memory.free.unwrap_or_default().into());

    // Pass Storage to UI
    ui.set_storage_name(_storage.name.unwrap_or_default().into());
    ui.set_storage_total(_storage.total_space.unwrap_or_default().into());
    ui.set_storage_used(_storage.used_space.unwrap_or_default().into());
    ui.set_storage_free(_storage.free_space.unwrap_or_default().into());
    ui.set_storage_percent_used(_storage.percent_used.unwrap_or_default().into());

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
            ui.set_cpu_id(_cpu.name.unwrap_or_default().into());
            ui.set_cpu_vendor(_cpu.vendor.unwrap_or_default().into());
            ui.set_cpu_speed(_cpu.speed.unwrap_or_default().into());
            ui.set_cpu_cores(_cpu.cores.unwrap_or_default().into());
            ui.set_cpu_usage(_cpu.usage.unwrap_or_default().into());
            ui.set_cpu_family(_cpu.family.unwrap_or_default().into());
            // Pass Memory to UI
            ui.set_memory_total(_memory.total.unwrap_or_default().into());
            ui.set_memory_used(_memory.used.unwrap_or_default().into());
            ui.set_memory_free(_memory.free.unwrap_or_default().into());
            // Pass Storage to the UI.
            ui.set_storage_name(_storage.name.unwrap_or_default().into());
            ui.set_storage_total(_storage.total_space.unwrap_or_default().into());
            ui.set_storage_used(_storage.used_space.unwrap_or_default().into());
            ui.set_storage_free(_storage.free_space.unwrap_or_default().into());
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
        let mut wi = WindowInformation::default();
        move || {
            // get scale factor of display
            let scale_factor = ui_handle.unwrap().window().scale_factor() as u32;
            // save parameter values
            wi.set_all(
                ui_handle.unwrap().window().position().x,
                ui_handle.unwrap().window().position().y,
                ui_handle.unwrap().window().size().width / scale_factor,
                ui_handle.unwrap().window().size().height / scale_factor,
                ui_handle.unwrap().window().is_maximized(),
                ui_handle.unwrap().window().is_fullscreen(),
            );
            // save to the database
            wi.save_to_db(&conn).unwrap();
            // DEBUGGING
            #[cfg(debug_assertions)]
            println!("Saving window position: {:?}", wi);
            #[cfg(debug_assertions)]
            println!(
                "Scale factor: {}",
                ui_handle.unwrap().window().scale_factor()
            );
            // End the application
            std::process::exit(0);
        }
    });

    // Configure launching of application
    let weak_app = ui.as_weak();
    //let dimensions = db_controls::get_window_position(&conn);
    let mut app_dimensions = Dimension::default();
    app_dimensions.x_position = Some(*wi.get_x());
    app_dimensions.y_position = Some(*wi.get_y());
    let app_width = *wi.get_width() as f32;
    let app_height = *wi.get_height() as f32;
    let app_size = slint::LogicalSize::new(app_width, app_height);
    let app_fullscreen = *wi.get_fullscreen();

    slint::invoke_from_event_loop(move || {
        #[cfg(debug_assertions)]
        println!("{:?}", app_size);

        weak_app.unwrap().window().set_size(app_size);
        weak_app.unwrap().window().set_fullscreen(app_fullscreen);


        // Set the window position to specific x, y coordinates
        weak_app
            .unwrap()
            .window()
            .set_position(slint::PhysicalPosition::new(
                app_dimensions.x_position.unwrap_or_default(),
                app_dimensions.y_position.unwrap_or_default(),
            ));
    })
    .unwrap();

    // Launch application event loop
    ui.run()?;
    Ok(())
}
