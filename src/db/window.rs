use crate::WindowInformation;
use crate::db::path::set_db_path;
use rusqlite::{Connection, Result as SqliteResult};

impl WindowInformation {
    // Write our getters and setters here
    pub fn get_x(&self) -> &i32 { &self.x }
    pub fn get_y(&self) -> &i32 { &self.y }
    pub fn get_width(&self) -> &u32 { &self.width }
    pub fn get_height(&self) -> &u32 { &self.height }
    pub fn get_maximized(&self) -> &bool { &self.maximized }
    pub fn get_fullscreen(&self) -> &bool { &self.fullscreen }
    pub fn get_all(&self) -> &WindowInformation { &self }
    pub fn set_x(&mut self, x: i32) { self.x = x }
    pub fn set_y(&mut self, y: i32) { self.y = y }
    pub fn set_width(&mut self, width: u32) { self.width = width }
    pub fn set_height(&mut self, height: u32) { self.height = height }
    pub fn set_maximized(&mut self, maximized: bool) { self.maximized = maximized }
    pub fn set_fullscreen(&mut self, fullscreen: bool) { self.fullscreen = fullscreen }
    pub fn set_all(&mut self, x: i32, y: i32, width: u32, height: u32, maximized: bool, fullscreen: bool) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
        self.maximized = maximized;
        self.fullscreen = fullscreen;
    }

    pub fn connect_to_db() -> SqliteResult<Connection> {
        // Open the database
        let db_path = set_db_path().map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;
        if let Some(is_dev) = crate::db::path::get_if_dev() {
            if is_dev {
                println!("Using development database at {}", db_path.display());
            }
        }
        let conn = Connection::open(&db_path)?;

        // Create the table only if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS WindowSettings (
            id INTEGER PRIMARY KEY,
            x INTEGER NOT NULL,
            y INTEGER NOT NULL,
            width INTEGER NOT NULL,
            height INTEGER NOT NULL,
            maximized INTEGER NOT NULL,
            fullscreen INTEGER NOT NULL,
            modified_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
            [],
        )?;

        // Insert default values ONLY if they don't exist (using INSERT OR IGNORE)
        conn.execute(
            "INSERT OR IGNORE INTO WindowSettings (id, x, y, width, height, maximized, fullscreen)
         VALUES
            (1, '600', '300', '1000', '600', '0', '0')",
            [],
        )?;

        // Return the connection
        Ok(conn)
    }
    pub fn save_to_db(&self, conn: &Connection) -> SqliteResult<()> {
        conn.execute(
            "INSERT INTO WindowSettings (id, x, y, width, height, maximized, fullscreen, modified_at)
             VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6, CURRENT_TIMESTAMP)
             ON CONFLICT(id) DO UPDATE SET
                 x = excluded.x,
                 y = excluded.y,
                 width = excluded.width,
                 height = excluded.height,
                 maximized = excluded.maximized,
                 fullscreen = excluded.fullscreen,
                 modified_at = CURRENT_TIMESTAMP",
            (self.x, self.y, self.width as i64, self.height as i64, i32::from(self.maximized), i32::from(self.fullscreen)),
        )?;
        Ok(())
    }
    pub fn load_from_db(conn: &Connection) -> SqliteResult<WindowInformation> {
        let mut wi = WindowInformation::default();
        conn.query_row(
            "SELECT x, y, width, height, maximized, fullscreen FROM WindowSettings WHERE id = 1",
            [],
            |row| {
                wi.x = row.get(0)?;
                wi.y = row.get(1)?;
                let w: i64 = row.get(2)?;
                let h: i64 = row.get(3)?;
                let m: i64 = row.get(4)?;
                let f: i64 = row.get(5)?;
                wi.width = w as u32;
                wi.height = h as u32;
                wi.maximized = m != 0;
                wi.fullscreen = f != 0;
                Ok(())
            },
        )?;
        Ok(wi)
    }
}