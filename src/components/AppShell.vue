<script setup lang="ts">
/**
 * AppShell — 整体页面结构容器
 * 职责：管理主题背景、全局弹窗挂载点、窗口标题栏区域
 * 布局：基于 design.md 第 14.2 节
 */

import { useTheme } from "../composables/useTheme";
import FolderPicker from "./FolderPicker.vue";
import TaskProgress from "./TaskProgress.vue";
import LogPanel from "./LogPanel.vue";

const { currentTheme } = useTheme();
</script>

<template>
  <div class="app-shell" :data-theme="currentTheme">
    <!-- 顶部标题区 -->
    <header class="app-header">
      <div class="header-content">
        <h1 class="app-title">sjs-unzip-tool</h1>
        <p class="app-subtitle">自动整理 7z 分卷压缩包，并连续处理伪装为 txt 的 rar 文件</p>
      </div>
    </header>

    <!-- 主内容区 -->
    <main class="app-main">
      <div class="main-content">
        <!-- 主操作卡片 -->
        <section class="card card-operation">
          <FolderPicker />
        </section>

        <!-- 底部区域：任务状态 + 日志 -->
        <div class="bottom-area">
          <!-- 任务状态卡片 -->
          <section class="card card-task">
            <TaskProgress />
          </section>

          <!-- 日志区 -->
          <section class="card card-log">
            <LogPanel />
          </section>
        </div>
      </div>
    </main>
  </div>
</template>

<style scoped>
.app-shell {
  min-height: 100vh;
  background:
    radial-gradient(circle at 12% 18%, rgba(108, 140, 255, 0.18), transparent 28%),
    radial-gradient(circle at 88% 12%, rgba(32, 180, 134, 0.14), transparent 24%),
    linear-gradient(135deg, var(--color-bg-start), var(--color-bg-end));
  display: flex;
  flex-direction: column;
}

.app-header {
  padding: 16px 24px 8px;
  flex-shrink: 0;
}

.header-content {
  max-width: 800px;
  margin: 0 auto;
}

.app-title {
  font-size: 18px;
  font-weight: 700;
  color: var(--color-text-main);
  letter-spacing: -0.02em;
}

.app-subtitle {
  font-size: 12px;
  color: var(--color-text-muted);
  margin-top: 2px;
}

.app-main {
  flex: 1;
  padding: 0 24px 16px;
}

.main-content {
  max-width: 800px;
  margin: 0 auto;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.card {
  background: var(--color-card);
  border: 1px solid var(--color-card-border);
  border-radius: var(--radius-card);
  box-shadow: var(--shadow-card);
  backdrop-filter: blur(16px);
  padding: 16px;
}

.card-operation {
  flex-shrink: 0;
}

.bottom-area {
  flex: 1;
  display: flex;
  gap: 12px;
  min-height: 0;
}

.card-task {
  flex: 0 0 auto;
  width: 240px;
}

.card-log {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
}
</style>
