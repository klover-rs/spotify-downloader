use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Result as AnyResult, anyhow};
use tauri::Manager;
use crate::ws::connect_ws;
use crate::db::creds::get_creds;
use crate::db::file_path_location::{get_file_path_location, store_file_path_location};
use spotify_dl_lib::SpotifyDownloader;

enum SpotifyType {
    Track,
    Playlist,
    Album
}

fn input_validation(input: &str) -> AnyResult<String> {

    if !input.starts_with("https://open.spotify.com/") {
        return Err(anyhow!("not a spotify url."));
    }

    let spotify_type = match input {
        s if s.starts_with("https://open.spotify.com/track/") => SpotifyType::Track,
        s if s.starts_with("https://open.spotify.com/playlist/") => SpotifyType::Playlist,
        s if s.starts_with("https://open.spotify.com/album/") => SpotifyType::Album,
        _ => return Err(anyhow!("unsupported type"))
    };

    let input_parts: Vec<String> = input.split("/").map(|s| s.to_string()).collect();

    if let Some(last_input_part) = input_parts.last() {
        let split_last_input_part: Vec<String> = last_input_part.split("?").map(|s| s.to_string()).collect();

        if split_last_input_part.len() == 2 {
            return Ok(input.to_string());
        } else if split_last_input_part.len() == 1 {
            let spotify_uri = spotify_url_to_uri(&input, spotify_type).unwrap();
            return Ok(spotify_uri);
        } else {
            return Err(anyhow!("url contains a unvalid query"))
        }
    } else {
        return Err(anyhow!("if you triggered this error, you really fucked up. thats not a valid probably url"))
    }


}

fn spotify_url_to_uri(spotify_url: &str, spotify_type: SpotifyType) -> Option<String> {
    match spotify_type {
        SpotifyType::Track => {
            let prefix_to_replace = "https://open.spotify.com/track/";

            let new_prefix = "spotify:track:";

            if spotify_url.starts_with(prefix_to_replace) {
                let spotify_uri = spotify_url.replacen(prefix_to_replace, &new_prefix, 1);
                Some(spotify_uri)
            } else {
                None
            }
        }
        SpotifyType::Playlist => {
            let prefix_to_replace = "https://open.spotify.com/playlist/";

            let new_prefix = "spotify:playlist:";

            if spotify_url.starts_with(prefix_to_replace) {
                let spotify_uri = spotify_url.replacen(prefix_to_replace, &new_prefix, 1);
                Some(spotify_uri)
            } else {
                None
            }
        }
        SpotifyType::Album => {
            let prefix_to_replace = "https://open.spotify.com/album/";

            let new_prefix = "spotify:album:";

            if spotify_url.starts_with(prefix_to_replace) {
                let spotify_uri = spotify_url.replacen(prefix_to_replace, &new_prefix, 1);
                Some(spotify_uri)
            } else {
                None
            }
        }
        
    }
}

#[tauri::command] 
pub async fn download_tracks(url: &str, app_handle: tauri::AppHandle) -> Result<String, String> {
    let input_validated = input_validation(url).map_err(|e| e.to_string())?;

    start_downloading(&input_validated, app_handle).await.map_err(|e| e.to_string())?;

    Ok(input_validated)
}

pub async fn start_downloading(spotify_url_or_uri: &str, app_handle: tauri::AppHandle) -> AnyResult<()> {

    let app_handle_clone = app_handle.clone();


    let spotify_url_or_uri = vec![spotify_url_or_uri.to_string()];

    let (username, password) = get_creds()?;

    let file_path = PathBuf::from(get_file_path_location()?);

    let spotify_dl = SpotifyDownloader::new(file_path, &username, &password).await?;

    tokio::time::sleep(Duration::from_secs(1)).await;

    tokio::task::spawn(async move {
        connect_ws(app_handle).await.unwrap()
    });
    
    spotify_dl.download_tracks(spotify_url_or_uri, Some(5), Some(4), "mp3").await?;

    app_handle_clone.emit_all("finished-download", true)?;

    Ok(())
}

#[tauri::command]
pub fn set_directory(file_path: &str) -> Result<(), String> {
    
    store_file_path_location(&PathBuf::from(file_path)).map_err(|e| e.to_string())?;
    
    Ok(())
}