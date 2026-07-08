/**
 * useTheme — 主题管理
 * 职责：管理当前主题（目前仅支持浅色主题）
 */

import { ref } from "vue";

type Theme = "light";

export function useTheme() {
  const currentTheme = ref<Theme>("light");

  /** 切换主题（预留扩展） */
  function setTheme(theme: Theme): void {
    currentTheme.value = theme;
    document.documentElement.setAttribute("data-theme", theme);
  }

  return {
    currentTheme,
    setTheme,
  };
}
