/**
 * 进度事件定义
 * 基于 design.md 第 10.1 节
 */
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Idle,
    Scanning,
    Warning,
    MovingVolumes,
    CleaningFolders,
    Extracting7z,
    DeletingVolumes,
    FindingTxt,
    RenamingTxtToRar,
    CleaningExceptRar,
    ExtractingRar,
    DeletingRar,
    RenamingRoot,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressEvent {
    pub status: TaskStatus,
    pub step_name: String,
    pub progress: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}
