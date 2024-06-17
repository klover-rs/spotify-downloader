
use anyhow::{Result, anyhow};
use std::{fs, path::PathBuf, process::Command};

pub mod creds;
pub mod file_path_location;

const DB_NAME: &str = "storage-mf.db3";

fn get_current_user() -> Result<String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "whoami"])
            .output()?
    } else {
        Command::new("sh")
            .arg("whoami")
            .output()?
    };

    let output = output.stdout;
    let output_string = String::from_utf8_lossy(&output);

    Ok(output_string.to_string())
}


fn get_database_path() -> Result<PathBuf> {
    let document_dir = dirs::document_dir();
    let document_dir = if !document_dir.is_none() {
        document_dir.unwrap()
    } else {
        return Err(anyhow!("document dir doesnt exist!"));
    };

    let spotify_data_folder = document_dir.join("spotify_dl_data");

    if !spotify_data_folder.exists() {
        fs::create_dir(spotify_data_folder.clone())?;
    }

    Ok(spotify_data_folder.join(format!("{}", DB_NAME)))
}