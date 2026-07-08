/**
 * 密码配置相关类型定义
 * 基于 design.md 第 5 节
 */

/** 密码配置文件格式 */
export interface PasswordConfig {
  version: number;
  passwords: string[];
  updated_at: string;
}
