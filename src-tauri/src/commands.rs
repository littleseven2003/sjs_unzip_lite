/**
 * Tauri 命令定义
 * 基于 design.md 第 9.1 节
 */
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::config::PasswordConfig;
use crate::error::AppError;
use crate::task::context::TaskContext;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskInput {
    pub root_dir: String,
    pub final_folder_name: Option<String>,
    pub continue_on_initial_extra_files: bool,
    pub selected_volume_group_id: Option<String>,
    pub selected_txt_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskPreview {
    pub root_dir: String,
    pub default_final_folder_name: String,
    pub volume_groups: Vec<VolumeGroupPreview>,
    pub extra_files: Vec<FilePreview>,
    pub extra_folders: Vec<FilePreview>,
    pub warnings: Vec<WarningItem>,
    pub can_start: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeGroupPreview {
    pub id: String,
    pub base_name: String,
    pub first_volume_path: String,
    pub volume_count: u32,
    pub total_size: u64,
    pub missing_indexes: Vec<u32>,
    pub duplicate_indexes: Vec<u32>,
    pub files: Vec<VolumeFilePreview>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeFilePreview {
    pub path: String,
    pub index: u32,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilePreview {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WarningItem {
    pub code: String,
    pub message: String,
    pub detail: Option<String>,
}

#[tauri::command]
pub async fn get_app_config() -> Result<serde_json::Value, AppError> {
    Ok(serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

#[tauri::command]
pub async fn load_passwords() -> Result<PasswordConfig, AppError> {
    // TODO: 从应用数据目录加载密码配置
    Ok(PasswordConfig::default())
}

#[tauri::command]
pub async fn save_passwords(config: PasswordConfig) -> Result<(), AppError> {
    // TODO: 保存密码配置到应用数据目录
    Ok(())
}

#[tauri::command]
pub async fn preview_task(input: TaskInput) -> Result<TaskPreview, AppError> {
    // TODO: 实现预检查逻辑
    Err(AppError::Unknown("未实现".to_string()))
}

#[tauri::command]
pub async fn start_task(input: TaskInput, app: AppHandle) -> Result<(), AppError> {
    // TODO: 实现任务启动逻辑
    Err(AppError::Unknown("未实现".to_string()))
}

#[tauri::command]
pub async fn cancel_task() -> Result<(), AppError> {
    // TODO: 实现任务取消逻辑
    Err(AppError::Unknown("未实现".to_string()))
}

#[tauri::command]
pub async fn open_log_folder() -> Result<(), AppError> {
    // TODO: 打开日志文件夹
    Err(AppError::Unknown("未实现".to_string()))
}
