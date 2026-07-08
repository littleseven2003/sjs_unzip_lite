/**
 * 任务相关类型定义
 * 基于 design.md 第 8、9 节
 */

/** 任务状态枚举 */
export type TaskStatus =
  | "idle"
  | "scanning"
  | "warning"
  | "moving_volumes"
  | "cleaning_folders"
  | "extracting_7z"
  | "deleting_volumes"
  | "finding_txt"
  | "renaming_txt_to_rar"
  | "cleaning_except_rar"
  | "extracting_rar"
  | "deleting_rar"
  | "renaming_root"
  | "completed"
  | "failed"
  | "cancelled";

/** 任务输入参数 */
export interface TaskInput {
  rootDir: string;
  finalFolderName?: string;
  continueOnInitialExtraFiles: boolean;
  selectedVolumeGroupId?: string;
  selectedTxtPath?: string;
}

/** 任务预览结果 */
export interface TaskPreview {
  rootDir: string;
  defaultFinalFolderName: string;
  volumeGroups: VolumeGroupPreview[];
  extraFiles: FilePreview[];
  extraFolders: FilePreview[];
  warnings: WarningItem[];
  canStart: boolean;
}

/** 分卷组预览 */
export interface VolumeGroupPreview {
  id: string;
  baseName: string;
  firstVolumePath: string;
  volumeCount: number;
  totalSize: number;
  missingIndexes: number[];
  duplicateIndexes: number[];
  files: VolumeFilePreview[];
}

/** 分卷文件预览 */
export interface VolumeFilePreview {
  path: string;
  index: number;
  size: number;
}

/** 文件预览 */
export interface FilePreview {
  path: string;
  name: string;
  size: number;
  isDir: boolean;
}

/** 警告项 */
export interface WarningItem {
  code: string;
  message: string;
  detail?: string;
}

/** 进度事件 */
export interface ProgressEvent {
  status: TaskStatus;
  stepName: string;
  progress: number;
  current?: number;
  total?: number;
  detail?: string;
}
