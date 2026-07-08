/**
 * 密码配置管理
 * 基于 design.md 第 5 节
 */
use serde::{Deserialize, Serialize};

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
