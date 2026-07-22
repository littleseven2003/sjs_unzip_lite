<script setup lang="ts">
/**
 * ResultDialog — 结果弹窗组件
 * 职责：展示任务成功或失败、提供打开文件夹按钮、提供复制错误详情按钮
 * 基于 design.md 第 14.6 节
 */
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export interface ResultDialogProps {
  success: boolean;
  title: string;
  message: string;
  detail?: string;
  folderPath?: string;
}

const props = withDefaults(defineProps<ResultDialogProps>(), {
  detail: "",
  folderPath: "",
});

const emit = defineEmits<{
  close: [];
}>();

const copied = ref(false);

/** 打开文件夹 */
async function handleOpenFolder(): Promise<void> {
  if (props.folderPath) {
    try {
      await invoke("open_folder", { path: props.folderPath });
    } catch (err) {
      console.error("打开文件夹失败:", err);
    }
  }
}

/** 复制错误详情 */
async function handleCopyDetail(): Promise<void> {
  if (props.detail) {
    await navigator.clipboard.writeText(props.detail);
    copied.value = true;
    setTimeout(() => {
      copied.value = false;
    }, 2000);
  }
}
</script>

<template>
  <div class="result-dialog">
    <div class="dialog-header">
      <div class="status-icon" :class="success ? 'success' : 'error'">
        <span v-if="success">✓</span>
        <span v-else>✗</span>
      </div>
      <h3 class="dialog-title">{{ title }}</h3>
    </div>

    <div class="dialog-body">
      <p class="dialog-message">{{ message }}</p>
      <p v-if="detail" class="dialog-detail">{{ detail }}</p>
    </div>

    <div class="dialog-footer">
      <button v-if="detail" class="btn btn-secondary" @click="handleCopyDetail">
        {{ copied ? "已复制" : "复制详情" }}
      </button>
      <button
        v-if="success && folderPath"
        class="btn btn-primary"
        @click="handleOpenFolder"
      >
        打开文件夹
      </button>
      <button class="btn btn-secondary" @click="emit('close')">
        完成
      </button>
    </div>
  </div>
</template>

<style scoped>
.result-dialog {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 24px;
  max-width: 440px;
  width: 100%;
}

.dialog-header {
  display: flex;
  align-items: center;
  gap: 16px;
}

.status-icon {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
  font-weight: 700;
  flex-shrink: 0;
}

.status-icon.success {
  background: rgba(32, 180, 134, 0.1);
  color: var(--color-success);
}

.status-icon.error {
  background: rgba(240, 82, 82, 0.1);
  color: var(--color-danger);
}

.dialog-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-main);
}

.dialog-body {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.dialog-message {
  font-size: 14px;
  color: var(--color-text-main);
  line-height: 1.6;
}

.dialog-detail {
  font-size: 13px;
  color: var(--color-text-muted);
  line-height: 1.5;
  padding: 12px;
  background: rgba(0, 0, 0, 0.03);
  border-radius: 8px;
  font-family: "SF Mono", "Fira Code", "Consolas", monospace;
  white-space: pre-wrap;
  word-break: break-all;
  max-height: 120px;
  overflow-y: auto;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 8px;
}

.btn {
  height: 40px;
  padding: 0 20px;
  border: none;
  border-radius: var(--radius-button);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  white-space: nowrap;
}

.btn-primary {
  background: linear-gradient(135deg, #6c8cff, #7c6cff);
  color: white;
  box-shadow: 0 10px 24px rgba(108, 140, 255, 0.28);
}

.btn-primary:hover {
  background: linear-gradient(135deg, #5c7df2, #6c5cf2);
}

.btn-secondary {
  background: rgba(255, 255, 255, 0.78);
  color: var(--color-text-main);
  border: 1px solid rgba(130, 150, 180, 0.22);
}

.btn-secondary:hover {
  background: rgba(255, 255, 255, 0.95);
}
</style>
