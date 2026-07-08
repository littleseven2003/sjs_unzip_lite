<script setup lang="ts">
/**
 * WarningDialog — 警告弹窗组件
 * 职责：展示危险操作确认、展示多选项确认、接收用户选择结果
 * 基于 design.md 第 14.6 节
 */
import { ref } from "vue";

export interface WarningDialogProps {
  title: string;
  message: string;
  detail?: string;
  type?: "confirm" | "select";
  options?: Array<{ label: string; value: string; danger?: boolean }>;
  confirmLabel?: string;
  cancelLabel?: string;
}

const props = withDefaults(defineProps<WarningDialogProps>(), {
  type: "confirm",
  confirmLabel: "继续处理",
  cancelLabel: "取消",
  options: () => [],
});

const emit = defineEmits<{
  confirm: [];
  cancel: [];
  select: [value: string];
}>();

const selectedValue = ref<string | null>(null);

function handleConfirm(): void {
  if (props.type === "select" && selectedValue.value) {
    emit("select", selectedValue.value);
  } else {
    emit("confirm");
  }
}

function handleCancel(): void {
  emit("cancel");
}

function handleSelect(value: string): void {
  selectedValue.value = value;
}
</script>

<template>
  <div class="warning-dialog">
    <div class="dialog-header">
      <h3 class="dialog-title">{{ title }}</h3>
    </div>

    <div class="dialog-body">
      <p class="dialog-message">{{ message }}</p>
      <p v-if="detail" class="dialog-detail">{{ detail }}</p>

      <!-- 多选项模式 -->
      <div v-if="type === 'select' && options.length > 0" class="options-list">
        <div
          v-for="option in options"
          :key="option.value"
          class="option-item"
          :class="{
            selected: selectedValue === option.value,
            danger: option.danger,
          }"
          @click="handleSelect(option.value)"
        >
          <span class="option-radio">
            <span v-if="selectedValue === option.value" class="radio-checked" />
          </span>
          <span class="option-label">{{ option.label }}</span>
        </div>
      </div>
    </div>

    <div class="dialog-footer">
      <button class="btn btn-secondary" @click="handleCancel">
        {{ cancelLabel }}
      </button>
      <button
        class="btn"
        :class="type === 'select' ? 'btn-primary' : 'btn-warning'"
        :disabled="type === 'select' && !selectedValue"
        @click="handleConfirm"
      >
        {{ type === "select" ? "确认选择" : confirmLabel }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.warning-dialog {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 24px;
  max-width: 480px;
  width: 100%;
}

.dialog-header {
  display: flex;
  align-items: center;
  gap: 12px;
}

.dialog-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-main);
}

.dialog-body {
  display: flex;
  flex-direction: column;
  gap: 12px;
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
}

.options-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-top: 8px;
}

.option-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  border: 1px solid var(--color-card-border);
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.2s;
}

.option-item:hover {
  background: rgba(108, 140, 255, 0.05);
  border-color: var(--color-primary);
}

.option-item.selected {
  background: rgba(108, 140, 255, 0.08);
  border-color: var(--color-primary);
}

.option-item.danger {
  border-color: rgba(240, 82, 82, 0.2);
}

.option-item.danger.selected {
  background: rgba(240, 82, 82, 0.05);
  border-color: var(--color-danger);
}

.option-radio {
  width: 18px;
  height: 18px;
  border: 2px solid var(--color-card-border);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.option-item.selected .option-radio {
  border-color: var(--color-primary);
}

.option-item.danger.selected .option-radio {
  border-color: var(--color-danger);
}

.radio-checked {
  width: 10px;
  height: 10px;
  background: var(--color-primary);
  border-radius: 50%;
}

.option-item.danger .radio-checked {
  background: var(--color-danger);
}

.option-label {
  font-size: 14px;
  color: var(--color-text-main);
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

.btn-warning {
  background: rgba(245, 165, 36, 0.1);
  color: var(--color-warning);
  border: 1px solid rgba(245, 165, 36, 0.2);
}

.btn-warning:hover:not(:disabled) {
  background: rgba(245, 165, 36, 0.2);
}
</style>
