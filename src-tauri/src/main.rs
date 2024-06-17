// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod downloader;
mod ws;
mod auth;
mod db;

use crate::{
    downloader::{download_tracks, set_directory},
    auth::{login, is_logged_in}
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, download_tracks, login, is_logged_in, set_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
