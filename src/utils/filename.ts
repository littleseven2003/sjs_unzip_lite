/**
 * 文件名工具函数
 * 基于 design.md 第 13.2 节
 */

/** Windows 保留名称 */
const WINDOWS_RESERVED_NAMES = new Set([
  "CON",
  "PRN",
  "AUX",
  "NUL",
  "COM1",
  "COM2",
  "COM3",
  "COM4",
  "COM5",
  "COM6",
  "COM7",
  "COM8",
  "COM9",
  "LPT1",
  "LPT2",
  "LPT3",
  "LPT4",
  "LPT5",
  "LPT6",
  "LPT7",
  "LPT8",
  "LPT9",
]);

/** Windows 非法字符 */
const WINDOWS_INVALID_CHARS = /[<>:"|?*]/;

/**
 * 校验文件夹名是否合法
 * @returns 错误信息，合法时返回 null
 */
export function validateFolderName(name: string): string | null {
  const trimmed = name.trim();

  if (trimmed === "") {
    return "文件夹名不能为空";
  }

  if (trimmed === "." || trimmed === "..") {
    return "文件夹名不能为 . 或 ..";
  }

  if (trimmed.includes("/") || trimmed.includes("\\")) {
    return "文件夹名不能包含路径分隔符";
  }

  if (WINDOWS_INVALID_CHARS.test(trimmed)) {
    return "文件夹名包含非法字符";
  }

  if (WINDOWS_RESERVED_NAMES.has(trimmed.toUpperCase())) {
    return "文件夹名为系统保留名称";
  }

  return null;
}
