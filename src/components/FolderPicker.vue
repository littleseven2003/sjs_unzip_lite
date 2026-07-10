<script setup lang="ts">
/**
 * FolderPicker — 文件夹选择组件
 * 职责：选择根文件夹、展示路径、推断默认最终文件夹名、校验危险目录
 * 基于 design.md 第 14.3 节
 */
import { ref, computed, watch } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { useTaskRunner } from "../composables/useTaskRunner";
import PasswordManager from "./PasswordManager.vue";
import WarningDialog from "./WarningDialog.vue";
import ResultDialog from "./ResultDialog.vue";
import type { TaskPreview } from "../types/task";

const {
  status,
  loading,
  errorMessage,
  previewTask,
  startTask: executeTask,
  cancelTask,
} = useTaskRunner();

const folderPath = ref("");
const finalFolderName = ref("");
const preview = ref<TaskPreview | null>(null);
const showPasswordManager = ref(false);
const showWarning = ref(false);
const showResult = ref(false);
const warningTitle = ref("");
const warningMessage = ref("");
const warningDetail = ref("");
const warningType = ref<"confirm" | "select">("confirm");
const warningOptions = ref<Array<{ label: string; value: string; danger?: boolean }>>([]);
const pendingContinueWithExtraFiles = ref(false);

/** 默认最终文件夹名 */
const defaultFolderName = computed(() => {
  if (!folderPath.value) return "";
  const parts = folderPath.value.replace(/\\/g, "/").split("/");
  return parts[parts.length - 1] || "";
});

/** 显示的最终文件夹名提示 */
const placeholderText = computed(() => {
  if (defaultFolderName.value) {
    return `留空则使用「${defaultFolderName.value}」`;
  }
  return "留空则使用所选文件夹名称";
});

/** 是否正在运行任务 */
const isRunning = computed(() => {
  return status.value !== "idle" && status.value !== "completed" && status.value !== "failed" && status.value !== "cancelled";
});

/** 选择文件夹 */
async function selectFolder(): Promise<void> {
  try {
    errorMessage.value = "";
    preview.value = null;
    showWarning.value = false;

    const selected = await open({
      directory: true,
      multiple: false,
      title: "选择包含 7z 分卷文件的文件夹",
    });

    if (selected) {
      folderPath.value = selected as string;
      finalFolderName.value = "";
    }
  } catch (err) {
    errorMessage.value = `选择文件夹失败：${err}`;
  }
}

/** 预检查 */
async function handlePreview(): Promise<void> {
  if (!folderPath.value) return;

  const result = await previewTask({
    rootDir: folderPath.value,
    finalFolderName: finalFolderName.value || undefined,
    continueOnInitialExtraFiles: false,
  });

  if (result) {
    preview.value = result;

    // 检查是否有需要用户确认的警告
    const hasExtraFiles = result.warnings.some((w) => w.code === "EXTRA_FILES");
    const hasMultipleGroups = result.warnings.some((w) => w.code === "MULTIPLE_GROUPS");

    if (hasExtraFiles) {
      // 显示额外文件警告
      warningTitle.value = "检测到额外文件";
      warningMessage.value = "所选文件夹中检测到除 7z 分卷文件以外的其他文件或非空文件夹。继续处理可能会在后续清理步骤中删除这些内容。";
      warningDetail.value = "建议先备份该文件夹。";
      warningType.value = "confirm";
      warningOptions.value = [];
      pendingContinueWithExtraFiles.value = true;
      showWarning.value = true;
    } else if (hasMultipleGroups) {
      // 显示多组分卷选择
      const groups = result.volumeGroups;
      warningTitle.value = "找到多组分卷文件";
      warningMessage.value = "请选择需要处理的分卷组。";
      warningDetail.value = "";
      warningType.value = "select";
      warningOptions.value = groups.map((g) => ({
        label: `${g.baseName} (${g.volumeCount} 个分卷)`,
        value: g.id,
      }));
      showWarning.value = true;
    }
  }
}

/** 预检查（跳过警告） */
async function handlePreviewSkipWarning(): Promise<void> {
  if (!folderPath.value) return;

  const result = await previewTask({
    rootDir: folderPath.value,
    finalFolderName: finalFolderName.value || undefined,
    continueOnInitialExtraFiles: true,
  });

  if (result) {
    preview.value = result;
  }
}

