/**
 * 清理模块
 * 基于 design.md 第 12 节
 */
use std::path::{Path, PathBuf};

use crate::error::AppError;

/// 清理目录，保留指定文件
///
/// 安全约束（design 第 7.3、7.4 节）：
/// - 删除前对每个目标做真实路径校验，确保仍在 `root_dir` 内，禁止 `..` 越界。
/// - 跳过符号链接，不跟随链接删除其指向的真实路径。
pub fn clean_root_except(root_dir: &Path, keep_paths: &[PathBuf]) -> Result<(), AppError> {
    // 先固定工作目录的真实路径，后续逐项比对
    let canonical_root = root_dir
        .canonicalize()
        .map_err(|e| AppError::DeleteFailed(format!("无法解析根目录：{} - {}", root_dir.display(), e)))?;

    for entry in std::fs::read_dir(root_dir).map_err(|e| AppError::DeleteFailed(e.to_string()))? {
        let entry = entry.map_err(|e| AppError::DeleteFailed(e.to_string()))?;
        let path = entry.path();

        // 跳过符号链接，不跟随
        if super::safety::is_symlink(&path) {
            continue;
        }

        // 跳过需要保留的文件
        if keep_paths.iter().any(|k| k == &path) {
            continue;
        }

        // 跳过临时目录
        if path.file_name().map(|n| n == ".sjs_unzip_temp").unwrap_or(false) {
            continue;
        }

        // 删除前真实路径校验：确保目标经 canonicalize 后仍位于根目录内
        super::safety::validate_path_in_root(&path, &canonical_root)?;

        if path.is_dir() {
            std::fs::remove_dir_all(&path)
                .map_err(|e| AppError::DeleteFailed(e.to_string()))?;
        } else {
            std::fs::remove_file(&path)
                .map_err(|e| AppError::DeleteFailed(e.to_string()))?;
        }
    }

    Ok(())
}

/// 删除单个文件
///
/// 安全约束：跳过符号链接；文件不存在时静默跳过。
/// 调用方须确保 `path` 位于工作目录内。
pub fn delete_file(path: &Path) -> Result<(), AppError> {
    if !path.exists() {
        return Ok(());
    }

    // 跳过符号链接，避免误删链接指向的真实路径
    if super::safety::is_symlink(path) {
        return Ok(());
    }

    std::fs::remove_file(path).map_err(|e| AppError::DeleteFailed(e.to_string()))?;
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
