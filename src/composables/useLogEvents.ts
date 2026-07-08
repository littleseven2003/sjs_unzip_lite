/**
 * useLogEvents — 日志事件管理
 * 职责：接收后端日志事件、维护日志列表
 */

import { ref } from "vue";
import type { LogEvent } from "../types/log";

export function useLogEvents() {
  const logs = ref<LogEvent[]>([]);

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

  return {
    logs,
    addLog,
    clearLogs,
    copyLogs,
  };
}
