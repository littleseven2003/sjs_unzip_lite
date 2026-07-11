/**
 * Tauri 命令定义
 * 基于 design.md 第 9.1 节
 */
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::config::PasswordConfig;
use crate::error::AppError;
use crate::task::safety;
use crate::task::scanner;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskInput {
    pub root_dir: String,
    pub final_folder_name: Option<String>,
    pub continue_on_initial_extra_files: bool,
    pub selected_volume_group_id: Option<String>,
    pub selected_txt_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct VolumeFilePreview {
    pub path: String,
    pub index: u32,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilePreview {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    PasswordConfig::load()
}

#[tauri::command]
pub async fn save_passwords(config: PasswordConfig) -> Result<(), AppError> {
    config.save()
}

#[tauri::command]
pub async fn preview_task(input: TaskInput) -> Result<TaskPreview, AppError> {
    let root_dir = PathBuf::from(&input.root_dir);

    // 安全校验
    safety::validate_root_dir(&root_dir)?;

    // 扫描目录
    let scan_result = scanner::scan_root_recursively(&root_dir)?;

    // 构建默认最终文件夹名
    let default_final_folder_name = root_dir
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    // 构建分卷组预览
    let volume_groups: Vec<VolumeGroupPreview> = scan_result
        .volume_groups
        .iter()
        .map(|group| {
            let first_volume_path = group
                .files
                .first()
                .map(|f| f.path.to_string_lossy().to_string())
                .unwrap_or_default();

            let total_size: u64 = group.files.iter().map(|f| f.size).sum();

            // 检查缺失编号
            let missing_indexes = scanner::missing_indexes_of(
                &group.files.iter().map(|f| f.index).collect::<Vec<u32>>(),
            );

            // 检查重复编号
            let mut indexes: Vec<u32> = group.files.iter().map(|f| f.index).collect();
            indexes.sort();
            let mut seen = std::collections::HashSet::new();
            let mut duplicate_indexes = Vec::new();
            for idx in &indexes {
                if !seen.insert(idx) {
                    duplicate_indexes.push(*idx);
                }
            }

            VolumeGroupPreview {
                id: group.id.clone(),
                base_name: group.base_name.clone(),
                first_volume_path,
                volume_count: group.files.len() as u32,
                total_size,
                missing_indexes,
                duplicate_indexes,
                files: group
                    .files
                    .iter()
                    .map(|f| VolumeFilePreview {
                        path: f.path.to_string_lossy().to_string(),
                        index: f.index,
                        size: f.size,
                    })
                    .collect(),
            }
        })
        .collect();

    // 构建额外文件预览
    let extra_files: Vec<FilePreview> = scan_result
        .extra_files
        .iter()
        .map(|path| {
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            let size = path.metadata().map(|m| m.len()).unwrap_or(0);
            FilePreview {
                path: path.to_string_lossy().to_string(),
                name,
                size,
                is_dir: false,
            }
        })
        .collect();

    let extra_folders: Vec<FilePreview> = scan_result
        .extra_dirs
        .iter()
        .map(|path| {
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            FilePreview {
                path: path.to_string_lossy().to_string(),
                name,
                size: 0,
                is_dir: true,
            }
        })
        .collect();

    // 构建警告
    let mut warnings = Vec::new();

    if volume_groups.is_empty() {
        warnings.push(WarningItem {
            code: "NO_VOLUMES".to_string(),
            message: "未找到 7z 分卷文件，请确认选择的文件夹是否正确。".to_string(),
            detail: None,
        });
    }

    if volume_groups.len() > 1 {
        warnings.push(WarningItem {
            code: "MULTIPLE_GROUPS".to_string(),
            message: format!("找到 {} 组分卷文件，将需要选择处理其中一组。", volume_groups.len()),
            detail: None,
        });
    }

    for group in &volume_groups {
        if !group.missing_indexes.is_empty() {
            warnings.push(WarningItem {
                code: "MISSING_VOLUMES".to_string(),
                message: format!(
                    "分卷组「{}」缺少编号：{:?}",
                    group.base_name, group.missing_indexes
                ),
                detail: None,
            });
        }
        if !group.duplicate_indexes.is_empty() {
            warnings.push(WarningItem {
                code: "DUPLICATE_VOLUMES".to_string(),
                message: format!(
                    "分卷组「{}」存在重复编号：{:?}",
                    group.base_name, group.duplicate_indexes
                ),
                detail: None,
            });
        }
    }

    if !extra_files.is_empty() {
        warnings.push(WarningItem {
            code: "EXTRA_FILES".to_string(),
            message: format!("检测到 {} 个额外文件，继续处理可能会在后续清理步骤中删除这些内容。", extra_files.len()),
            detail: None,
        });
    }

    // 判断是否可以开始
    let can_start = !volume_groups.is_empty()
        && volume_groups.iter().all(|g| g.missing_indexes.is_empty())
        && volume_groups.len() <= 1
        && (extra_files.is_empty() || input.continue_on_initial_extra_files);

    Ok(TaskPreview {
        root_dir: input.root_dir,
        default_final_folder_name,
        volume_groups,
        extra_files,
        extra_folders,
        warnings,
        can_start,
    })
}

#[tauri::command]
pub async fn start_task(input: TaskInput, app: AppHandle) -> Result<(), AppError> {
    let root_dir = PathBuf::from(&input.root_dir);

    // 安全校验
    safety::validate_root_dir(&root_dir)?;

    // 加载密码配置
    let password_config = PasswordConfig::load()?;
    let passwords = password_config.normalized_passwords();

    // 确定最终文件夹名
    let final_folder_name = input.final_folder_name.clone().unwrap_or_else(|| {
        root_dir
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default()
    });

    // 创建任务上下文
    let mut ctx = crate::task::context::TaskContext::new(root_dir, final_folder_name, passwords);

    // 执行任务
    crate::task::runner::run_task(&mut ctx, &app).await
}

#[tauri::command]
pub async fn cancel_task(app: AppHandle) -> Result<(), AppError> {
    crate::task::cancel::request_cancel();

    // 发送取消事件到前端
    use tauri::Emitter;
    let _ = app.emit("task-log", crate::events::LogEvent::warning("用户请求取消任务", None));

    Ok(())
}

#[tauri::command]
pub async fn open_folder(path: String) -> Result<(), AppError> {
    let folder_path = std::path::PathBuf::from(&path);

    if !folder_path.exists() {
        return Err(AppError::Unknown(format!("目录不存在：{}", path)));
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&folder_path)
            .spawn()
            .map_err(|e| AppError::Unknown(format!("打开目录失败：{}", e)))?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&folder_path)
            .spawn()
            .map_err(|e| AppError::Unknown(format!("打开目录失败：{}", e)))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&folder_path)
            .spawn()
            .map_err(|e| AppError::Unknown(format!("打开目录失败：{}", e)))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn open_log_folder() -> Result<(), AppError> {
    let log_dir = dirs::config_dir()
        .ok_or_else(|| AppError::Unknown("无法获取配置目录".to_string()))?
        .join("sjs-unzip-tool")
        .join("logs");

    // 确保目录存在
    std::fs::create_dir_all(&log_dir)
        .map_err(|e| AppError::Unknown(format!("创建日志目录失败：{}", e)))?;

    // 打开目录
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&log_dir)
            .spawn()
            .map_err(|e| AppError::Unknown(format!("打开目录失败：{}", e)))?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&log_dir)
            .spawn()
            .map_err(|e| AppError::Unknown(format!("打开目录失败：{}", e)))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&log_dir)
            .spawn()
            .map_err(|e| AppError::Unknown(format!("打开目录失败：{}", e)))?;
    }

    Ok(())
}
