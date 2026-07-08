# sjs-unzip-tool

自动整理 7z 分卷压缩包，并连续处理伪装为 txt 的 rar 文件的桌面工具。

## 功能

- 自动扫描并归集散落在子文件夹中的 7z 分卷文件
- 支持密码表自动尝试解压
- 自动识别伪装为 .txt 的 .rar 文件并继续解压
- 处理完成后将文件夹重命名为用户指定名称
- 内置解压组件，无需安装 7-Zip 或 WinRAR

## 技术栈

- 桌面框架：Tauri 2
- 前端：Vue 3 + Vite + TypeScript
- 后端：Rust
- 解压工具：7-Zip（sidecar 打包）

## 支持平台

- Windows x86_64
- macOS Intel (x86_64)
- macOS Apple Silicon (aarch64)

## 开发

```bash
# 安装依赖
pnpm install

# 启动开发服务
pnpm tauri dev

# 构建
pnpm tauri build
```

## 许可证

本项目使用 MIT 许可证。

内置 7-Zip 组件使用 GNU LGPL 许可证，详见 [docs/THIRD_PARTY_NOTICES.md](docs/THIRD_PARTY_NOTICES.md)。
