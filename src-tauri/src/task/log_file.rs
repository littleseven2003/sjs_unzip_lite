/**
 * 日志文件模块
 * 基于 design.md 第 10.4 节
 */
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

use chrono::Local;
use regex::Regex;

use crate::error::AppError;
use crate::events::LogEvent;

/// 日志文件写入器
pub struct LogFileWriter {
    file: Mutex<std::fs::File>,
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
        })
    }

    /// 获取日志目录
    fn log_dir() -> Result<PathBuf, AppError> {
        let config_dir =
            dirs::config_dir().ok_or_else(|| AppError::Unknown("无法获取配置目录".to_string()))?;
        Ok(config_dir.join("sjs-unzip-tool").join("logs"))
    }

    /// 写入头部信息
    pub fn write_header(&self, root_dir: &str, final_folder_name: &str) -> Result<(), AppError> {
        let mut file = self
            .file
            .lock()
            .map_err(|e| AppError::Unknown(format!("获取日志文件锁失败：{}", e)))?;

        let os_info = if cfg!(target_os = "macos") {
            "macOS"
        } else if cfg!(target_os = "windows") {
            "Windows"
        } else {
            "Linux"
        };

        writeln!(file, "========================================").map_err(Self::write_error)?;
        writeln!(file, "sjs-unzip-tool 日志").map_err(Self::write_error)?;
        writeln!(file, "========================================").map_err(Self::write_error)?;
        writeln!(file, "版本：{}", env!("CARGO_PKG_VERSION")).map_err(Self::write_error)?;
        writeln!(file, "操作系统：{}", os_info).map_err(Self::write_error)?;
        writeln!(file, "根目录：{}", root_dir).map_err(Self::write_error)?;
        writeln!(file, "最终文件夹名：{}", final_folder_name).map_err(Self::write_error)?;
        writeln!(
            file,
            "开始时间：{}",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        )
        .map_err(Self::write_error)?;
        writeln!(file, "========================================").map_err(Self::write_error)?;

        Ok(())
    }

    /// 写入日志事件
    pub fn write_log(&self, event: &LogEvent) -> Result<(), AppError> {
        let mut file = self
            .file
            .lock()
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

        write!(file, "[{}] {:<8} {}", event.time, level_str, message).map_err(Self::write_error)?;

        if let Some(detail) = detail {
            write!(file, " | {}", detail).map_err(Self::write_error)?;
        }

        writeln!(file).map_err(Self::write_error)?;

        Ok(())
    }

    /// 写入尾部信息
    pub fn write_footer(&self, success: bool) -> Result<(), AppError> {
        let mut file = self
            .file
            .lock()
            .map_err(|e| AppError::Unknown(format!("获取日志文件锁失败：{}", e)))?;

        writeln!(file, "========================================").map_err(Self::write_error)?;
        writeln!(
            file,
            "结束时间：{}",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        )
        .map_err(Self::write_error)?;
        writeln!(file, "结果：{}", if success { "成功" } else { "失败" })
            .map_err(Self::write_error)?;
        writeln!(file, "========================================").map_err(Self::write_error)?;

        Ok(())
    }

    /// 过滤密码信息
    ///
    /// 遮蔽 7-Zip 命令行中携带的密码参数：`-p"secret"`、`-p'secret'`、`-psecret`，
    /// 统一替换为 `-p***`，避免密码明文落入日志文件。
    fn filter_passwords(message: &str) -> String {
        use std::sync::OnceLock;

        static RE: OnceLock<Regex> = OnceLock::new();
        let re = RE.get_or_init(|| {
            // 三种形态依次匹配：
            //   -p"..."   双引号包裹的密码
            //   -p'...'   单引号包裹的密码
            //   -p<token> 无引号、延续到下一个空白或字符串末尾的密码
            Regex::new(r#"-p"(?:[^"\\]|\\.)*"|-p'(?:[^'\\]|\\.)*'|-p[^\s]*"#).unwrap()
        });

        re.replace_all(message, "-p***").to_string()
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
        // 双引号包裹的密码：整段替换为 -p***
        assert_eq!(
            LogFileWriter::filter_passwords(r#"7zz x a.rar -o"/root" -y -p"secret""#),
            r#"7zz x a.rar -o"/root" -y -p***"#
        );
        // 单引号包裹的密码
        assert_eq!(
            LogFileWriter::filter_passwords(r#"7zz x a.rar -p'secret' -y"#),
            r#"7zz x a.rar -p*** -y"#
        );
        // 无引号、延续到空白为止的密码
        assert_eq!(
            LogFileWriter::filter_passwords("7zz x a.rar -psecret -y"),
            "7zz x a.rar -p*** -y"
        );
        // 多个密码参数一并遮蔽
        assert_eq!(
            LogFileWriter::filter_passwords(r#"-p"a" then -p"b""#),
            "-p*** then -p***"
        );
        // 无密码参数时保持原样
        assert_eq!(
            LogFileWriter::filter_passwords("正在尝试密码序号：3"),
            "正在尝试密码序号：3"
        );
    }
}
