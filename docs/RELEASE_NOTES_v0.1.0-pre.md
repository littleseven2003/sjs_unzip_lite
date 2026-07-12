# v0.1.0-pre 预发布说明

## 版本性质

`v0.1.0-pre` 为预发布版本，主要用于验证自动构建与三平台产物可用性，**重点测试 Windows 平台工作是否正常**。请勿用于生产环境。

## 产物

| 平台 | Runner | 安装包 |
|------|--------|--------|
| Windows x86_64 | `windows-latest` | `.msi` / `.exe` (NSIS) |
| macOS Intel | `macos-13` | `.dmg` |
| macOS Apple Silicon | `macos-14` | `.dmg` |

内置 7-Zip 解压组件随安装包分发，无需用户额外安装 7-Zip 或 WinRAR。

## 安装注意事项

### Windows
- 首次运行可能被 SmartScreen 拦截，点击「更多信息 → 仍要运行」即可。
- 重点验证：选含 7z 分卷的文件夹 → 预检查 → 开始 → 完成后文件夹重命名是否符合预期。

### macOS（未签名）
- 本预发布版本**未经 Apple 代码签名与公证**。
- 首次打开：右键点击应用 → 「打开」→ 在弹出的安全提示中再次选择「打开」。
- 或在终端执行：`xattr -cr "/Applications/sjs-unzip-tool.app"`

## 测试重点

1. 7z 分卷识别与归集（含子文件夹散落分卷）。
2. 密码表尝试解压（确认日志无明文密码）。
3. txt → rar 伪装解包循环。
4. 根目录重命名为用户指定名称。
5. Windows 下文件占用与长路径场景。

## 已知限制

- **多组分卷选择**与**多个 txt 文件选择**的用户交互流程尚未接通，命中时会以错误弹窗中止。该流程将在后续版本实现。
- 不支持 Linux 平台（设计目标为 Windows 与 macOS）。

## 关联

- 设计文档：[docs/design.md](design.md)
- 变更记录：见 GitHub Release 描述