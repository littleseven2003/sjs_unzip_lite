<script setup lang="ts">
/**
 * PasswordManager — 密码管理组件
 * 职责：加载密码表、展示密码列表、新增/删除/排序密码、导入导出 JSON
 * 基于 design.md 第 5.4 节
 */
import { ref, onMounted } from "vue";
import { usePasswordStore } from "../composables/usePasswordStore";

const emit = defineEmits<{
  close: [];
}>();

const {
  config,
  loading,
  errorMessage,
  loadPasswords,
  savePasswords,
  addPassword,
  removePassword,
  movePassword,
  clearPasswords,
  exportJson,
  importJson,
} = usePasswordStore();

const showPasswords = ref(false);
const newPassword = ref("");

/** 添加新密码 */
function handleAddPassword(): void {
  if (newPassword.value.trim()) {
    addPassword(newPassword.value);
    newPassword.value = "";
  }
}

/** 删除密码 */
function handleRemove(index: number): void {
  removePassword(index);
}

/** 上移密码 */
function handleMoveUp(index: number): void {
  if (index > 0) {
    movePassword(index, index - 1);
  }
}

/** 下移密码 */
function handleMoveDown(index: number): void {
  if (index < config.value.passwords.length - 1) {
    movePassword(index, index + 1);
  }
}

/** 保存并关闭 */
async function handleSave(): Promise<void> {
  const success = await savePasswords();
  if (success) {
    emit("close");
  }
}

/** 导出 JSON */
function handleExport(): void {
  const json = exportJson();
  const blob = new Blob([json], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = "passwords.json";
  a.click();
  URL.revokeObjectURL(url);
}

/** 导入 JSON */
function handleImport(): void {
  const input = document.createElement("input");
  input.type = "file";
  input.accept = ".json";
  input.onchange = async (e) => {
    const file = (e.target as HTMLInputElement).files?.[0];
    if (!file) return;

    const text = await file.text();
    importJson(text);
  };
  input.click();
}

/** 清空密码表 */
function handleClear(): void {
  if (confirm("确定要清空所有密码吗？")) {
    clearPasswords();
  }
}

// 组件挂载时加载密码
onMounted(() => {
  loadPasswords();
});
</script>

<template>
  <div class="password-manager">
    <div class="pm-header">
      <h2 class="pm-title">密码管理</h2>
      <p class="pm-hint">密码保存在本机，不会上传到任何服务器。</p>
    </div>

    <!-- 密码列表 -->
    <div class="pm-list">
      <div v-if="config.passwords.length === 0" class="pm-empty">
        暂无密码，点击下方添加
      </div>
      <div
        v-for="(pwd, index) in config.passwords"
        :key="index"
        class="pm-item"
      >
        <span class="pm-index">{{ index + 1 }}</span>
        <input
          :type="showPasswords ? 'text' : 'password'"
          :value="pwd"
          class="pm-input"
          placeholder="密码"
          @input="(e) => config.passwords[index] = (e.target as HTMLInputElement).value"
        />
        <div class="pm-actions">
          <button
            class="btn-icon"
            :disabled="index === 0"
            title="上移"
            @click="handleMoveUp(index)"
          >↑</button>
          <button
            class="btn-icon"
            :disabled="index === config.passwords.length - 1"
            title="下移"
            @click="handleMoveDown(index)"
          >↓</button>
          <button
            class="btn-icon btn-danger"
            title="删除"
            @click="handleRemove(index)"
          >×</button>
        </div>
      </div>
    </div>

    <!-- 添加密码 -->
    <div class="pm-add">
      <input
        v-model="newPassword"
        type="text"
        class="pm-input"
        placeholder="输入新密码"
        @keyup.enter="handleAddPassword"
      />
      <button class="btn btn-secondary" @click="handleAddPassword">添加</button>
    </div>

    <!-- 工具栏 -->
    <div class="pm-toolbar">
      <button class="btn btn-secondary" @click="showPasswords = !showPasswords">
        {{ showPasswords ? "隐藏密码" : "显示密码" }}
      </button>
      <button class="btn btn-secondary" @click="handleImport">导入 JSON</button>
      <button class="btn btn-secondary" @click="handleExport">导出 JSON</button>
      <button class="btn btn-secondary" @click="handleClear">清空</button>
    </div>

    <!-- 错误提示 -->
    <p v-if="errorMessage" class="pm-error">{{ errorMessage }}</p>

    <!-- 操作按钮 -->
    <div class="pm-footer">
      <button class="btn btn-secondary" @click="emit('close')">取消</button>
      <button class="btn btn-primary" :disabled="loading" @click="handleSave">
        {{ loading ? "保存中..." : "保存" }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.password-manager {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 24px;
  max-width: 560px;
  width: 100%;
}

.pm-header {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.pm-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-main);
}

.pm-hint {
  font-size: 12px;
  color: var(--color-text-muted);
}

.pm-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 300px;
  overflow-y: auto;
}

.pm-empty {
  text-align: center;
  color: var(--color-text-muted);
  padding: 24px 0;
  font-size: 14px;
}

.pm-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.pm-index {
  width: 24px;
  text-align: center;
  font-size: 12px;
  color: var(--color-text-muted);
}

.pm-input {
  flex: 1;
  height: 36px;
  padding: 0 12px;
  border: 1px solid var(--color-card-border);
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.6);
  font-size: 14px;
  color: var(--color-text-main);
  outline: none;
}

.pm-input:focus {
  border-color: var(--color-primary);
}

.pm-input::placeholder {
  color: var(--color-text-muted);
}

.pm-actions {
  display: flex;
  gap: 4px;
}

.btn-icon {
  width: 28px;
  height: 28px;
  border: 1px solid var(--color-card-border);
  border-radius: 6px;
  background: rgba(255, 255, 255, 0.6);
  font-size: 14px;
  color: var(--color-text-muted);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-icon:hover:not(:disabled) {
  background: rgba(255, 255, 255, 0.9);
  color: var(--color-text-main);
}

.btn-icon:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.btn-icon.btn-danger:hover:not(:disabled) {
  color: var(--color-danger);
  border-color: rgba(240, 82, 82, 0.3);
}

.pm-add {
  display: flex;
  gap: 8px;
}

.pm-toolbar {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.pm-error {
  font-size: 13px;
  color: var(--color-danger);
  padding: 8px 12px;
  background: rgba(240, 82, 82, 0.08);
  border-radius: 8px;
}

.pm-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  margin-top: 8px;
}

.btn {
  height: 36px;
  padding: 0 16px;
  border: none;
  border-radius: 8px;
  font-size: 13px;
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
}

.btn-primary:hover:not(:disabled) {
  background: linear-gradient(135deg, #5c7df2, #6c5cf2);
}

.btn-secondary {
  background: rgba(255, 255, 255, 0.78);
  color: var(--color-text-main);
  border: 1px solid rgba(130, 150, 180, 0.22);
}

.btn-secondary:hover:not(:disabled) {
  background: rgba(255, 255, 255, 0.95);
}
</style>
