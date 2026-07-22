/**
 * useLogEvents — 日志事件管理
 * 职责：接收后端日志事件、维护日志列表
 */

import { ref, onUnmounted } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { LogEvent } from "../types/log";

export function useLogEvents() {
  const logs = ref<LogEvent[]>([]);

  let unlistenLog: UnlistenFn | null = null;

  /** 监听日志事件 */
  async function startListening(): Promise<void> {
    unlistenLog = await listen<LogEvent>("task-log", (event) => {
      addLog(event.payload);
    });
  }

  /** 停止监听 */
  function stopListening(): void {
    if (unlistenLog) {
      unlistenLog();
      unlistenLog = null;
    }
  }

  /** 添加日志 */
  function addLog(event: LogEvent): void {
    logs.value.push(event);
  }

  /** 清空日志 */
  function clearLogs(): void {
    logs.value = [];
  }

  /** 复制全部日志 */
  function copyLogs(): string {
    return logs.value
      .map((log) => `[${log.time}] ${log.level.toUpperCase()} ${log.message}`)
      .join("\n");
  }

  // 自动开始监听
  startListening();

  // 组件卸载时停止监听
  onUnmounted(() => {
    stopListening();
  });

  return {
    logs,
    addLog,
    clearLogs,
    copyLogs,
  };
}
