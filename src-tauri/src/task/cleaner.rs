/**
 * 清理模块
 * 基于 design.md 第 12 节
 */
use std::path::{Path, PathBuf};

use crate::error::AppError;

/// 清理目录，保留指定文件
pub fn clean_root_except(root_dir: &Path, keep_paths: &[PathBuf]) -> Result<(), AppError> {
    for entry in std::fs::read_dir(root_dir).map_err(|e| AppError::DeleteFailed(e.to_string()))? {
        let entry = entry.map_err(|e| AppError::DeleteFailed(e.to_string()))?;
        let path = entry.path();

        // 安全校验：确保路径在根目录内
        if !path.starts_with(root_dir) {
            return Err(AppError::DeleteFailed(format!(
                "路径越界：{}",
                path.display()
            )));
        }

        // 跳过需要保留的文件
        if keep_paths.iter().any(|k| k == &path) {
            continue;
        }

        // 跳过临时目录
        if path.file_name().map(|n| n == ".sjs_unzip_temp").unwrap_or(false) {
            continue;
        }

        if path.is_dir() {
            std::fs::remove_dir_all(&path)
                .map_err(|e| AppError::DeleteFailed(e.to_string()))?;
        } else {
            std::fs::remove_file(&path).map_err(|e| AppError::DeleteFailed(e.to_string()))?;
        }
    }

    Ok(())
}

/// 删除单个文件
pub fn delete_file(path: &Path) -> Result<(), AppError> {
    if path.exists() {
        std::fs::remove_file(path).map_err(|e| AppError::DeleteFailed(e.to_string()))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_clean_except_keeps_specified() {
        let tmp = TempDir::new().unwrap();
        let keep = tmp.path().join("keep.txt");
        let remove = tmp.path().join("remove.txt");
        fs::write(&keep, b"keep").unwrap();
        fs::write(&remove, b"remove").unwrap();

        clean_root_except(tmp.path(), &[keep.clone()]).unwrap();

        assert!(keep.exists());
        assert!(!remove.exists());
    }

    #[test]
    fn test_clean_removes_dirs() {
        let tmp = TempDir::new().unwrap();
        let sub = tmp.path().join("subdir");
        fs::create_dir(&sub).unwrap();
        fs::write(sub.join("file.txt"), b"data").unwrap();

        clean_root_except(tmp.path(), &[]).unwrap();
        assert!(!sub.exists());
    }
}
