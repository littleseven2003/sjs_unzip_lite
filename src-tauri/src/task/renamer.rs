/**
 * 重命名模块
 * 基于 design.md 第 13 节
 */
use std::path::{Path, PathBuf};

use crate::error::AppError;

/// 将 txt 文件改名为 rar
pub fn rename_txt_to_rar(txt_path: &Path) -> Result<PathBuf, AppError> {
    let new_name = txt_path
        .file_stem()
        .ok_or_else(|| AppError::RenameFailed("无法获取文件名".to_string()))?
        .to_string_lossy()
        .to_string()
        + ".rar";

    let new_path = txt_path.parent().unwrap_or(Path::new(".")).join(new_name);

    std::fs::rename(txt_path, &new_path)
        .map_err(|e| AppError::RenameFailed(e.to_string()))?;

    Ok(new_path)
}

/// 将 rar 文件改回 txt（校验失败时使用）
pub fn rename_rar_back_to_txt(rar_path: &Path) -> Result<PathBuf, AppError> {
    let new_name = rar_path
        .file_stem()
        .ok_or_else(|| AppError::RenameFailed("无法获取文件名".to_string()))?
        .to_string_lossy()
        .to_string()
        + ".txt";

    let new_path = rar_path.parent().unwrap_or(Path::new(".")).join(new_name);

    std::fs::rename(rar_path, &new_path)
        .map_err(|e| AppError::RenameFailed(e.to_string()))?;

    Ok(new_path)
}

/// 重命名根目录
pub fn rename_root_folder(
    root_dir: &Path,
    final_name: &str,
) -> Result<PathBuf, AppError> {
    let parent = root_dir
        .parent()
        .ok_or_else(|| AppError::RenameFailed("无法获取父目录".to_string()))?;

    let new_path = parent.join(final_name);

    // 检查同名冲突
    if new_path.exists() {
        return Err(AppError::RenameFailed(format!(
            "目标文件夹已存在：{}",
            new_path.display()
        )));
    }

    std::fs::rename(root_dir, &new_path)
        .map_err(|e| AppError::RenameFailed(e.to_string()))?;

    Ok(new_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_rename_txt_to_rar() {
        let tmp = TempDir::new().unwrap();
        let txt = tmp.path().join("stage_01.txt");
        fs::write(&txt, b"data").unwrap();

        let rar = rename_txt_to_rar(&txt).unwrap();
        assert_eq!(rar.file_name().unwrap().to_str().unwrap(), "stage_01.rar");
        assert!(!txt.exists());
        assert!(rar.exists());
    }

    #[test]
    fn test_rename_root_folder() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("old_name");
        fs::create_dir(&root).unwrap();

        let new_path = rename_root_folder(&root, "new_name").unwrap();
        assert_eq!(new_path.file_name().unwrap().to_str().unwrap(), "new_name");
        assert!(!root.exists());
    }

    #[test]
    fn test_rename_root_conflict() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("old");
        let conflict = tmp.path().join("new");
        fs::create_dir(&root).unwrap();
        fs::create_dir(&conflict).unwrap();

        let result = rename_root_folder(&root, "new");
        assert!(result.is_err());
    }
}
