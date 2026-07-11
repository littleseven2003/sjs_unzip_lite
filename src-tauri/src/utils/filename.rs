/**
 * 文件名工具函数
 * 基于 design.md 第 13.2 节
 */
/// Windows 保留名称
const WINDOWS_RESERVED_NAMES: &[&str] = &[
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
    "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

/// 校验文件夹名是否合法
/// 返回错误信息，合法时返回 None
pub fn validate_folder_name(name: &str) -> Option<String> {
    let trimmed = name.trim();

    if trimmed.is_empty() {
        return Some("文件夹名不能为空".to_string());
    }

    if trimmed == "." || trimmed == ".." {
        return Some("文件夹名不能为 . 或 ..".to_string());
    }

    if trimmed.contains('/') || trimmed.contains('\\') {
        return Some("文件夹名不能包含路径分隔符".to_string());
    }

    if trimmed.contains('<')
        || trimmed.contains('>')
        || trimmed.contains(':')
        || trimmed.contains('"')
        || trimmed.contains('|')
        || trimmed.contains('?')
        || trimmed.contains('*')
    {
        return Some("文件夹名包含非法字符".to_string());
    }

    let upper = trimmed.to_uppercase();
    if WINDOWS_RESERVED_NAMES.contains(&upper.as_str()) {
        return Some("文件夹名为系统保留名称".to_string());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_name() {
        assert_eq!(validate_folder_name("my_folder"), None);
    }

    #[test]
    fn test_empty_name() {
        assert!(validate_folder_name("").is_some());
        assert!(validate_folder_name("  ").is_some());
    }

    #[test]
    fn test_dot_name() {
        assert!(validate_folder_name(".").is_some());
        assert!(validate_folder_name("..").is_some());
    }

    #[test]
    fn test_path_separator() {
        assert!(validate_folder_name("a/b").is_some());
        assert!(validate_folder_name("a\\b").is_some());
    }

    #[test]
    fn test_invalid_chars() {
        assert!(validate_folder_name("a<b").is_some());
        assert!(validate_folder_name("a>b").is_some());
        assert!(validate_folder_name("a:b").is_some());
    }

    #[test]
    fn test_reserved_name() {
        assert!(validate_folder_name("CON").is_some());
        assert!(validate_folder_name("con").is_some());
        assert!(validate_folder_name("COM1").is_some());
    }
}