/** 开始处理 */
async function handleStartTask(): Promise<void> {
  if (!folderPath.value) return;

  const success = await executeTask({
    rootDir: folderPath.value,
    finalFolderName: finalFolderName.value || undefined,
    continueOnInitialExtraFiles: pendingContinueWithExtraFiles.value,
  });

  if (success) {
    preview.value = null;
  }
}

/** 取消任务 */
async function handleCancelTask(): Promise<void> {
  await cancelTask();
}

/** 处理警告确认 */
function handleWarningConfirm(): void {
  showWarning.value = false;
  if (pendingContinueWithExtraFiles.value) {
    // 用户确认继续处理，跳过警告重新预检查
    pendingContinueWithExtraFiles.value = false;
    handlePreviewSkipWarning();
  }
}

/** 处理警告取消 */
function handleWarningCancel(): void {
  showWarning.value = false;
  pendingContinueWithExtraFiles.value = false;
}

/** 处理警告选择 */
function handleWarningSelect(value: string): void {
  showWarning.value = false;
  // TODO: 处理用户选择的分卷组
  console.log("Selected volume group:", value);
}

/** 监听任务状态变化，显示结果弹窗 */
watch(status, (newStatus) => {
  if (newStatus === "completed") {
    showResult.value = true;
  } else if (newStatus === "failed") {
    showResult.value = true;
  }
});

/** 格式化文件大小 */
function formatSize(bytes: number): string {
  if (bytes < 1024) return bytes + " B";
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + " KB";
  if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(1) + " MB";
  return (bytes / (1024 * 1024 * 1024)).toFixed(1) + " GB";
}

/** 警告级别颜色 */
function warningClass(code: string): string {
  if (code === "NO_VOLUMES" || code === "MISSING_VOLUMES") return "warning-error";
  if (code === "MULTIPLE_GROUPS" || code === "DUPLICATE_VOLUMES") return "warning-warn";
  return "warning-info";
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
        <button class="btn btn-primary" :disabled="isRunning" @click="selectFolder">
          选择文件夹
        </button>
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
          :placeholder="placeholderText"
          :disabled="isRunning"
        />
      </div>
      <p class="form-hint">处理过程中会移动、删除和重命名文件。建议提前备份原始文件夹。</p>
    </div>

    <!-- 预检查结果 -->
    <div v-if="preview" class="preview-section">
      <div class="preview-header">
        <h3 class="preview-title">预检查结果</h3>
        <span class="preview-status" :class="preview.canStart ? 'can-start' : 'cannot-start'">
          {{ preview.canStart ? "可以开始" : "存在问题" }}
        </span>
      </div>

      <!-- 分卷组信息 -->
      <div v-if="preview.volumeGroups.length > 0" class="preview-group">
        <p class="preview-label">分卷组：</p>
        <div v-for="group in preview.volumeGroups" :key="group.id" class="volume-group">
          <p class="group-name">{{ group.baseName }}</p>
          <p class="group-info">{{ group.volumeCount }} 个分卷，{{ formatSize(group.totalSize) }}</p>
        </div>
      </div>

      <!-- 警告信息 -->
      <div v-if="preview.warnings.length > 0" class="warnings-list">
        <div
          v-for="(warning, index) in preview.warnings"
          :key="index"
          class="warning-item"
          :class="warningClass(warning.code)"
        >
          <p class="warning-message">{{ warning.message }}</p>
          <p v-if="warning.detail" class="warning-detail">{{ warning.detail }}</p>
        </div>
      </div>
    </div>

    <!-- 错误提示 -->
    <p v-if="errorMessage" class="error-message">{{ errorMessage }}</p>

    <!-- 操作按钮 -->
    <div class="form-actions">
      <template v-if="isRunning">
        <button class="btn btn-danger" @click="handleCancelTask">取消任务</button>
      </template>
      <template v-else>
        <button class="btn btn-secondary" @click="showPasswordManager = true">
          管理密码表
        </button>
        <button
          class="btn btn-secondary"
          :disabled="!folderPath || loading"
          @click="handlePreview"
        >
          {{ loading ? "检查中..." : "预检查" }}
        </button>
        <button
          class="btn btn-primary"
          :disabled="!folderPath || loading || (preview !== null && !preview.canStart)"
          @click="handleStartTask"
        >
          {{ loading ? "处理中..." : "开始处理" }}
        </button>
      </template>
    </div>

    <!-- 密码管理弹窗 -->
    <Teleport to="body">
      <div v-if="showPasswordManager" class="modal-overlay" @click.self="showPasswordManager = false">
        <div class="modal-content">
          <PasswordManager @close="showPasswordManager = false" />
        </div>
      </div>
    </Teleport>

    <!-- 警告弹窗 -->
    <Teleport to="body">
      <div v-if="showWarning" class="modal-overlay">
        <div class="modal-content">
          <WarningDialog
            :title="warningTitle"
            :message="warningMessage"
            :detail="warningDetail"
            :type="warningType"
            :options="warningOptions"
            @confirm="handleWarningConfirm"
            @cancel="handleWarningCancel"
            @select="handleWarningSelect"
          />
        </div>
      </div>
    </Teleport>

    <!-- 结果弹窗 -->
    <Teleport to="body">
      <div v-if="showResult" class="modal-overlay" @click.self="showResult = false">
        <div class="modal-content">
          <ResultDialog
            :success="status === 'completed'"
            :title="status === 'completed' ? '处理完成' : '处理失败'"
            :message="status === 'completed' ? '文件已整理完成。' : errorMessage || '处理过程中发生错误。'"
            :detail="status === 'failed' ? errorMessage : ''"
            :folder-path="folderPath"
            @close="showResult = false"
          />
        </div>
      </div>
    </Teleport>
  </div>
</template>

<style scoped>
.folder-picker {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-main);
}

