// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::command;
use std::process::Command;

#[command]
fn start_database() -> String {
    let output = Command::new("node")
        .arg("scripts/db/createdb.js")
        .output()
        .expect("Failed to start OrbitDB");

    let result = String::from_utf8_lossy(&output.stdout).to_string();
    format!("OrbitDB Output: {}", result)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_database])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
