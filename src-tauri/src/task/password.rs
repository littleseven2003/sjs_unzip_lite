/**
 * 密码尝试模块
 * 基于 design.md 第 11.3 节
 */

/// 密码规范化：去重空密码，确保至少有一个空密码
pub fn normalize_passwords(passwords: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    let mut has_empty = false;

    for pwd in passwords {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_with_empty() {
        let passwords = vec!["".to_string(), "123".to_string(), "".to_string()];
        let result = normalize_passwords(&passwords);
        assert_eq!(result, vec!["", "123"]);
    }

    #[test]
    fn test_normalize_without_empty() {
        let passwords = vec!["123".to_string(), "456".to_string()];
        let result = normalize_passwords(&passwords);
        assert_eq!(result, vec!["", "123", "456"]);
    }
}
