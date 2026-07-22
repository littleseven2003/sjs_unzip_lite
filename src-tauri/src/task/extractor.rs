/**
 * 解压模块
 * 基于 design.md 第 11 节
 */
use std::path::{Path, PathBuf};

use tauri::{AppHandle, Manager};

use crate::error::AppError;

/// 解压结果
#[derive(Debug)]
pub struct ExtractResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

/// 校验压缩包是否有效
pub async fn validate_archive(app: &AppHandle, archive_path: &Path) -> Result<bool, AppError> {
    let bin_path = get_7zz_path(app)?;

    let mut cmd = tokio::process::Command::new(&bin_path);
    cmd.arg("l") // list 命令
        .arg(archive_path);

    // 重定向 stdout 和 stderr
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    let output = cmd
        .output()
        .await
        .map_err(|e| AppError::ExtractFailed(format!("校验压缩包失败：{}", e)))?;

    // 7zz l 命令成功表示文件是有效压缩包
    Ok(output.status.success())
}

/// 使用密码列表尝试解压
pub async fn extract_with_passwords(
    app: &AppHandle,
    archive_path: &Path,
    output_dir: &Path,
    passwords: &[String],
) -> Result<usize, AppError> {
    // 创建临时目录
    let temp_dir = output_dir.join(".sjs_unzip_temp");
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| AppError::ExtractFailed(format!("创建临时目录失败：{}", e)))?;

    for (index, password) in passwords.iter().enumerate() {
        let result = run_7zz_extract(app, archive_path, &temp_dir, password).await?;

        if result.success {
            // 解压成功，移动到目标目录
            move_contents(&temp_dir, output_dir)?;
            let _ = std::fs::remove_dir_all(&temp_dir);
            return Ok(index);
        }

        if is_wrong_password(&result.stderr, &result.stdout, result.exit_code) {
            // 密码错误，继续尝试
            let _ = std::fs::remove_dir_all(&temp_dir);
            std::fs::create_dir_all(&temp_dir)
                .map_err(|e| AppError::ExtractFailed(format!("创建临时目录失败：{}", e)))?;
            continue;
        }

        // 其他错误，停止
        let _ = std::fs::remove_dir_all(&temp_dir);
        let error_msg = if result.stderr.is_empty() {
            format!(
                "7zz退出码：{:?}, stdout: {}",
                result.exit_code, result.stdout
            )
        } else {
            result.stderr
        };
        return Err(AppError::ExtractFailed(error_msg));
    }

    let _ = std::fs::remove_dir_all(&temp_dir);
    Err(AppError::PasswordFailed)
}

/// 调用 7zz 解压
async fn run_7zz_extract(
    app: &AppHandle,
    archive_path: &Path,
    output_dir: &Path,
    password: &str,
) -> Result<ExtractResult, AppError> {
    let bin_path = get_7zz_path(app)?;

    // 检查归档文件是否存在
    if !archive_path.exists() {
        return Err(AppError::ExtractFailed(format!(
            "归档文件不存在：{}",
            archive_path.display()
        )));
    }

    let mut cmd = tokio::process::Command::new(&bin_path);
    cmd.arg("x")
        .arg(archive_path)
        .arg(format!("-o{}", output_dir.display()))
        .arg("-y");

    // 传递密码参数（非空时）
    if !password.is_empty() {
        cmd.arg(format!("-p{}", password));
    }

    // 重定向 stdout 和 stderr 以便捕获
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    // 启动子进程
    let child = cmd
        .spawn()
        .map_err(|e| AppError::ExtractFailed(format!("启动7zz失败：{}", e)))?;

    // 注册子进程 ID 到取消模块
    if let Some(pid) = child.id() {
        super::cancel::register_child_pid(pid);
    }

    // 等待进程完成
    let output = child
        .wait_with_output()
        .await
        .map_err(|e| AppError::ExtractFailed(format!("等待7zz失败：{}", e)))?;

    // 注销子进程
    super::cancel::unregister_child_pid();

    // 检查是否被取消
    super::cancel::check_cancelled()?;

    // 构建详细的错误信息
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    Ok(ExtractResult {
        success: output.status.success(),
        stdout,
        stderr,
        exit_code: output.status.code(),
    })
}

/// 获取 7zz 路径
fn get_7zz_path(app: &AppHandle) -> Result<PathBuf, AppError> {
    // 构建 sidecar 路径
    #[cfg(target_os = "macos")]
    let bin_name = if cfg!(target_arch = "aarch64") {
        "7zz-aarch64-apple-darwin"
    } else {
        "7zz-x86_64-apple-darwin"
    };

    #[cfg(target_os = "windows")]
    let bin_name = "7zz-x86_64-pc-windows-msvc.exe";

    #[cfg(target_os = "linux")]
    let bin_name = "7zz-x86_64-unknown-linux-gnu";

    // 尝试多种路径
    let possible_paths = vec![
        // 1. Tauri sidecar 路径（生产环境）
        app.path()
            .resource_dir()
            .ok()
            .map(|p| p.join("binaries").join(bin_name)),
        // 2. 当前目录下的 src-tauri/binaries
        std::env::current_dir()
            .ok()
            .map(|p| p.join("src-tauri").join("binaries").join(bin_name)),
        // 3. 当前目录下的 binaries
        std::env::current_dir()
            .ok()
            .map(|p| p.join("binaries").join(bin_name)),
        // 4. 可执行文件同级目录
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.join(bin_name))),
    ];

    // 查找第一个存在的路径
    for path in possible_paths.into_iter().flatten() {
        if path.exists() {
            return Ok(path);
        }
    }

    Err(AppError::ExtractToolNotFound)
}

/// 判断是否为密码错误
fn is_wrong_password(stderr: &str, stdout: &str, exit_code: Option<i32>) -> bool {
    let combined = format!("{} {}", stderr, stdout).to_lowercase();

    // 检查输出中的密码错误信息
    if combined.contains("wrong password")
        || combined.contains("data error in encrypted file")
        || combined.contains("can not open encrypted archive")
        || combined.contains("headers error")
    {
        return true;
    }

    // 退出码 255 通常是用户中断或密码问题
    if exit_code == Some(255) {
        return true;
    }

    false
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