.section-desc {
  font-size: 12px;
  color: var(--color-text-muted);
  line-height: 1.4;
}

.form-row {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.form-label {
  font-size: 12px;
  font-weight: 500;
  color: var(--color-text-main);
}

.form-hint {
  font-size: 11px;
  color: var(--color-text-muted);
  line-height: 1.3;
}

.input-group {
  display: flex;
  gap: 8px;
}

.input-field {
  flex: 1;
  height: 32px;
  padding: 0 10px;
  border: 1px solid var(--color-card-border);
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.6);
  font-size: 13px;
  color: var(--color-text-main);
  outline: none;
  transition: border-color 0.2s;
}

.input-field:focus {
  border-color: var(--color-primary);
}

.input-field:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.input-field::placeholder {
  color: var(--color-text-muted);
}

.preview-section {
  background: rgba(255, 255, 255, 0.4);
  border: 1px solid var(--color-card-border);
  border-radius: 8px;
  padding: 10px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.preview-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.preview-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--color-text-main);
}

.preview-status {
  font-size: 11px;
  font-weight: 500;
  padding: 2px 6px;
  border-radius: 6px;
}

.can-start {
  background: rgba(32, 180, 134, 0.1);
  color: var(--color-success);
}

.cannot-start {
  background: rgba(240, 82, 82, 0.1);
  color: var(--color-danger);
}

.preview-group {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.preview-label {
  font-size: 12px;
  color: var(--color-text-muted);
}

.volume-group {
  padding: 8px;
  background: rgba(255, 255, 255, 0.6);
  border-radius: 8px;
}

.group-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-main);
}

.group-info {
  font-size: 12px;
  color: var(--color-text-muted);
}

.warnings-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.warning-item {
  padding: 8px 12px;
  border-radius: 8px;
  font-size: 13px;
}

.warning-error {
  background: rgba(240, 82, 82, 0.08);
  color: var(--color-danger);
}

.warning-warn {
  background: rgba(245, 165, 36, 0.08);
  color: var(--color-warning);
}

.warning-info {
  background: rgba(108, 140, 255, 0.08);
  color: var(--color-primary);
}

.warning-message {
  line-height: 1.4;
}

.warning-detail {
  font-size: 12px;
  opacity: 0.8;
  margin-top: 4px;
}

.error-message {
  font-size: 13px;
  color: var(--color-danger);
  padding: 8px 12px;
  background: rgba(240, 82, 82, 0.08);
  border-radius: 8px;
}

.form-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
  margin-top: 4px;
}

.btn {
  height: 32px;
  padding: 0 14px;
  border: none;
  border-radius: 8px;
  font-size: 12px;
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
  box-shadow: 0 6px 16px rgba(108, 140, 255, 0.28);
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

.btn-danger {
  background: rgba(240, 82, 82, 0.1);
  color: var(--color-danger);
  border: 1px solid rgba(240, 82, 82, 0.2);
}

.btn-danger:hover {
  background: rgba(240, 82, 82, 0.2);
}

:global(.modal-overlay) {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.4);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
}

:global(.modal-content) {
  background: var(--color-card);
  border-radius: var(--radius-card);
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.15);
  max-height: 80vh;
  overflow-y: auto;
  min-width: 400px;
  max-width: 90vw;
}
</style>
