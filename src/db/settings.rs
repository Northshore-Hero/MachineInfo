use rusqlite::{Connection, Result as SqliteResult};
use crate::db::path::set_db_path;
use crate::types::Dimension;

pub fn init_db() -> SqliteResult<Connection> {
    let db_path = set_db_path();
    println!("Using database at: {}", db_path.display());
    let conn = Connection::open(db_path)?;

    // Create the table only if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS UserSettings (
            id INTEGER PRIMARY KEY,
            item_name TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    // Insert default values ONLY if they don't exist (using INSERT OR IGNORE)
    conn.execute(
        "INSERT OR IGNORE INTO UserSettings (id, item_name, content)
         VALUES
            (1, 'Default Entry', 'Initial Setting'),
            (2, 'WindowWidth', '600'),
            (3, 'WindowHeight', '300')",
        [],
    )?;

    Ok(conn)
}

pub fn get_window_position(conn: &Connection) -> Dimension {
    let mut my_dimension = Dimension::default();
    my_dimension.x_position = conn
        .query_row(
            "SELECT content FROM UserSettings WHERE item_name = 'WindowWidth'",
            [],
            |row| row.get(0),
        )
        .expect("Read Failure");

    my_dimension.y_position = conn
        .query_row(
            "SELECT content FROM UserSettings WHERE item_name = 'WindowHeight'",
            [],
            |row| row.get(0),
        )
        .expect("Read Failure");
    // Return the dimensions
    my_dimension
}
// Retrieves the currently stored entry from the database
// Returns: Result containing the stored content string
pub fn get_saved_entry(conn: &Connection) -> SqliteResult<String> {
    conn.query_row("SELECT content FROM UserSettings WHERE id = 1", [], |row| {
        row.get(0)
    })
}
pub fn set_saved_entry(conn: &Connection, entry: &str) {
    conn.execute("UPDATE UserSettings SET content = ?1 WHERE id = 1", [entry])
        .expect("Unable to Save Entry");
}

pub fn set_window_position(conn: &Connection, width: i32, height: i32) {
    conn.execute(
        "REPLACE INTO UserSettings (id, item_name, content)
         VALUES
            (2, 'WindowWidth', ?1),
            (3, 'WindowHeight', ?2)",
        [width, height],
    )
        .expect("Unable to Save Window Position");
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