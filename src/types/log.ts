/**
 * 日志相关类型定义
 * 基于 design.md 第 10 节
 */

/** 日志级别 */
export type LogLevel = "info" | "success" | "warning" | "error";

/** 日志事件 */
export interface LogEvent {
  time: string;
  level: LogLevel;
  message: string;
  detail?: string;
}
