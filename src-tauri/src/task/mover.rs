/**
 * 文件移动模块
 * 基于 design.md 第 2.2 节（步骤 3）
 */
use std::path::{Path, PathBuf};

use super::context::VolumeGroup;
use crate::error::AppError;

/// 将分卷组中的所有文件移动到目标目录
pub fn move_volumes_to_root(volume_group: &VolumeGroup, target_dir: &Path) -> Result<(), AppError> {
    for file in &volume_group.files {
        let file_name = file
            .path
            .file_name()
            .ok_or_else(|| AppError::MoveFailed("无法获取文件名".to_string()))?;
        let dest = target_dir.join(file_name);

        // 如果源文件和目标路径相同，跳过
        if file.path == dest {
            continue;
        }

        // 检查目标是否已存在
        if dest.exists() {
            return Err(AppError::MoveFailed(format!(
                "目标文件已存在：{}",
                dest.display()
            )));
        }

        std::fs::rename(&file.path, &dest).or_else(|e| {
            // 跨磁盘时 rename 会失败，尝试 copy + delete
            if e.kind() == std::io::ErrorKind::CrossesDevices {
                std::fs::copy(&file.path, &dest)
                    .and_then(|_| std::fs::remove_file(&file.path))
                    .map_err(|e2| AppError::MoveFailed(e2.to_string()))
            } else {
                Err(AppError::MoveFailed(e.to_string()))
            }
        })?;
    }

    Ok(())
}

/// 删除空的源文件夹
pub fn remove_empty_source_folders(root_dir: &Path) -> Result<Vec<PathBuf>, AppError> {
    let mut removed = Vec::new();

    for entry in walkdir::WalkDir::new(root_dir)
        .follow_links(false)
        .min_depth(1)
        .sort_by(|a, b| b.path().cmp(a.path()))
    // 深度优先
    {
        let entry = entry.map_err(|e| AppError::Unknown(e.to_string()))?;
        if entry.file_type().is_dir() {
            let path = entry.path();
            if is_empty_dir(path) {
                std::fs::remove_dir(path).map_err(|e| AppError::DeleteFailed(e.to_string()))?;
                removed.push(path.to_path_buf());
            }
        }
    }

    Ok(removed)
}

fn is_empty_dir(path: &Path) -> bool {
    path.read_dir()
        .map(|mut d| d.next().is_none())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_remove_empty_dirs() {
        let tmp = TempDir::new().unwrap();
        let sub = tmp.path().join("empty_sub");
        fs::create_dir(&sub).unwrap();

        let removed = remove_empty_source_folders(tmp.path()).unwrap();
        assert_eq!(removed.len(), 1);
        assert!(!sub.exists());
    }

    #[test]
    fn test_keep_non_empty_dirs() {
        let tmp = TempDir::new().unwrap();
        let sub = tmp.path().join("non_empty");
        fs::create_dir(&sub).unwrap();
        fs::write(sub.join("file.txt"), b"data").unwrap();

        let removed = remove_empty_source_folders(tmp.path()).unwrap();
        assert!(removed.is_empty());
        assert!(sub.exists());
    }
}
