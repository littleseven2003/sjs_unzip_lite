/**
 * 任务上下文
 * 基于 design.md 第 8.2 节
 */
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct VolumeGroup {
    pub id: String,
    pub base_name: String,
    pub files: Vec<VolumeFile>,
}

#[derive(Debug, Clone)]
pub struct VolumeFile {
    pub path: PathBuf,
    pub index: u32,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct TaskContext {
    pub root_dir: PathBuf,
    pub final_folder_name: String,
    pub password_list: Vec<String>,
    pub selected_volume_group: Option<VolumeGroup>,
    pub current_archive: Option<PathBuf>,
    pub current_iteration: u32,
    pub max_iterations: u32,
    pub dry_run: bool,
}

impl TaskContext {
    pub fn new(root_dir: PathBuf, final_folder_name: String, password_list: Vec<String>) -> Self {
        Self {
            root_dir,
            final_folder_name,
            password_list,
            selected_volume_group: None,
            current_archive: None,
            current_iteration: 0,
            max_iterations: 20,
            dry_run: false,
        }
    }
}
