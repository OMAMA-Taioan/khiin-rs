// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::RwLock;
use tauri::Manager;
use tauri::State;
use tauri::StateManager;
use tauri_plugin_log::LogTarget;

use khiin_settings::AppSettings;
use khiin_settings::SettingsManager;

#[derive(Default)]
struct SettingsStore {
    store: RwLock<SettingsManager>,
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn load_settings(
    state: State<SettingsStore>,
    window: tauri::Window,
) {
    if let Ok(reader) = state.store.read() {
        emit_settings(&reader.settings, window);
    }
}

#[tauri::command]
fn update_settings(
    settings: &str,
    state: State<SettingsStore>,
    window: tauri::Window,
) {
    if let Ok(settings_update) = serde_json::from_str::<AppSettings>(settings) {
        if let Ok(mut writer) = state.store.write() {
            let prev_settings = writer.settings.clone();
            // writer.settings = settings_update.merge(prev_settings.clone());
            if let Ok(_) = writer.save_to_file() {
                emit_settings(&writer.settings, window);
            } else {
                emit_settings(&prev_settings, window);
            }
        }
    }
}

fn emit_settings(settings: &AppSettings, window: tauri::Window) {
    window.emit("update_settings", settings.clone()).unwrap();
}

fn load_settings_manager() -> SettingsStore {
    if let Ok(mut filename) = env::current_dir() {
        filename.push("Khiin.toml");

        log::debug!("{:?}", filename);

        if filename.exists() {
            return SettingsStore {
                store: RwLock::new(SettingsManager::load_from_file(&filename)),
            };
        }
    }

    Default::default()
}

fn main() {
    let settings_manager = load_settings_manager();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    LogTarget::LogDir,
                    LogTarget::Stdout,
                    LogTarget::Webview,
                ])
                .build(),
        )
        .manage(settings_manager)
        .invoke_handler(tauri::generate_handler![
            load_settings,
            update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
