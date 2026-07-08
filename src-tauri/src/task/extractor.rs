/**
 * 解压模块
 * 基于 design.md 第 11 节
 */
use std::path::{Path, PathBuf};

use crate::error::AppError;

/// 解压结果
#[derive(Debug)]
pub struct ExtractResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

/// 使用密码列表尝试解压
pub async fn extract_with_passwords(
    archive_path: &Path,
    output_dir: &Path,
    passwords: &[String],
) -> Result<usize, AppError> {
    // 创建临时目录
    let temp_dir = output_dir.join(".sjs_unzip_temp");
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| AppError::ExtractFailed(format!("创建临时目录失败：{}", e)))?;

    for (index, password) in passwords.iter().enumerate() {
        let result = run_7zz_extract(archive_path, &temp_dir, password).await?;

        if result.success {
            // 解压成功，移动到目标目录
            move_contents(&temp_dir, output_dir)?;
            let _ = std::fs::remove_dir_all(&temp_dir);
            return Ok(index);
        }

        if is_wrong_password(&result.stderr) {
            // 密码错误，继续尝试
            let _ = std::fs::remove_dir_all(&temp_dir);
            std::fs::create_dir_all(&temp_dir)
                .map_err(|e| AppError::ExtractFailed(format!("创建临时目录失败：{}", e)))?;
            continue;
        }

        // 其他错误，停止
        let _ = std::fs::remove_dir_all(&temp_dir);
        return Err(AppError::ExtractFailed(result.stderr));
    }

    let _ = std::fs::remove_dir_all(&temp_dir);
    Err(AppError::PasswordFailed)
}

/// 调用 7zz 解压
async fn run_7zz_extract(
    archive_path: &Path,
    output_dir: &Path,
    password: &str,
) -> Result<ExtractResult, AppError> {
    let bin_path = get_7zz_path()?;

    let mut cmd = tokio::process::Command::new(&bin_path);
    cmd.arg("x")
        .arg(archive_path)
        .arg(format!("-o{}", output_dir.display()))
        .arg("-y");

    if !password.is_empty() {
        cmd.arg(format!("-p{}", password));
    }

    // 启动子进程
    let mut child = cmd
        .spawn()
        .map_err(|e| AppError::ExtractFailed(e.to_string()))?;

    // 注册子进程 ID 到取消模块
    if let Some(pid) = child.id() {
        super::cancel::register_child_pid(pid);
    }

    // 等待进程完成
    let output = child
        .wait_with_output()
        .await
        .map_err(|e| AppError::ExtractFailed(e.to_string()))?;

    // 注销子进程
    super::cancel::unregister_child_pid();

    // 检查是否被取消
    super::cancel::check_cancelled()?;

    Ok(ExtractResult {
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code(),
    })
}

/// 获取 7zz 路径
fn get_7zz_path() -> Result<PathBuf, AppError> {
    // Tauri sidecar 路径由运行时决定，此处为占位
    // 实际使用时通过 tauri::api::path::resource_dir 获取
    Err(AppError::ExtractToolNotFound)
}

/// 判断是否为密码错误
fn is_wrong_password(stderr: &str) -> bool {
    let lower = stderr.to_lowercase();
    lower.contains("wrong password")
        || lower.contains("can not open encrypted archive")
        || lower.contains("headers error")
}

/// 移动目录内容到目标
fn move_contents(src: &Path, dest: &Path) -> Result<(), AppError> {
    for entry in std::fs::read_dir(src).map_err(|e| AppError::MoveFailed(e.to_string()))? {
        let entry = entry.map_err(|e| AppError::MoveFailed(e.to_string()))?;
        let target = dest.join(entry.file_name());
        std::fs::rename(entry.path(), &target).map_err(|e| AppError::MoveFailed(e.to_string()))?;
    }
    Ok(())
}
