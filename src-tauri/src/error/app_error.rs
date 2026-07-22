/**
 * 统一错误类型
 * 基于 design.md 第 17 节
 */
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum AppError {
    #[error("无效的根目录：{0}")]
    InvalidRootDir(String),

    #[error("无效的最终文件夹名：{0}")]
    InvalidFolderName(String),

    #[error("危险的根目录：{0}")]
    DangerousRootDir(String),

    #[error("权限不足：{0}")]
    PermissionDenied(String),

    #[error("未找到分卷文件")]
    VolumeNotFound,

    #[error("找到多个分卷组：{0:?}")]
    MultipleVolumeGroups(Vec<String>),

    #[error("缺少第一个分卷文件")]
    MissingFirstVolume,

    #[error("分卷编号缺失：{0:?}")]
    MissingVolumeIndexes(Vec<u32>),

    #[error("分卷编号重复：{0:?}")]
    DuplicateVolumeIndexes(Vec<u32>),

    #[error("检测到额外文件：{0:?}")]
    ExtraFilesDetected(Vec<String>),

    #[error("文件移动失败：{0}")]
    MoveFailed(String),

    #[error("文件删除失败：{0}")]
    DeleteFailed(String),

    #[error("解压工具不可用")]
    ExtractToolNotFound,

    #[error("解压失败：{0}")]
    ExtractFailed(String),

    #[error("所有密码均尝试失败")]
    PasswordFailed,

    #[error("未找到 txt 文件")]
    TxtNotFound,

    #[error("找到多个 txt 文件：{0:?}")]
    MultipleTxtFound(Vec<String>),

    #[error("无效的压缩包：{0}")]
    InvalidArchive(String),

    #[error("重命名失败：{0}")]
    RenameFailed(String),

    #[error("任务已取消")]
    Cancelled,

    #[error("未知错误：{0}")]
    Unknown(String),
}
