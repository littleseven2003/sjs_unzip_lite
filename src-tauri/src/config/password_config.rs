/**
 * 密码配置管理
 * 基于 design.md 第 5 节
 */
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordConfig {
    pub version: u32,
    pub passwords: Vec<String>,
    pub updated_at: String,
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            version: 1,
            passwords: vec![String::new()],
            updated_at: chrono::Local::now().to_rfc3339(),
        }
    }
}

impl PasswordConfig {
    /// 获取配置文件路径
    pub fn config_path() -> Result<PathBuf, AppError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| AppError::Unknown("无法获取配置目录".to_string()))?;
        let app_dir = config_dir.join("sjs-unzip-tool");
        Ok(app_dir.join("passwords.json"))
    }

    /// 从文件加载配置
    pub fn load() -> Result<Self, AppError> {
        let path = Self::config_path()?;

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(&path)
            .map_err(|e| AppError::Unknown(format!("读取密码配置失败：{}", e)))?;

        let config: Self = serde_json::from_str(&content)
            .map_err(|e| AppError::Unknown(format!("解析密码配置失败：{}", e)))?;

        Ok(config)
    }

    /// 保存配置到文件
    pub fn save(&self) -> Result<(), AppError> {
        let path = Self::config_path()?;

        // 确保目录存在
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::Unknown(format!("创建配置目录失败：{}", e)))?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| AppError::Unknown(format!("序列化密码配置失败：{}", e)))?;

        std::fs::write(&path, content)
            .map_err(|e| AppError::Unknown(format!("保存密码配置失败：{}", e)))?;

        Ok(())
    }

    /// 获取规范化后的密码列表（去重空密码，保留顺序）
    pub fn normalized_passwords(&self) -> Vec<String> {
        let mut result = Vec::new();
        let mut has_empty = false;

        for pwd in &self.passwords {
            if pwd.is_empty() {
                if !has_empty {
                    result.push(String::new());
                    has_empty = true;
                }
            } else {
                result.push(pwd.clone());
            }
        }

        if !has_empty {
            result.insert(0, String::new());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = PasswordConfig::default();
        assert_eq!(config.version, 1);
        assert_eq!(config.passwords.len(), 1);
        assert_eq!(config.passwords[0], "");
    }

    #[test]
    fn test_normalized_passwords() {
        let config = PasswordConfig {
            version: 1,
            passwords: vec!["".to_string(), "123".to_string(), "".to_string(), "456".to_string()],
            updated_at: chrono::Local::now().to_rfc3339(),
        };
        let normalized = config.normalized_passwords();
        assert_eq!(normalized, vec!["", "123", "456"]);
    }

    #[test]
    fn test_normalized_without_empty() {
        let config = PasswordConfig {
            version: 1,
            passwords: vec!["123".to_string(), "456".to_string()],
            updated_at: chrono::Local::now().to_rfc3339(),
        };
        let normalized = config.normalized_passwords();
        assert_eq!(normalized, vec!["", "123", "456"]);
    }
}
