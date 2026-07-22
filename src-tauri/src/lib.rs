/**
 * Tauri 应用库
 */
pub mod commands;
pub mod config;
pub mod error;
pub mod events;
pub mod task;
pub mod utils;

/// 应用入口逻辑
///
/// `main.rs` 仅做最小委托，所有模块声明与 Builder 构建统一在此处维护，
/// 避免二进制与库两处重复声明模块。
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_app_config,
            commands::load_passwords,
            commands::save_passwords,
            commands::preview_task,
            commands::start_task,
            commands::cancel_task,
            commands::open_folder,
            commands::open_log_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
