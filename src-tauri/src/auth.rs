use spotify_dl_lib::verify_login;
use crate::db::{creds::get_creds, creds::store_creds};

#[tauri::command]
pub async fn login(username: &str, password: &str) -> Result<(), String> {
    verify_login(&username, &password).await.map_err(|e| e.to_string())?;

    store_creds(&username, &password).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn is_logged_in() -> Result<bool, String> {
    let (username, password) = get_creds().map_err(|e| e.to_string())?;

    verify_login(&username, &password).await.map_err(|e| e.to_string())?;

    Ok(true)
}

