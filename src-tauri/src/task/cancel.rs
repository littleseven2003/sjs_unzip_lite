/**
 * 任务取消管理
 * 基于 design.md 第 19 节
 */
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// 全局取消标记
static CANCELLED: AtomicBool = AtomicBool::new(false);

/// 全局子进程 ID
static CHILD_PID: AtomicU32 = AtomicU32::new(0);

/// 检查是否已取消
pub fn is_cancelled() -> bool {
    CANCELLED.load(Ordering::Relaxed)
}

/// 设置取消标记
pub fn request_cancel() {
    CANCELLED.store(true, Ordering::Relaxed);

    // 尝试终止子进程
    let pid = CHILD_PID.load(Ordering::Relaxed);
    if pid > 0 {
        kill_process(pid);
    }
}

/// 重置取消标记
pub fn reset() {
    CANCELLED.store(false, Ordering::Relaxed);
    CHILD_PID.store(0, Ordering::Relaxed);
}

/// 注册子进程 ID
pub fn register_child_pid(pid: u32) {
    CHILD_PID.store(pid, Ordering::Relaxed);
}

/// 移除子进程注册
pub fn unregister_child_pid() {
    CHILD_PID.store(0, Ordering::Relaxed);
}

/// 检查是否已取消，如果已取消则返回错误
pub fn check_cancelled() -> Result<(), crate::error::AppError> {
    if is_cancelled() {
        Err(crate::error::AppError::Cancelled)
    } else {
        Ok(())
    }
}

/// 终止进程
fn kill_process(pid: u32) {
    #[cfg(unix)]
    {
        unsafe {
            libc::kill(pid as i32, libc::SIGTERM);
        }
    }

    #[cfg(windows)]
    {
        use std::process::Command;
        let _ = Command::new("taskkill")
            .args(["/F", "/PID", &pid.to_string()])
            .output();
    }
}
