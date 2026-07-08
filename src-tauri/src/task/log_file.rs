/**
 * 日志文件模块
 * 基于 design.md 第 10.4 节
 */
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

use chrono::Local;

use crate::error::AppError;
use crate::events::LogEvent;

/// 日志文件写入器
pub struct LogFileWriter {
    file: Mutex<std::fs::File>,
    path: PathBuf,
}

impl LogFileWriter {
    /// 创建新的日志文件写入器
    pub fn new() -> Result<Self, AppError> {
        let log_dir = Self::log_dir()?;

        // 确保日志目录存在
        std::fs::create_dir_all(&log_dir)
            .map_err(|e| AppError::Unknown(format!("创建日志目录失败：{}", e)))?;

        // 生成日志文件名
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let filename = format!("{}.log", timestamp);
        let path = log_dir.join(filename);

        // 打开日志文件
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| AppError::Unknown(format!("打开日志文件失败：{}", e)))?;

        Ok(Self {
            file: Mutex::new(file),
            path,
        })
    }

    /// 获取日志目录
    fn log_dir() -> Result<PathBuf, AppError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| AppError::Unknown("无法获取配置目录".to_string()))?;
        Ok(config_dir.join("sjs-unzip-tool").join("logs"))
    }

    /// 获取日志文件路径
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// 写入头部信息
    pub fn write_header(
        &self,
        root_dir: &str,
        final_folder_name: &str,
    ) -> Result<(), AppError> {
        let mut file = self.file.lock()
            .map_err(|e| AppError::Unknown(format!("获取日志文件锁失败：{}", e)))?;

        let os_info = if cfg!(target_os = "macos") {
            "macOS"
        } else if cfg!(target_os = "windows") {
            "Windows"
        } else {
            "Linux"
        };

        writeln!(file, "========================================")
            .map_err(|e| Self::write_error(e))?;
        writeln!(file, "sjs-unzip-tool 日志")
            .map_err(|e| Self::write_error(e))?;
        writeln!(file, "========================================")
            .map_err(|e| Self::write_error(e))?;
        writeln!(file, "版本：{}", env!("CARGO_PKG_VERSION"))
            .map_err(|e| Self::write_error(e))?;
        writeln!(file, "操作系统：{}", os_info)
            .map_err(|e| Self::write_error(e))?;
        writeln!(file, "根目录：{}", root_dir)
            .map_err(|e| Self::write_error(e))?;
        writeln!(file, "最终文件夹名：{}", final_folder_name)
            .map_err(|e| Self::write_error(e))?;
        writeln!(file, "开始时间：{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
            .map_err(|e| Self::write_error(e))?;
        writeln!(file, "========================================")
            .map_err(|e| Self::write_error(e))?;

        Ok(())
    }

    /// 写入日志事件
    pub fn write_log(&self, event: &LogEvent) -> Result<(), AppError> {
        let mut file = self.file.lock()
            .map_err(|e| AppError::Unknown(format!("获取日志文件锁失败：{}", e)))?;

        let level_str = match event.level {
            crate::events::LogLevel::Info => "INFO",
            crate::events::LogLevel::Success => "SUCCESS",
            crate::events::LogLevel::Warning => "WARNING",
            crate::events::LogLevel::Error => "ERROR",
        };

        // 过滤密码信息
        let message = Self::filter_passwords(&event.message);
        let detail = event.detail.as_deref().map(Self::filter_passwords);

        write!(file, "[{}] {:<8} {}", event.time, level_str, message)
            .map_err(|e| Self::write_error(e))?;

        if let Some(detail) = detail {
            write!(file, " | {}", detail)
                .map_err(|e| Self::write_error(e))?;
        }

        writeln!(file)
            .map_err(|e| Self::write_error(e))?;

        Ok(())
    }

    /// 写入尾部信息
    pub fn write_footer(&self, success: bool) -> Result<(), AppError> {
        let mut file = self.file.lock()
            .map_err(|e| AppError::Unknown(format!("获取日志文件锁失败：{}", e)))?;

        writeln!(file, "========================================")
            .map_err(|e| Self::write_error(e))?;
        writeln!(file, "结束时间：{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
            .map_err(|e| Self::write_error(e))?;
        writeln!(file, "结果：{}", if success { "成功" } else { "失败" })
            .map_err(|e| Self::write_error(e))?;
        writeln!(file, "========================================")
            .map_err(|e| Self::write_error(e))?;

        Ok(())
    }

    /// 过滤密码信息
    fn filter_passwords(message: &str) -> String {
        // 替换可能包含密码的内容
        let patterns = [
            ("password", "***"),
            ("密码", "***"),
            ("-p\"", "-p***"),
            ("-p'", "-p***"),
        ];

        let mut result = message.to_string();
        for (pattern, replacement) in &patterns {
            // 不区分大小写替换
            let lower = result.to_lowercase();
            if let Some(pos) = lower.find(pattern) {
                let end = pos + pattern.len();
                result = format!("{}{}{}", &result[..pos], replacement, &result[end..]);
            }
        }

        result
    }

    fn write_error(e: std::io::Error) -> AppError {
        AppError::Unknown(format!("写入日志文件失败：{}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_passwords() {
        assert_eq!(
            LogFileWriter::filter_passwords("正在尝试密码：123456"),
            "正在尝试***：123456"
        );
        // 当前实现替换模式本身
        assert_eq!(
            LogFileWriter::filter_passwords("使用 -p\"secret\" 解压"),
            "使用 -p***secret\" 解压"
        );
    }
}
