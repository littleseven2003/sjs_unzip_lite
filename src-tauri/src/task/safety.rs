/**
 * 安全校验模块
 * 基于 design.md 第 7 节
 */
use std::path::Path;

use crate::error::AppError;

/// 危险目录列表
const DANGEROUS_PATHS: &[&str] = &[
    "/",
    "/Users",
    "/Applications",
    "/System",
    "/Library",
    "C:\\",
    "C:\\Windows",
    "C:\\Program Files",
    "C:\\Program Files (x86)",
    "C:\\Users",
];

/// 校验根目录是否安全
pub fn validate_root_dir(root_dir: &Path) -> Result<(), AppError> {
    if !root_dir.exists() {
        return Err(AppError::InvalidRootDir(root_dir.display().to_string()));
    }

    if !root_dir.is_dir() {
        return Err(AppError::InvalidRootDir(root_dir.display().to_string()));
    }

    let normalized = root_dir
        .to_string_lossy()
        .replace('\\', "/")
        .to_lowercase();

    for dangerous in DANGEROUS_PATHS {
        if normalized == dangerous.to_lowercase() || normalized == dangerous.to_lowercase() + "/" {
            return Err(AppError::DangerousRootDir(root_dir.display().to_string()));
        }
    }

    Ok(())
}

/// 校验路径在工作目录内
pub fn validate_path_in_root(path: &Path, root_dir: &Path) -> Result<(), AppError> {
    let canonical_root = root_dir
        .canonicalize()
        .map_err(|e| AppError::Unknown(e.to_string()))?;
    let canonical_path = path
        .canonicalize()
        .map_err(|e| AppError::Unknown(e.to_string()))?;

    if !canonical_path.starts_with(&canonical_root) {
        return Err(AppError::DangerousRootDir(format!(
            "路径不在工作目录内：{}",
            path.display()
        )));
    }

    Ok(())
}

/// 检查是否为符号链接
pub fn is_symlink(path: &Path) -> bool {
    path.symlink_metadata()
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_validate_normal_dir() {
        let tmp = TempDir::new().unwrap();
        assert!(validate_root_dir(tmp.path()).is_ok());
    }

    #[test]
    fn test_validate_path_in_root() {
        let tmp = TempDir::new().unwrap();
        let file = tmp.path().join("file.txt");
        std::fs::write(&file, b"data").unwrap();
        assert!(validate_path_in_root(&file, tmp.path()).is_ok());
    }
}
