/**
 * 任务执行器
 * 基于 design.md 第 18 节（任务主流程伪代码）
 */
use std::path::{Path, PathBuf};

use tauri::{AppHandle, Emitter};

use crate::error::AppError;
use crate::events::{LogEvent, ProgressEvent, TaskStatus};
use super::context::TaskContext;
use super::{cleaner, extractor, mover, renamer, safety, scanner};

/// 执行任务主流程
pub async fn run_task(ctx: &mut TaskContext, app: &AppHandle) -> Result<(), AppError> {
    // 重置取消标记
    super::cancel::reset();

    // 创建日志文件写入器
    let log_writer = super::log_file::LogFileWriter::new()?;
    log_writer.write_header(
        &ctx.root_dir.to_string_lossy(),
        &ctx.final_folder_name,
    )?;

    // 执行任务，处理取消情况
    match run_task_inner(ctx, app, &log_writer).await {
        Ok(()) => {
            log_writer.write_footer(true)?;
            Ok(())
        }
        Err(AppError::Cancelled) => {
            emit_progress(app, TaskStatus::Cancelled, "任务已取消", 0, None);
            emit_log_and_file(app, &log_writer, LogEvent::warning("任务已取消", None));
            log_writer.write_footer(false)?;
            Err(AppError::Cancelled)
        }
        Err(e) => {
            emit_progress(app, TaskStatus::Failed, "处理失败", 0, Some(e.to_string()));
            emit_log_and_file(app, &log_writer, LogEvent::error(format!("处理失败：{}", e), None));
            log_writer.write_footer(false)?;
            Err(e)
        }
    }
}

