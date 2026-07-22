<script setup lang="ts">
/**
 * TaskProgress — 任务进度组件
 * 职责：展示当前任务状态、进度条、当前步骤、当前文件、耗时
 * 基于 design.md 第 14.4 节
 */
import { useTaskRunner } from "../composables/useTaskRunner";

const { status, progress, currentStep, detail } = useTaskRunner();
</script>

<template>
  <div class="task-progress">
    <h2 class="section-title">任务状态</h2>

    <div class="status-area">
      <template v-if="status === 'idle'">
        <p class="status-idle">等待开始</p>
      </template>

      <template v-else-if="status === 'completed'">
        <p class="status-completed">✓ 完成</p>
      </template>

      <template v-else-if="status === 'failed'">
        <p class="status-failed">✗ 失败</p>
      </template>

      <template v-else-if="status === 'cancelled'">
        <p class="status-cancelled">已取消</p>
      </template>

      <template v-else>
        <div class="progress-info">
          <div class="progress-header">
            <span class="step-name">{{ currentStep }}</span>
            <span class="progress-percent">{{ progress }}%</span>
          </div>
          <div class="progress-bar">
            <div class="progress-fill" :style="{ width: progress + '%' }" />
          </div>
          <p v-if="detail" class="progress-detail">{{ detail }}</p>
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped>
.task-progress {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.section-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--color-text-main);
}

.status-area {
  min-height: 32px;
  display: flex;
  align-items: center;
}

.status-idle {
  color: var(--color-text-muted);
  font-size: 12px;
}

.status-completed {
  color: var(--color-success);
  font-size: 12px;
  font-weight: 500;
}

.status-failed {
  color: var(--color-danger);
  font-size: 12px;
  font-weight: 500;
}

.status-cancelled {
  color: var(--color-warning);
  font-size: 12px;
  font-weight: 500;
}

.progress-info {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.progress-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.step-name {
  font-size: 11px;
  font-weight: 500;
  color: var(--color-text-main);
}

.progress-percent {
  font-size: 11px;
  color: var(--color-text-muted);
}

.progress-bar {
  width: 100%;
  height: 6px;
  background: rgba(130, 150, 180, 0.12);
  border-radius: 3px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #6c8cff, #7c6cff);
  border-radius: 3px;
  transition: width 0.3s ease;
}

.progress-detail {
  font-size: 10px;
  color: var(--color-text-muted);
}
</style>
