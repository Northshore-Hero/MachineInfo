#[derive(Debug, Default, Clone)]
pub struct WindowInformation {
    pub x: i32, // X Start Position
    pub y: i32, // Y Start Position
    pub width: u32, // Width of the window
    pub height: u32, // Height of the window
    pub maximized: bool, // Is the window maximized?
    pub fullscreen: bool // Is the window fullscreen?
}

#[derive(Debug, Default)]
pub struct Dimension {
    // String because app.db uses TEXT for the column type
    pub x_position: Option<i32>,
    pub y_position: Option<i32>,
}

#[derive(Debug, Default, Clone)]
pub struct Storage {
    pub name: Option<String>,
    pub usage: Option<String>,
    pub mount_point: Option<String>,
    pub file_system: Option<String>,
    pub type_: Option<String>,
    pub total_space: Option<String>,  // e.g. "500.00 GB"
    pub free_space: Option<String>,   // e.g. "120.20 GB"
    pub used_space: Option<String>,   // e.g. "379.80 GB"
    pub percent_used: Option<String>, // e.g. "75.96 %"
}

#[derive(Debug, Default, Clone)]
pub struct Processor {
    pub name: Option<String>,
    pub vendor: Option<String>,
    pub family: Option<String>,
    pub speed: Option<String>,
    pub cores: Option<String>,
    pub usage: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct Memory {
    pub total: Option<String>,
    pub used: Option<String>,
    pub free: Option<String>,
}
