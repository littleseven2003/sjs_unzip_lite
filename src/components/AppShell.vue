<script setup lang="ts">
/**
 * AppShell — 整体页面结构容器
 * 职责：管理主题背景、全局弹窗挂载点、窗口标题栏区域
 */
</script>

<template>
  <div class="app-shell">
    <slot />
  </div>
</template>

<style scoped>
.app-shell {
  min-height: 100vh;
  background:
    radial-gradient(circle at 12% 18%, rgba(108, 140, 255, 0.18), transparent 28%),
    radial-gradient(circle at 88% 12%, rgba(32, 180, 134, 0.14), transparent 24%),
    linear-gradient(135deg, var(--color-bg-start), var(--color-bg-end));
}
</style>
