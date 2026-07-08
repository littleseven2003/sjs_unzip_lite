<script setup lang="ts">
/**
 * FolderPicker — 文件夹选择组件
 * 职责：选择根文件夹、展示路径、推断默认最终文件夹名、校验危险目录
 * 基于 design.md 第 14.3 节
 */
import { ref } from "vue";

const folderPath = ref("");
const finalFolderName = ref("");

/** 选择文件夹（调用 Tauri 对话框） */
async function selectFolder(): Promise<void> {
  // TODO: 调用 Tauri dialog API
}

/** 开始处理 */
async function startTask(): Promise<void> {
  // TODO: 调用 preview_task → 确认 → start_task
}
</script>

<template>
  <div class="folder-picker">
    <h2 class="section-title">选择文件夹</h2>
    <p class="section-desc">请选择包含 7z 分卷文件的文件夹。分卷文件可以位于子文件夹中。</p>

    <!-- 文件夹路径 -->
    <div class="form-row">
      <div class="input-group">
        <input
          type="text"
          class="input-field"
          :value="folderPath"
          placeholder="请选择文件夹..."
          readonly
        />
        <button class="btn btn-primary" @click="selectFolder">选择文件夹</button>
      </div>
    </div>

    <!-- 最终文件夹名 -->
    <div class="form-row">
      <label class="form-label">最终文件夹名</label>
      <div class="input-group">
        <input
          v-model="finalFolderName"
          type="text"
          class="input-field"
          placeholder="留空则使用所选文件夹名称"
        />
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="form-actions">
      <button class="btn btn-secondary">管理密码表</button>
      <button class="btn btn-secondary">预检查</button>
      <button
        class="btn btn-primary"
        :disabled="!folderPath"
        @click="startTask"
      >
        开始处理
      </button>
    </div>
  </div>
</template>

<style scoped>
.folder-picker {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-main);
}

.section-desc {
  font-size: 13px;
  color: var(--color-text-muted);
  line-height: 1.5;
}

.form-row {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--color-text-main);
}

.input-group {
  display: flex;
  gap: 10px;
}

.input-field {
  flex: 1;
  height: 40px;
  padding: 0 14px;
  border: 1px solid var(--color-card-border);
  border-radius: var(--radius-button);
  background: rgba(255, 255, 255, 0.6);
  font-size: 14px;
  color: var(--color-text-main);
  outline: none;
  transition: border-color 0.2s;
}

.input-field:focus {
  border-color: var(--color-primary);
}

.input-field::placeholder {
  color: var(--color-text-muted);
}

.form-actions {
  display: flex;
  gap: 10px;
  justify-content: flex-end;
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

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-primary {
  background: linear-gradient(135deg, #6c8cff, #7c6cff);
  color: white;
  box-shadow: 0 10px 24px rgba(108, 140, 255, 0.28);
}

.btn-primary:hover:not(:disabled) {
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
