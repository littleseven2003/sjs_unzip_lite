/**
 * 通用校验工具函数
 */

/**
 * 校验路径是否为危险目录
 * 基于 design.md 第 7.2 节
 */
export function isDangerousPath(path: string): boolean {
  const normalized = path.replace(/\\/g, "/").toLowerCase();

  const dangerousPatterns = [
    /^\/$/,
    /^\/users$/,
    /^\/applications$/,
    /^\/system$/,
    /^\/library$/,
    /^c:\/$/,
    /^c:\/windows$/,
    /^c:\/program files$/,
    /^c:\/program files \(x86\)$/,
    /^c:\/users$/,
  ];

  return dangerousPatterns.some((pattern) => pattern.test(normalized));
}
