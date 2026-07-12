// Tauri 应用入口：委托 sjs_unzip_lib::run，避免与 lib.rs 重复声明模块
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    sjs_unzip_lib::run();
}
