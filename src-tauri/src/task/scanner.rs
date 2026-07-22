/**
 * 目录扫描模块
 * 基于 design.md 第 6.1 ~ 6.5 节
 */
use std::path::{Path, PathBuf};

use regex::Regex;

use super::context::VolumeGroup;
use crate::error::AppError;

/// 扫描结果
#[derive(Debug)]
pub struct ScanResult {
    pub volume_groups: Vec<VolumeGroup>,
    pub extra_files: Vec<PathBuf>,
    pub extra_dirs: Vec<PathBuf>,
}

/// 分卷文件正则：xxx.7z.001 或 xxx.001
fn volume_regex() -> Regex {
    Regex::new(r#"(?i)^(.+?)(?:\.7z)?\.(\d{3})$"#).unwrap()
}

/// 递归扫描根目录
pub fn scan_root_recursively(root_dir: &Path) -> Result<ScanResult, AppError> {
    if !root_dir.is_dir() {
        return Err(AppError::InvalidRootDir(root_dir.display().to_string()));
    }

    let re = volume_regex();
    let mut volume_files: Vec<(String, u32, PathBuf, u64)> = Vec::new();
    let mut extra_files: Vec<PathBuf> = Vec::new();
    let mut extra_dirs: Vec<PathBuf> = Vec::new();

    for entry in walkdir::WalkDir::new(root_dir)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !e.path_is_symlink())
    {
        let entry = entry.map_err(|e| AppError::Unknown(e.to_string()))?;
        let path = entry.path();

        if path == root_dir {
            continue;
        }

        if entry.file_type().is_dir() {
            // 记录非空子目录（后续检查是否为空）
            extra_dirs.push(path.to_path_buf());
            continue;
        }

        if !entry.file_type().is_file() {
            continue;
        }

        let file_name = entry.file_name().to_string_lossy();

        if let Some(caps) = re.captures(&file_name) {
            let base_name = caps[1].to_string();
            let index: u32 = caps[2].parse().unwrap_or(0);
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            volume_files.push((base_name, index, path.to_path_buf(), size));
        } else {
            extra_files.push(path.to_path_buf());
        }
    }

    // 按 base_name 分组
    let mut groups: std::collections::HashMap<String, Vec<(u32, PathBuf, u64)>> =
        std::collections::HashMap::new();
    for (base_name, index, path, size) in volume_files {
        groups
            .entry(base_name)
            .or_default()
            .push((index, path, size));
    }

    let volume_groups: Vec<VolumeGroup> = groups
        .into_iter()
        .map(|(base_name, mut files)| {
            files.sort_by_key(|f| f.0);
            VolumeGroup {
                id: uuid_from_base_name(&base_name),
                base_name,
                files: files
                    .into_iter()
                    .map(|(index, path, size)| super::context::VolumeFile { path, index, size })
                    .collect(),
            }
        })
        .collect();

    Ok(ScanResult {
        volume_groups,
        extra_files,
        extra_dirs,
    })
}

/// 计算缺失的分卷编号
///
/// 在已有编号的最小值与最大值之间，找出未出现的编号。
/// 用于 runner 与 preview 的统一缺失检测。
pub fn missing_indexes_of(indexes: &[u32]) -> Vec<u32> {
    if indexes.is_empty() {
        return Vec::new();
    }
    let min = *indexes.iter().min().unwrap();
    let max = *indexes.iter().max().unwrap();
    (min..=max).filter(|i| !indexes.contains(i)).collect()
}

/// 计算分卷组中缺失的编号
pub fn missing_indexes(group: &super::context::VolumeGroup) -> Vec<u32> {
    let indexes: Vec<u32> = group.files.iter().map(|f| f.index).collect();
    missing_indexes_of(&indexes)
}

/// 简易 UUID 生成（基于 base_name hash）
fn uuid_from_base_name(base_name: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    base_name.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_scan_empty_directory() {
        let tmp = TempDir::new().unwrap();
        let result = scan_root_recursively(tmp.path()).unwrap();
        assert!(result.volume_groups.is_empty());
        assert!(result.extra_files.is_empty());
    }

    #[test]
    fn test_scan_single_volume_group() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("archive.7z.001"), b"fake").unwrap();
        fs::write(tmp.path().join("archive.7z.002"), b"fake").unwrap();

        let result = scan_root_recursively(tmp.path()).unwrap();
        assert_eq!(result.volume_groups.len(), 1);
        assert_eq!(result.volume_groups[0].files.len(), 2);
        assert_eq!(result.volume_groups[0].base_name, "archive");
    }

    #[test]
    fn test_scan_multiple_volume_groups() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("a.7z.001"), b"fake").unwrap();
        fs::write(tmp.path().join("b.7z.001"), b"fake").unwrap();

        let result = scan_root_recursively(tmp.path()).unwrap();
        assert_eq!(result.volume_groups.len(), 2);
    }

    #[test]
    fn test_scan_extra_files() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("archive.7z.001"), b"fake").unwrap();
        fs::write(tmp.path().join("readme.md"), b"hello").unwrap();

        let result = scan_root_recursively(tmp.path()).unwrap();
        assert_eq!(result.volume_groups.len(), 1);
        assert_eq!(result.extra_files.len(), 1);
    }
}
