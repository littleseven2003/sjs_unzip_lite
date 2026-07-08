<script setup lang="ts">
/**
 * FolderPicker — 文件夹选择组件
 * 职责：选择根文件夹、展示路径、推断默认最终文件夹名、校验危险目录
 * 基于 design.md 第 14.3 节
 */
import { ref, computed } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";

const folderPath = ref("");
const finalFolderName = ref("");
const loading = ref(false);
const errorMessage = ref("");

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

/** 选择文件夹 */
async function selectFolder(): Promise<void> {
  try {
    errorMessage.value = "";
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
async function previewTask(): Promise<void> {
  if (!folderPath.value) return;

  loading.value = true;
  errorMessage.value = "";

  try {
    const result = await invoke("preview_task", {
      input: {
        root_dir: folderPath.value,
        final_folder_name: finalFolderName.value || null,
        continue_on_initial_extra_files: false,
      },
    });
    // TODO: 处理预检查结果，展示给用户
    console.log("preview result:", result);
  } catch (err) {
    errorMessage.value = `${err}`;
  } finally {
    loading.value = false;
  }
}

/** 开始处理 */
async function startTask(): Promise<void> {
  if (!folderPath.value) return;

  loading.value = true;
  errorMessage.value = "";

  try {
    await invoke("start_task", {
      input: {
        root_dir: folderPath.value,
        final_folder_name: finalFolderName.value || null,
        continue_on_initial_extra_files: false,
      },
    });
  } catch (err) {
    errorMessage.value = `${err}`;
  } finally {
    loading.value = false;
  }
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
          :placeholder="placeholderText"
        />
      </div>
      <p class="form-hint">处理过程中会移动、删除和重命名文件。建议提前备份原始文件夹。</p>
    </div>

    <!-- 错误提示 -->
    <p v-if="errorMessage" class="error-message">{{ errorMessage }}</p>

    <!-- 操作按钮 -->
    <div class="form-actions">
      <button class="btn btn-secondary" :disabled="!folderPath || loading" @click="previewTask">
        预检查
      </button>
      <button
        class="btn btn-primary"
        :disabled="!folderPath || loading"
        @click="startTask"
      >
        {{ loading ? "处理中..." : "开始处理" }}
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

.form-hint {
  font-size: 12px;
  color: var(--color-text-muted);
  line-height: 1.4;
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

.error-message {
  font-size: 13px;
  color: var(--color-danger);
  padding: 8px 12px;
  background: rgba(240, 82, 82, 0.08);
  border-radius: 8px;
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

.btn-secondary:hover:not(:disabled) {
  background: rgba(255, 255, 255, 0.95);
}
</style>
