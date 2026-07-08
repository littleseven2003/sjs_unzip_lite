/**
 * usePasswordStore — 密码表管理
 * 职责：加载、保存、编辑密码表
 */

import { ref } from "vue";
import type { PasswordConfig } from "../types/password";

export function usePasswordStore() {
  const config = ref<PasswordConfig>({
    version: 1,
    passwords: [],
    updated_at: new Date().toISOString(),
  });
  const loading = ref(false);

  /** 加载密码表 */
  async function loadPasswords(): Promise<void> {
    // TODO: 调用 Tauri 命令 load_passwords
    loading.value = true;
    try {
      // ...
    } finally {
      loading.value = false;
    }
  }

  /** 保存密码表 */
  async function savePasswords(): Promise<void> {
    // TODO: 调用 Tauri 命令 save_passwords
  }

  /** 添加密码 */
  function addPassword(password: string): void {
    config.value.passwords.push(password);
  }

  /** 删除密码 */
  function removePassword(index: number): void {
    config.value.passwords.splice(index, 1);
  }

  /** 导出 JSON */
  function exportJson(): string {
    return JSON.stringify(config.value, null, 2);
  }

  /** 导入 JSON */
  function importJson(json: string): boolean {
    try {
      const parsed = JSON.parse(json) as PasswordConfig;
      if (parsed.version && Array.isArray(parsed.passwords)) {
        config.value = parsed;
        return true;
      }
      return false;
    } catch {
      return false;
    }
  }

  return {
    config,
    loading,
    loadPasswords,
    savePasswords,
    addPassword,
    removePassword,
    exportJson,
    importJson,
  };
}