/// 任务主流程内部实现
async fn run_task_inner(
    ctx: &mut TaskContext,
    app: &AppHandle,
    log_writer: &super::log_file::LogFileWriter,
) -> Result<(), AppError> {
    // 校验根目录
    safety::validate_root_dir(&ctx.root_dir)?;

    // 校验最终文件夹名
    if !ctx.final_folder_name.is_empty() {
        let err = crate::utils::filename::validate_folder_name(&ctx.final_folder_name);
        if let Some(msg) = err {
            return Err(AppError::InvalidRootDir(msg));
        }
    }

    emit_progress(app, TaskStatus::Scanning, "正在扫描文件夹", 5, None);
    emit_log_and_file(app, log_writer, LogEvent::info("开始扫描文件夹", Some(ctx.root_dir.display().to_string())));

    // 扫描目录
    let scan_result = scanner::scan_root_recursively(&ctx.root_dir)?;

    if scan_result.volume_groups.is_empty() {
        return Err(AppError::VolumeNotFound);
    }

    // 选择分卷组（如果只有一个则自动选择）
    let volume_group = if scan_result.volume_groups.len() == 1 {
        scan_result.volume_groups.into_iter().next().unwrap()
    } else {
        // 多组分卷，需要用户选择（通过前端弹窗）
        return Err(AppError::MultipleVolumeGroups(
            scan_result.volume_groups.iter().map(|g| g.base_name.clone()).collect(),
        ));
    };

    // 检查分卷完整性
    let indexes: Vec<u32> = volume_group.files.iter().map(|f| f.index).collect();
    let missing = find_missing_indexes(&indexes);
    if !missing.is_empty() {
        return Err(AppError::MissingVolumeIndexes(missing));
    }

    ctx.selected_volume_group = Some(volume_group.clone());

    emit_log_and_file(app, log_writer, LogEvent::info(
        format!("找到分卷组：{}，共 {} 个文件", volume_group.base_name, volume_group.files.len()),
        None,
    ));

    // 移动分卷到根目录
    super::cancel::check_cancelled()?;
    emit_progress(app, TaskStatus::MovingVolumes, "正在归集分卷文件", 15, None);
    mover::move_volumes_to_root(&volume_group, &ctx.root_dir)?;
    emit_log_and_file(app, log_writer, LogEvent::success("分卷文件已移动到根目录", None));

    // 清理空文件夹
    emit_progress(app, TaskStatus::CleaningFolders, "正在清理空文件夹", 25, None);
    let removed = mover::remove_empty_source_folders(&ctx.root_dir)?;
    if !removed.is_empty() {
        emit_log_and_file(app, log_writer, LogEvent::info(format!("已清理 {} 个空文件夹", removed.len()), None));
    }

    // 清理额外文件（在解压前清理，避免解压后同名文件被误删）
    let volume_file_names: Vec<String> = volume_group.files.iter()
        .filter_map(|f| f.path.file_name().map(|n| n.to_string_lossy().to_string()))
        .collect();

    let extra_files: Vec<PathBuf> = std::fs::read_dir(&ctx.root_dir)
        .map_err(|e| AppError::Unknown(e.to_string()))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            if !path.is_file() {
                return false;
            }
            let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            // 跳过分卷文件
            if volume_file_names.contains(&file_name) {
                return false;
            }
            // 跳过隐藏文件
            if file_name.starts_with(".") {
                return false;
            }
            true
        })
        .collect();

    if !extra_files.is_empty() {
        emit_progress(app, TaskStatus::CleaningFolders, "正在清理额外文件", 30, None);
        let mut cleaned_count = 0;
        for file in &extra_files {
            match cleaner::delete_file(file) {
                Ok(()) => cleaned_count += 1,
                Err(e) => {
                    emit_log_and_file(app, log_writer, LogEvent::warning(
                        format!("清理文件失败：{} - {}", file.display(), e),
                        None,
                    ));
                }
            }
        }
        emit_log_and_file(app, log_writer, LogEvent::info(format!("已清理 {} 个额外文件", cleaned_count), None));
    }

    // 解压 7z 分卷
    super::cancel::check_cancelled()?;
    emit_progress(app, TaskStatus::Extracting7z, "正在解压 7z 分卷压缩包", 35, None);
    let first_volume = ctx.root_dir.join(format!("{}.7z.001", volume_group.base_name));
    let first_volume = if first_volume.exists() {
        first_volume
    } else {
        // 尝试不带 .7z 的格式
        ctx.root_dir.join(format!("{}.001", volume_group.base_name))
    };

    if !first_volume.exists() {
        return Err(AppError::MissingFirstVolume);
    }

    let password_index = extractor::extract_with_passwords(
        app,
        &first_volume,
        &ctx.root_dir,
        &ctx.password_list,
    ).await?;

    emit_log_and_file(app, log_writer, LogEvent::success(
        format!("解压成功，使用的密码序号：{}", password_index),
        None,
    ));

    // 删除分卷文件
    emit_progress(app, TaskStatus::DeletingVolumes, "正在删除分卷文件", 45, None);
    for file in &volume_group.files {
        let path = ctx.root_dir.join(file.path.file_name().unwrap_or_default());
        if path.exists() {
            cleaner::delete_file(&path)?;
        }
    }

    emit_log_and_file(app, log_writer, LogEvent::success("已删除分卷文件", None));

    // txt → rar 循环
    for iteration in 1..=ctx.max_iterations {
        super::cancel::check_cancelled()?;
        ctx.current_iteration = iteration;

        emit_progress(app, TaskStatus::FindingTxt, "正在查找 txt 文件", 50, None);

        // 查找 txt 文件
        let txt_candidates = find_txt_candidates(&ctx.root_dir)?;

        if txt_candidates.is_empty() {
            // 循环结束，重命名根目录
            break;
        }

        let txt_file = if txt_candidates.len() == 1 {
            txt_candidates.into_iter().next().unwrap()
        } else {
            // 多个 txt，需要用户选择
            return Err(AppError::MultipleTxtFound(
                txt_candidates.iter().map(|p| p.display().to_string()).collect(),
            ));
        };

        emit_log_and_file(app, log_writer, LogEvent::info(
            format!("找到 txt 文件：{}", txt_file.display()),
            None,
        ));

        // 改名 txt → rar
        emit_progress(app, TaskStatus::RenamingTxtToRar, "正在将 txt 改名为 rar", 55, None);
        let rar_file = renamer::rename_txt_to_rar(&txt_file)?;
        emit_log_and_file(app, log_writer, LogEvent::success(
            format!("已改名为：{}", rar_file.display()),
            None,
        ));

        // 校验压缩包有效性
        emit_progress(app, TaskStatus::CleaningExceptRar, "正在校验压缩包", 60, None);
        let is_valid = extractor::validate_archive(app, &rar_file).await?;
        if !is_valid {
            // 不是有效压缩包，改回原名
            let restored_path = renamer::rename_rar_back_to_txt(&rar_file)?;
            emit_log_and_file(app, log_writer, LogEvent::warning(
                format!("文件不是有效的压缩包，已恢复原名：{}", restored_path.display()),
                None,
            ));
            // 停止处理
            break;
        }
        emit_log_and_file(app, log_writer, LogEvent::success("压缩包校验通过", None));

        // 清理无关文件
        cleaner::clean_root_except(&ctx.root_dir, &[rar_file.clone()])?;
        emit_log_and_file(app, log_writer, LogEvent::success("已清理无关文件", None));

        // 解压 rar
        super::cancel::check_cancelled()?;
        emit_progress(app, TaskStatus::ExtractingRar, "正在解压 rar 文件", 70, None);
        let password_index = extractor::extract_with_passwords(
            app,
            &rar_file,
            &ctx.root_dir,
            &ctx.password_list,
        ).await?;

        emit_log_and_file(app, log_writer, LogEvent::success(
            format!("RAR 解压成功，使用的密码序号：{}", password_index),
            None,
        ));

        // 删除 rar 文件
        emit_progress(app, TaskStatus::DeletingRar, "正在删除 rar 文件", 80, None);
        cleaner::delete_file(&rar_file)?;
        emit_log_and_file(app, log_writer, LogEvent::success("已删除 rar 文件", None));
    }

    // 重命名根目录
    emit_progress(app, TaskStatus::RenamingRoot, "正在重命名根文件夹", 90, None);
    if !ctx.final_folder_name.is_empty() {
        let current_name = ctx.root_dir.file_name().unwrap_or_default().to_string_lossy();
        if current_name != ctx.final_folder_name {
            let new_path = renamer::rename_root_folder(&ctx.root_dir, &ctx.final_folder_name)?;
            emit_log_and_file(app, log_writer, LogEvent::success(
                format!("已重命名为：{}", new_path.display()),
                None,
            ));
        }
    }

    emit_progress(app, TaskStatus::Completed, "处理完成", 100, None);
    emit_log_and_file(app, log_writer, LogEvent::success("处理完成", None));

    Ok(())
}

