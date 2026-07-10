/**
 * useTaskRunner — 任务运行管理
 * 职责：管理任务生命周期、调用后端命令、维护任务状态
 */

import { ref, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { TaskStatus, TaskInput, TaskPreview, ProgressEvent } from "../types/task";

/** 格式化错误信息 */
function formatError(err: unknown): string {
  if (typeof err === "string") {
    return err;
  }
  if (err instanceof Error) {
    return err.message;
  }
  if (typeof err === "object" && err !== null) {
    // Tauri 错误对象
    if ("message" in err && typeof (err as any).message === "string") {
      return (err as any).message;
    }
    // 尝试 JSON 序列化
    try {
      return JSON.stringify(err);
    } catch {
      return String(err);
    }
  }
  return String(err);
}

export function useTaskRunner() {
  const status = ref<TaskStatus>("idle");
  const progress = ref(0);
  const currentStep = ref("");
  const detail = ref("");
  const loading = ref(false);
  const errorMessage = ref("");

  let unlistenProgress: UnlistenFn | null = null;

  /** 监听进度事件 */
  async function startListening(): Promise<void> {
    unlistenProgress = await listen<ProgressEvent>("task-progress", (event) => {
      handleProgressEvent(event.payload);
    });
  }

  /** 停止监听 */
  function stopListening(): void {
    if (unlistenProgress) {
      unlistenProgress();
      unlistenProgress = null;
    }
  }

  /** 预检查任务 */
  async function previewTask(input: TaskInput): Promise<TaskPreview | null> {
    loading.value = true;
    errorMessage.value = "";

    try {
      const result = await invoke<TaskPreview>("preview_task", { input });
      return result;
    } catch (err) {
      errorMessage.value = formatError(err);
      return null;
    } finally {
      loading.value = false;
    }
  }

  /** 开始任务 */
  async function startTask(input: TaskInput): Promise<boolean> {
    loading.value = true;
    errorMessage.value = "";
    status.value = "scanning";
    progress.value = 0;

    try {
      await invoke("start_task", { input });
      return true;
    } catch (err) {
      errorMessage.value = formatError(err);
      status.value = "failed";
      return false;
    } finally {
      loading.value = false;
    }
  }

  /** 取消任务 */
  async function cancelTask(): Promise<void> {
    try {
      await invoke("cancel_task");
    } catch (err) {
      errorMessage.value = formatError(err);
    }
  }

  /** 处理进度事件 */
  function handleProgressEvent(event: ProgressEvent): void {
    status.value = event.status;
    progress.value = event.progress;
    currentStep.value = event.stepName;
    detail.value = event.detail ?? "";
  }

  /** 重置状态 */
  function reset(): void {
    status.value = "idle";
    progress.value = 0;
    currentStep.value = "";
    detail.value = "";
    errorMessage.value = "";
  }

  // 自动开始监听
  startListening();

  // 组件卸载时停止监听
  onUnmounted(() => {
    stopListening();
  });

  return {
    status,
    progress,
    currentStep,
    detail,
    loading,
    errorMessage,
    previewTask,
    startTask,
    cancelTask,
    reset,
  };
}
