/**
 * useTaskRunner — 任务运行管理
 * 职责：管理任务生命周期、调用后端命令、维护任务状态
 */

import { ref } from "vue";
import type { TaskStatus, TaskInput, TaskPreview, ProgressEvent } from "../types/task";

export function useTaskRunner() {
  const status = ref<TaskStatus>("idle");
  const progress = ref(0);
  const currentStep = ref("");
  const detail = ref("");

  /** 预检查任务 */
  async function previewTask(input: TaskInput): Promise<TaskPreview | null> {
    // TODO: 调用 Tauri 命令 preview_task
    return null;
  }

  /** 开始任务 */
  async function startTask(input: TaskInput): Promise<void> {
    // TODO: 调用 Tauri 命令 start_task
  }

  /** 取消任务 */
  async function cancelTask(): Promise<void> {
    // TODO: 调用 Tauri 命令 cancel_task
  }

  /** 处理进度事件 */
  function handleProgressEvent(event: ProgressEvent): void {
    status.value = event.status;
    progress.value = event.progress;
    currentStep.value = event.stepName;
    detail.value = event.detail ?? "";
  }

  return {
    status,
    progress,
    currentStep,
    detail,
    previewTask,
    startTask,
    cancelTask,
    handleProgressEvent,
  };
}
