/**
 * 日志事件定义
 * 基于 design.md 第 10.2 节
 */
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEvent {
    pub time: String,
    pub level: LogLevel,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl LogEvent {
    pub fn info(message: impl Into<String>, detail: Option<String>) -> Self {
        Self {
            time: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            level: LogLevel::Info,
            message: message.into(),
            detail,
        }
    }

    pub fn success(message: impl Into<String>, detail: Option<String>) -> Self {
        Self {
            time: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            level: LogLevel::Success,
            message: message.into(),
            detail,
        }
    }

    pub fn warning(message: impl Into<String>, detail: Option<String>) -> Self {
        Self {
            time: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            level: LogLevel::Warning,
            message: message.into(),
            detail,
        }
    }

    pub fn error(message: impl Into<String>, detail: Option<String>) -> Self {
        Self {
            time: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            level: LogLevel::Error,
            message: message.into(),
            detail,
        }
    }
}