/// 查找缺失的分卷编号
fn find_missing_indexes(indexes: &[u32]) -> Vec<u32> {
    if indexes.is_empty() {
        return Vec::new();
    }
    let min = *indexes.iter().min().unwrap();
    let max = *indexes.iter().max().unwrap();
    (min..=max).filter(|i| !indexes.contains(i)).collect()
}

/// 查找 txt 候选文件
fn find_txt_candidates(root_dir: &Path) -> Result<Vec<PathBuf>, AppError> {
    let mut candidates = Vec::new();

    for entry in std::fs::read_dir(root_dir).map_err(|e| AppError::Unknown(e.to_string()))? {
        let entry = entry.map_err(|e| AppError::Unknown(e.to_string()))?;
        let path = entry.path();

        if path.is_file() {
            let name = path.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
            if name.ends_with(".txt") {
                // 检查文件大小
                let size = path.metadata().map(|m| m.len()).unwrap_or(0);
                if size == 0 {
                    return Err(AppError::InvalidArchive(format!(
                        "txt 文件大小为 0：{}",
                        path.display()
                    )));
                }
                candidates.push(path);
            }
        }
    }

    Ok(candidates)
}

/// 发送进度事件
fn emit_progress(app: &AppHandle, status: TaskStatus, step_name: &str, progress: u32, detail: Option<String>) {
    let event = ProgressEvent {
        status,
        step_name: step_name.to_string(),
        progress,
        current: None,
        total: None,
        detail,
    };
    let _ = app.emit("task-progress", &event);
}

/// 发送日志事件并写入日志文件
fn emit_log_and_file(app: &AppHandle, log_writer: &super::log_file::LogFileWriter, event: LogEvent) {
    let _ = app.emit("task-log", &event);
    let _ = log_writer.write_log(&event);
}
