/**
 * Tauri 应用入口
 */
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod error;
mod events;
mod task;
mod utils;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_app_config,
            commands::load_passwords,
            commands::save_passwords,
            commands::preview_task,
            commands::start_task,
            commands::cancel_task,
            commands::open_log_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
