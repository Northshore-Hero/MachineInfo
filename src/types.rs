#[derive(Debug, Default)]
pub struct Dimension {
    // String because app.db uses TEXT for the column type
    pub x_position: Option<String>,
    pub y_position: Option<String>,
}

#[derive(Debug, Default)]
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
