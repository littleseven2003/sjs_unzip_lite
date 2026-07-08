/**
 * 任务取消管理
 * 基于 design.md 第 19 节
 */
use std::sync::atomic::{AtomicBool, Ordering};

/// 全局取消标记
static CANCELLED: AtomicBool = AtomicBool::new(false);

/// 检查是否已取消
pub fn is_cancelled() -> bool {
    CANCELLED.load(Ordering::Relaxed)
}

/// 设置取消标记
pub fn request_cancel() {
    CANCELLED.store(true, Ordering::Relaxed);
}

/// 重置取消标记
pub fn reset() {
    CANCELLED.store(false, Ordering::Relaxed);
}

/// 检查是否已取消，如果已取消则返回错误
pub fn check_cancelled() -> Result<(), crate::error::AppError> {
    if is_cancelled() {
        Err(crate::error::AppError::Cancelled)
    } else {
        Ok(())
    }
}
