<script setup lang="ts">
/**
 * LogPanel — 日志面板组件
 * 职责：展示滚动日志、自动滚动、暂停滚动、复制日志、清空日志、打开日志文件夹
 * 基于 design.md 第 14.5 节
 */
import { ref, watch, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useLogEvents } from "../composables/useLogEvents";

const { logs, clearLogs, copyLogs } = useLogEvents();

const autoScroll = ref(true);
const logContainer = ref<HTMLElement | null>(null);

/** 监听日志变化，自动滚动 */
watch(logs, async () => {
  if (autoScroll.value) {
    await nextTick();
    if (logContainer.value) {
      logContainer.value.scrollTop = logContainer.value.scrollHeight;
    }
  }
}, { deep: true });

/** 复制全部日志 */
function handleCopyLogs(): void {
  const text = copyLogs();
  navigator.clipboard.writeText(text);
}

/** 切换自动滚动 */
function toggleAutoScroll(): void {
  autoScroll.value = !autoScroll.value;
}

/** 打开日志文件夹 */
async function openLogFolder(): Promise<void> {
  try {
    await invoke("open_log_folder");
  } catch (err) {
    console.error("打开日志文件夹失败:", err);
  }
}
</script>

<template>
  <div class="log-panel">
    <div class="log-header">
      <h2 class="section-title">日志</h2>
      <div class="log-actions">
        <button
          class="btn-icon"
          :class="{ active: autoScroll }"
          title="自动滚动"
          @click="toggleAutoScroll"
        >
          <span>{{ autoScroll ? "暂停滚动" : "自动滚动" }}</span>
        </button>
        <button class="btn-icon" title="复制日志" @click="handleCopyLogs">
          <span>复制</span>
        </button>
        <button class="btn-icon" title="清空日志" @click="clearLogs">
          <span>清空</span>
        </button>
        <button class="btn-icon" title="打开日志文件夹" @click="openLogFolder">
          <span>文件夹</span>
        </button>
      </div>
    </div>

    <div ref="logContainer" class="log-content">
      <div v-if="logs.length === 0" class="log-empty">
        暂无日志
      </div>
      <div v-else class="log-list">
        <div
          v-for="(log, index) in logs"
          :key="index"
          class="log-item"
          :class="'log-' + log.level"
        >
          <span class="log-time">[{{ log.time }}]</span>
          <span class="log-level">{{ log.level.toUpperCase() }}</span>
          <span class="log-message">{{ log.message }}</span>
          <span v-if="log.detail" class="log-detail">{{ log.detail }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.log-panel {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.log-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-main);
}

.log-actions {
  display: flex;
  gap: 6px;
}

.btn-icon {
  padding: 4px 10px;
  border: 1px solid var(--color-card-border);
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.6);
  font-size: 12px;
  color: var(--color-text-muted);
  cursor: pointer;
  transition: all 0.2s;
}

.btn-icon:hover {
  background: rgba(255, 255, 255, 0.9);
  color: var(--color-text-main);
}

.btn-icon.active {
  background: rgba(108, 140, 255, 0.1);
  border-color: var(--color-primary);
  color: var(--color-primary);
}

.log-content {
  min-height: 120px;
  max-height: 240px;
  overflow-y: auto;
  font-family: "SF Mono", "Fira Code", "Consolas", monospace;
  font-size: 12px;
  line-height: 1.8;
}

.log-empty {
  color: var(--color-text-muted);
  text-align: center;
  padding: 32px 0;
}

.log-list {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.log-item {
  display: flex;
  gap: 8px;
  padding: 2px 0;
}

.log-time {
  color: var(--color-text-muted);
  flex-shrink: 0;
}

.log-level {
  font-weight: 600;
  width: 56px;
  flex-shrink: 0;
}

.log-message {
  color: var(--color-text-main);
}

.log-detail {
  color: var(--color-text-muted);
  font-size: 11px;
}

.log-info .log-level {
  color: var(--color-primary);
}

.log-success .log-level {
  color: var(--color-success);
}

.log-warning .log-level {
  color: var(--color-warning);
}

.log-error .log-level {
  color: var(--color-danger);
}
</style>
