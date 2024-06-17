use std::path::PathBuf;
use anyhow::{anyhow, Result};
use rusqlite::{params, Connection};

use super::get_database_path;


pub fn store_file_path_location(file_path: &PathBuf) -> Result<()> {

    let file_path_str = file_path.to_string_lossy().into_owned();

    let db_path = get_database_path()?;

    let mut conn = Connection::open(&db_path)?;

    let trans = conn.transaction()?;

    trans.execute("CREATE TABLE IF NOT EXISTS download_folder (
        key INTEGER PRIMARY KEY,
        file_path TEXT
    )", [])?;

    trans.execute("INSERT OR REPLACE INTO download_folder (
        file_path
    ) VALUES (?1)", [&file_path_str])?;

    trans.commit()?;

    Ok(())
}

pub fn get_file_path_location() -> Result<String> {
    
    let db_path = get_database_path()?;

    let conn = Connection::open(&db_path)?;

    let mut stmt = conn.prepare("SELECT * FROM download_folder")?;

    let rows = stmt.query_map(params![], |row| {
        Ok(
            row.get::<_, String>(1)?,
        )
    })?;

    let mut all_rows = Vec::new();

    for row in rows {
        all_rows.push(row?);
    }

    if let Some(file_path) = all_rows.first() {
        Ok(file_path.clone())
    } else {
        Err(anyhow!("No rows were returned from the database!"))
    }
}