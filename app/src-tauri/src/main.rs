// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::fs::File;
use std::io::Read;
use tauri::Manager;
use tauri_plugin_log::LogTarget;

use khiin_settings::Settings;

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn load_settings() -> Option<Settings> {
    if let Ok(mut filename) = env::current_dir() {
        filename.set_file_name("Khiin.toml");
        Settings::load_from_file(&filename)
    } else {
        None
    }
}

fn main() {
    log::debug!("Testing logger");
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .plugin(tauri_plugin_log::Builder::default().targets([
            LogTarget::Stdout,
        ]).build())
        .invoke_handler(tauri::generate_handler![load_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
