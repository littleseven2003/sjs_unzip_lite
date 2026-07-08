/**
 * usePasswordStore — 密码表管理
 * 职责：加载、保存、编辑密码表
 */

import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { PasswordConfig } from "../types/password";

export function usePasswordStore() {
  const config = ref<PasswordConfig>({
    version: 1,
    passwords: [],
    updated_at: new Date().toISOString(),
  });
  const loading = ref(false);
  const errorMessage = ref("");

  /** 加载密码表 */
  async function loadPasswords(): Promise<boolean> {
    loading.value = true;
    errorMessage.value = "";

    try {
      const result = await invoke<PasswordConfig>("load_passwords");
      config.value = result;
      return true;
    } catch (err) {
      errorMessage.value = `${err}`;
      return false;
    } finally {
      loading.value = false;
    }
  }

  /** 保存密码表 */
  async function savePasswords(): Promise<boolean> {
    loading.value = true;
    errorMessage.value = "";

    try {
      config.value.updated_at = new Date().toISOString();
      await invoke("save_passwords", { config: config.value });
      return true;
    } catch (err) {
      errorMessage.value = `${err}`;
      return false;
    } finally {
      loading.value = false;
    }
  }

  /** 添加密码 */
  function addPassword(password: string): void {
    config.value.passwords.push(password);
  }

  /** 删除密码 */
  function removePassword(index: number): void {
    config.value.passwords.splice(index, 1);
  }

  /** 更新密码 */
  function updatePassword(index: number, password: string): void {
    config.value.passwords[index] = password;
  }

  /** 移动密码位置 */
  function movePassword(fromIndex: number, toIndex: number): void {
    const passwords = config.value.passwords;
    if (fromIndex < 0 || fromIndex >= passwords.length) return;
    if (toIndex < 0 || toIndex >= passwords.length) return;

    const [removed] = passwords.splice(fromIndex, 1);
    passwords.splice(toIndex, 0, removed);
  }

  /** 清空密码表 */
  function clearPasswords(): void {
    config.value.passwords = [];
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
      errorMessage.value = "无效的密码配置格式";
      return false;
    } catch {
      errorMessage.value = "JSON 解析失败";
      return false;
    }
  }

  return {
    config,
    loading,
    errorMessage,
    loadPasswords,
    savePasswords,
    addPassword,
    removePassword,
    updatePassword,
    movePassword,
    clearPasswords,
    exportJson,
    importJson,
  };
}
