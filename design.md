# design.md

# 小工具设计文档：分卷压缩包递归解包与整理工具

## 1. 项目定位

本工具用于处理一种特定的压缩包整理场景：用户选择一个包含多个 `7z` 分卷文件的文件夹，分卷文件可能散落在多个子文件夹中。程序需要自动将分卷文件归集到根目录，清理解压过程中的临时文件，按照固定规则反复识别伪装为 `.txt` 的 `.rar` 文件并解压，最终将根目录重命名为用户指定的最终文件夹名。

工具面向普通用户使用，不要求用户安装 7-Zip、WinRAR、UnRAR、命令行工具或任何额外运行环境。所有必要的解压工具应随程序一同打包，程序内部调用。

工具应支持：

- Windows
- macOS Intel
- macOS Apple Silicon

---

## 2. 核心功能范围

### 2.1 用户输入

用户需要在界面中完成以下输入：

1. 选择待处理的根文件夹。
2. 输入最终文件夹名。
3. 可选：不输入最终文件夹名时，默认使用用户所选根文件夹当前名称。
4. 可选：查看和维护解压密码表。
5. 点击“开始处理”。

### 2.2 自动处理流程

程序需要完成以下流程：

1. 扫描用户选择的根文件夹。
2. 在根文件夹及所有子文件夹中查找 `7z` 分卷文件，例如：
   - `.7z.001`
   - `.001`
   - `.7z.002`
   - `.002`
   - `.7z.003`
   - `.003`
3. 将所有分卷文件移动到用户选择的根目录下。
4. 检查根目录是否存在分卷文件之外的其他文件。
5. 如存在其他文件，弹出警告，提示用户是否继续。
6. 删除分卷文件来源路径中的空文件夹。
7. 调用内置解压工具，解压分卷压缩包到根目录。
8. 解压成功后删除原始分卷文件。
9. 在根目录当前解压结果中查找 `.txt` 文件。
10. 将找到的目标 `.txt` 文件改名为 `.rar` 文件。
11. 删除除了该 `.rar` 文件之外的其他文件和文件夹。
12. 调用内置解压工具解压该 `.rar` 文件到根目录。
13. 解压成功后删除该 `.rar` 文件。
14. 重复执行 `.txt` → `.rar` → 解压 → 清理 的流程，直到无法再找到符合条件的 `.txt` 文件。
15. 将根文件夹重命名为用户指定的最终文件夹名。
16. 展示完成提示。

---

## 3. 推荐技术栈

### 3.1 总体技术栈

推荐使用：

- 桌面框架：Tauri 2
- 前端框架：Vue 3
- 构建工具：Vite
- UI 组件：UnoCSS / Tailwind CSS + 自定义组件
- 前端语言：TypeScript
- 后端语言：Rust
- 解压工具：随应用打包的 7-Zip 命令行二进制
- 配置文件：JSON
- 日志：Rust 后端事件流 + 前端滚动日志面板

### 3.2 选择理由

#### Tauri 2

Tauri 适合该工具的原因：

- 支持 Windows 和 macOS 桌面应用。
- 前端可以使用现代 Web 技术实现更美观的界面。
- 后端使用 Rust，适合做文件系统操作、进程调用、路径安全校验。
- 安装包体积通常比 Electron 更轻。
- 可以通过 Tauri sidecar 能力打包外部命令行程序，例如 7-Zip。
- 前后端职责清晰：前端负责交互和展示，后端负责文件操作、解压、清理和日志事件。

#### Vue 3 + Vite + TypeScript

适合该工具的原因：

- 开发效率高。
- UI 状态管理简单。
- 适合构建小型桌面工具。
- TypeScript 便于维护任务状态、日志类型、错误类型、进度事件。
- 后续扩展密码管理、历史记录、主题切换等功能较方便。

#### Rust 后端

Rust 后端负责：

- 扫描文件夹。
- 移动分卷文件。
- 删除文件和文件夹。
- 调用解压命令。
- 管理解压密码尝试逻辑。
- 解析外部命令退出码。
- 处理跨平台路径。
- 发送进度和日志事件给前端。
- 防止误删用户目录中的重要文件。

Rust 后端不应把核心文件操作逻辑放在前端，避免路径权限、异常处理和安全校验分散。

#### 7-Zip 命令行工具

解压工具推荐使用 7-Zip 命令行版本，作为应用 sidecar 打包。

原因：

- 能处理 `7z` 分卷压缩包。
- 能处理 `.rar` 解压。
- 支持命令行密码参数。
- Windows 和 macOS 都可以打包对应平台的二进制。
- 用户不需要单独安装任何工具。
- 不需要程序直接实现复杂压缩格式解析。

需要注意：

- 7-Zip 对 RAR 的支持是解压，不是创建 RAR。
- 本工具只需要解压 RAR，因此适合使用。
- 项目文档中应保留 7-Zip 许可证说明。
- 不应将 7-Zip 的 unRAR 相关代码用于重建 RAR 压缩算法。
- 应随应用附带 `license.txt` 或 `THIRD_PARTY_NOTICES.md`。

---

## 4. 项目目录结构

推荐目录结构如下：

```text
sjs-unzip-tool/
├─ package.json
├─ pnpm-lock.yaml
├─ index.html
├─ vite.config.ts
├─ tsconfig.json
├─ src/
│  ├─ main.ts
│  ├─ App.vue
│  ├─ styles/
│  │  ├─ index.css
│  │  ├─ theme.css
│  │  └─ animations.css
│  ├─ components/
│  │  ├─ AppShell.vue
│  │  ├─ FolderPicker.vue
│  │  ├─ PasswordManager.vue
│  │  ├─ TaskProgress.vue
│  │  ├─ LogPanel.vue
│  │  ├─ WarningDialog.vue
│  │  └─ ResultDialog.vue
│  ├─ composables/
│  │  ├─ useTaskRunner.ts
│  │  ├─ useLogEvents.ts
│  │  ├─ usePasswordStore.ts
│  │  └─ useTheme.ts
│  ├─ types/
│  │  ├─ task.ts
│  │  ├─ log.ts
│  │  └─ password.ts
│  └─ utils/
│     ├─ filename.ts
│     └─ validation.ts
├─ src-tauri/
│  ├─ Cargo.toml
│  ├─ tauri.conf.json
│  ├─ build.rs
│  ├─ capabilities/
│  │  └─ default.json
│  ├─ binaries/
│  │  ├─ 7zz-x86_64-pc-windows-msvc.exe
│  │  ├─ 7zz-aarch64-apple-darwin
│  │  └─ 7zz-x86_64-apple-darwin
│  ├─ src/
│  │  ├─ main.rs
│  │  ├─ commands.rs
│  │  ├─ task/
│  │  │  ├─ mod.rs
│  │  │  ├─ context.rs
│  │  │  ├─ scanner.rs
│  │  │  ├─ mover.rs
│  │  │  ├─ extractor.rs
│  │  │  ├─ cleaner.rs
│  │  │  ├─ renamer.rs
│  │  │  ├─ password.rs
│  │  │  └─ safety.rs
│  │  ├─ events/
│  │  │  ├─ mod.rs
│  │  │  ├─ log_event.rs
│  │  │  └─ progress_event.rs
│  │  ├─ config/
│  │  │  ├─ mod.rs
│  │  │  └─ password_config.rs
│  │  └─ error/
│  │     ├─ mod.rs
│  │     └─ app_error.rs
│  └─ icons/
├─ docs/
│  ├─ design.md
│  ├─ THIRD_PARTY_NOTICES.md
│  └─ test_cases.md
└─ README.md
```

---

## 5. 配置文件设计

### 5.1 密码表文件位置

密码表应存储在应用数据目录中，而不是项目目录中。

不同平台推荐路径：

```text
Windows:
%APPDATA%/sjs-unzip-tool/passwords.json

macOS:
~/Library/Application Support/sjs-unzip-tool/passwords.json
```

### 5.2 密码表 JSON 格式

```json
{
  "version": 1,
  "passwords": [
    "",
    "123456",
    "password1",
    "password2"
  ],
  "updated_at": "2026-07-08T10:00:00+09:00"
}
```

### 5.3 密码规则

密码尝试顺序：

1. 先尝试空密码。
2. 再按照 `passwords` 数组顺序逐个尝试。
3. 如果用户在密码表中手动填写了空字符串，也只保留一个空密码尝试。
4. 密码中的前后空格默认保留，除非用户在界面中明确点击“去除首尾空格”。
5. 密码表不应上传、不应联网同步。

### 5.4 密码表维护界面

密码管理界面应支持：

- 查看密码条目数量。
- 新增密码。
- 删除密码。
- 调整密码顺序。
- 清空密码表。
- 导入 JSON。
- 导出 JSON。
- 保存修改。
- 密码默认以圆点隐藏。
- 提供“显示 / 隐藏密码”按钮。
- 提醒用户密码保存在本机。

---

## 6. 文件识别规则

### 6.1 分卷文件识别

需要识别以下常见情况：

```text
xxx.7z.001
xxx.7z.002
xxx.7z.003

xxx.001
xxx.002
xxx.003
```

推荐正则：

```regex
(?i)^(.+?)(?:\.7z)?\.(\d{3})$
```

说明：

- `(?i)` 表示忽略大小写。
- 文件名末尾必须是三位数字，例如 `001`、`002`、`003`。
- `.7z` 可以存在，也可以不存在。
- `001` 是解压入口文件。
- `002`、`003` 等是后续分卷。

### 6.2 分卷组识别

扫描到分卷后，需要按基础名分组。

例如：

```text
Movie.7z.001
Movie.7z.002
Movie.7z.003
```

基础名为：

```text
Movie
```

又如：

```text
Archive.001
Archive.002
Archive.003
```

基础名为：

```text
Archive
```

### 6.3 分卷完整性检查

程序应至少检查：

- 是否存在 `001`。
- 分卷编号是否连续。
- 是否存在重复编号。
- 同一分卷组中是否存在多个来源路径冲突。
- 根目录是否已经存在同名分卷文件。
- 文件大小是否为 0。

如果存在明显问题，应阻止继续并提示用户。

### 6.4 多组分卷处理

如果扫描到多组分卷，默认不自动处理，应弹窗让用户选择：

- 处理其中一组。
- 取消任务。

原因是本工具流程假设根目录中只有一个主分卷压缩包，后续会删除其他文件。如果自动处理多组分卷，容易误删用户数据。

### 6.5 `.txt` 文件识别

解压后需要查找一个 `.txt` 文件，并将其改名为 `.rar`。

需要注意：

- 只处理根目录当前层级下的 `.txt`，还是递归搜索 `.txt`，必须明确。
- 推荐递归搜索，但在找到目标后将其移动到根目录。
- 如果找到多个 `.txt` 文件，应弹窗让用户选择目标文件，不能自动随意选择。
- 如果找不到 `.txt` 文件，则认为循环解包结束。
- 如果 `.txt` 文件大小为 0，应提示异常。
- 如果 `.txt` 文件已经无法通过 7-Zip 识别为压缩包，应提示错误并保留现场。

### 6.6 文件格式校验

虽然流程是将 `.txt` 改名为 `.rar`，但程序不应只依赖扩展名判断。

建议在改名后调用：

```bash
7zz l target.rar
```

用于测试该文件是否能被识别为压缩包。

如果识别失败：

- 记录日志。
- 弹窗提示。
- 停止任务。
- 不继续删除其他文件。

---

## 7. 安全策略

本工具包含大量移动、删除、重命名操作，必须有严格安全边界。

### 7.1 工作目录限制

所有文件操作必须限制在用户选择的根目录内部。

禁止操作：

- 根目录的父目录。
- 用户主目录。
- 系统目录。
- 应用程序目录。
- 桌面根目录本身。
- 磁盘根目录。

### 7.2 禁止选择的目录

用户选择以下目录时应阻止：

Windows：

```text
C:\
C:\Windows
C:\Program Files
C:\Program Files (x86)
C:\Users
```

macOS：

```text
/
~/ 
/Users
/Applications
/System
/Library
```

如果用户确实需要处理桌面上的某个文件夹，应选择具体子文件夹，而不是选择整个桌面。

### 7.3 删除策略

删除操作应使用“安全删除”策略：

- 默认直接移动到系统回收站 / 废纸篓。
- 如果跨平台回收站实现复杂，可以提供“直接删除”但必须二次确认。
- 程序内部临时文件可以直接删除。
- 关键删除动作前应记录将要删除的路径。
- 删除前必须再次校验路径仍在工作目录内。
- 禁止跟随符号链接删除外部路径。

### 7.4 符号链接处理

扫描时如遇到符号链接：

- 默认跳过。
- 日志中记录。
- 不进入符号链接目录。
- 不删除符号链接指向的真实路径。

### 7.5 警告弹窗触发条件

以下情况必须弹窗：

- 初始根目录下存在分卷文件之外的其他文件。
- 初始根目录下存在非空子文件夹。
- 找到多个分卷组。
- 分卷编号不连续。
- 找到多个 `.txt` 候选文件。
- 目标最终文件夹名已存在同名兄弟文件夹。
- 解压需要密码，但所有密码均失败。
- 解压命令返回错误。
- 删除文件失败。
- 重命名根目录失败。
- 用户选择的根目录风险过高。

---

## 8. 任务状态机设计

### 8.1 状态枚举

```ts
type TaskStatus =
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
```

### 8.2 任务上下文

```rust
pub struct TaskContext {
    pub root_dir: PathBuf,
    pub final_folder_name: String,
    pub password_list: Vec<String>,
    pub selected_volume_group: Option<VolumeGroup>,
    pub current_archive: Option<PathBuf>,
    pub current_iteration: u32,
    pub max_iterations: u32,
    pub dry_run: bool,
}
```

### 8.3 最大循环次数

`.txt` → `.rar` → 解压 的流程理论上可能异常循环。

建议设置：

```text
max_iterations = 20
```

如果超过 20 次仍然继续发现 `.txt` 文件，应停止任务并提示：

```text
检测到过多层嵌套压缩包，已停止处理。请检查文件是否异常。
```

---

## 9. 后端命令接口设计

### 9.1 前端调用后端命令

Tauri 后端提供以下命令：

```rust
#[tauri::command]
async fn get_app_config() -> Result<AppConfig, AppError>;

#[tauri::command]
async fn load_passwords() -> Result<PasswordConfig, AppError>;

#[tauri::command]
async fn save_passwords(config: PasswordConfig) -> Result<(), AppError>;

#[tauri::command]
async fn preview_task(input: TaskInput) -> Result<TaskPreview, AppError>;

#[tauri::command]
async fn start_task(input: TaskInput) -> Result<(), AppError>;

#[tauri::command]
async fn cancel_task() -> Result<(), AppError>;

#[tauri::command]
async fn open_log_folder() -> Result<(), AppError>;
```

### 9.2 TaskInput

```ts
interface TaskInput {
  rootDir: string;
  finalFolderName?: string;
  continueOnInitialExtraFiles: boolean;
  selectedVolumeGroupId?: string;
  selectedTxtPath?: string;
}
```

### 9.3 TaskPreview

开始处理前，先生成预览信息：

```ts
interface TaskPreview {
  rootDir: string;
  defaultFinalFolderName: string;
  volumeGroups: VolumeGroupPreview[];
  extraFiles: FilePreview[];
  extraFolders: FilePreview[];
  warnings: WarningItem[];
  canStart: boolean;
}
```

### 9.4 VolumeGroupPreview

```ts
interface VolumeGroupPreview {
  id: string;
  baseName: string;
  firstVolumePath: string;
  volumeCount: number;
  totalSize: number;
  missingIndexes: number[];
  duplicateIndexes: number[];
  files: VolumeFilePreview[];
}
```

---

## 10. 进度与日志事件设计

### 10.1 进度事件

Rust 后端向前端发送进度事件：

```ts
interface ProgressEvent {
  status: TaskStatus;
  stepName: string;
  progress: number;
  current?: number;
  total?: number;
  detail?: string;
}
```

示例：

```json
{
  "status": "extracting_rar",
  "stepName": "正在解压 RAR 文件",
  "progress": 68,
  "detail": "正在尝试密码：******"
}
```

### 10.2 日志事件

```ts
type LogLevel = "info" | "success" | "warning" | "error";

interface LogEvent {
  time: string;
  level: LogLevel;
  message: string;
  detail?: string;
}
```

示例：

```json
{
  "time": "2026-07-08 10:30:15",
  "level": "info",
  "message": "已找到 3 个分卷文件",
  "detail": "Archive.7z.001, Archive.7z.002, Archive.7z.003"
}
```

### 10.3 日志展示规则

前端日志面板应：

- 支持自动滚动到底部。
- 支持暂停自动滚动。
- 支持复制全部日志。
- 支持清空界面日志。
- 支持打开本地日志文件夹。
- 错误日志使用醒目但不刺眼的样式。
- 警告日志使用柔和黄色。
- 成功日志使用柔和绿色。

### 10.4 本地日志文件

每次任务生成一个日志文件：

```text
~/Library/Application Support/sjs-unzip-tool/logs/2026-07-08_10-30-15.log
%APPDATA%/sjs-unzip-tool/logs/2026-07-08_10-30-15.log
```

日志文件中应包含：

- 程序版本。
- 操作系统。
- 根目录路径。
- 最终文件夹名。
- 分卷识别结果。
- 解压命令退出码。
- 错误详情。
- 任务开始和结束时间。

密码不能明文写入日志。

---

## 11. 解压命令设计

### 11.1 7z 分卷解压

只对 `001` 文件执行解压命令。

示例：

```bash
7zz x "/path/to/archive.7z.001" -o"/path/to/root" -y
```

如需密码：

```bash
7zz x "/path/to/archive.7z.001" -o"/path/to/root" -y -p"password"
```

### 11.2 RAR 解压

```bash
7zz x "/path/to/archive.rar" -o"/path/to/root" -y
```

如需密码：

```bash
7zz x "/path/to/archive.rar" -o"/path/to/root" -y -p"password"
```

### 11.3 密码尝试流程

伪代码：

```rust
fn extract_with_passwords(archive: &Path, output_dir: &Path, passwords: &[String]) -> Result<ExtractResult, AppError> {
    let mut candidates = normalize_passwords(passwords);

    for password in candidates {
        emit_log("info", format!("正在尝试解压：{}", archive.display()));

        let result = run_7zz_extract(archive, output_dir, Some(password));

        if result.success {
            emit_log("success", "解压成功");
            return Ok(result);
        }

        if result.is_wrong_password {
            emit_log("warning", "密码不正确，继续尝试下一个密码");
            continue;
        }

        if result.is_not_password_error {
            return Err(AppError::ExtractFailed(result.stderr));
        }
    }

    Err(AppError::PasswordFailed)
}
```

### 11.4 密码错误识别

7-Zip 输出可能包含以下信息：

```text
Wrong password
Can not open encrypted archive. Wrong password?
Headers Error
Data Error
```

程序不能只依赖一种文本判断，应结合：

- 退出码。
- stderr / stdout 文本。
- 是否产生了有效解压结果。
- 是否可以通过 `7zz t archive` 测试通过。

### 11.5 解压前测试

对于每个压缩包，可先执行：

```bash
7zz t "archive.rar" -p"password"
```

测试通过后再执行解压。

但为了减少耗时，也可以直接执行解压。如果解压失败再尝试下一个密码。

推荐策略：

- 小文件：先测试再解压。
- 大文件：直接解压，失败后清理残留再试下一个密码。
- 具体实现中可统一采用直接解压，以减少复杂度。
- 每次密码失败后，应删除该次失败产生的不完整输出。

### 11.6 解压输出隔离

为了避免错误密码或解压失败产生残留文件，建议每次解压先输出到临时目录：

```text
/root/.sjs_unzip_temp/extract_attempt_xxx/
```

解压成功后再移动到根目录。

流程：

1. 创建临时目录。
2. 解压到临时目录。
3. 解压成功后清空根目录中应被替换的旧内容。
4. 将临时目录中的内容移动到根目录。
5. 删除临时目录。

这样可以避免错误密码导致根目录出现不完整文件。

---

## 12. 清理规则设计

### 12.1 初始阶段清理

在分卷文件移动到根目录后：

- 删除原分卷所在路径上的空文件夹。
- 不删除非空文件夹，除非用户确认。
- 如果发现非空文件夹，应记录警告。
- 如果根目录下有非分卷文件，应弹窗确认。

### 12.2 解压 7z 后清理

解压成功后：

- 删除所有分卷文件。
- 保留解压结果。
- 保留日志文件。
- 保留程序内部临时目录之外的结果文件。

### 12.3 `.txt` 改名 `.rar` 前清理

找到目标 `.txt` 后：

1. 将目标 `.txt` 移动到根目录。
2. 改名为 `.rar`。
3. 删除根目录中除了该 `.rar` 文件以外的所有文件和文件夹。
4. 删除前必须确保 `.rar` 文件存在且可读。
5. 删除前可以调用 `7zz l` 校验该 `.rar` 是否为可识别压缩包。

### 12.4 解压 RAR 后清理

RAR 解压成功后：

- 删除当前 `.rar` 文件。
- 保留解压结果。
- 继续查找下一个 `.txt` 文件。

### 12.5 结束条件

满足以下任一条件时结束循环：

- 当前解压结果中找不到 `.txt` 文件。
- 找到的 `.txt` 文件不是可识别压缩包。
- 用户取消任务。
- 达到最大循环次数。
- 解压失败。
- 删除失败。
- 重命名失败。

---

## 13. 根目录重命名设计

### 13.1 默认最终文件夹名

如果用户不填写最终文件夹名：

```text
最终文件夹名 = 用户选择的根文件夹当前名称
```

这种情况下最后不需要实际重命名。

### 13.2 用户指定最终文件夹名

如果用户填写了最终文件夹名：

- 去除首尾空白字符。
- 禁止空字符串。
- 禁止路径分隔符：
  - `/`
  - `\`
- 禁止 Windows 非法字符：
  - `<`
  - `>`
  - `:`
  - `"`
  - `|`
  - `?`
  - `*`
- 禁止名称为：
  - `.`
  - `..`
- Windows 下避免保留名称：
  - `CON`
  - `PRN`
  - `AUX`
  - `NUL`
  - `COM1` 至 `COM9`
  - `LPT1` 至 `LPT9`

### 13.3 同名冲突

如果父目录下已经存在同名文件夹：

弹窗提示：

```text
目标文件夹名已存在，请修改最终文件夹名后重试。
```

不建议自动追加 `(1)`，因为用户明确指定了最终文件夹名，自动改名容易造成混淆。

### 13.4 重命名失败

可能原因：

- 文件夹被其他程序占用。
- 没有权限。
- 同名文件夹已存在。
- 路径过长。
- macOS Finder 正在占用。
- Windows 杀毒软件正在扫描。

处理方式：

- 停止任务。
- 保留当前文件夹。
- 弹窗提示。
- 日志记录错误详情。

---

## 14. 前端界面设计

### 14.1 视觉风格

界面参考 `https://bfzy.littleseven.me/` 的整体方向，采用：

- 明亮浅色系。
- 柔和背景渐变。
- 白色或半透明卡片。
- 圆角组件。
- 轻阴影。
- 清晰的主按钮。
- 柔和的蓝紫色或青蓝色强调色。
- 不使用强烈红色大面积警告。
- 不做复杂布局，突出“选择文件夹 → 设置名称 → 开始处理”。

### 14.2 页面布局

主界面分为四个区域：

```text
┌────────────────────────────────────────────┐
│ 顶部标题区                                  │
│ 小工具名称 / 简短说明 / 设置按钮             │
├────────────────────────────────────────────┤
│ 主操作卡片                                  │
│ 选择文件夹 / 最终文件夹名 / 密码表入口        │
├────────────────────────────────────────────┤
│ 任务状态卡片                                │
│ 当前步骤 / 进度条 / 状态提示                 │
├────────────────────────────────────────────┤
│ 滚动日志区                                  │
│ 日志列表 / 复制日志 / 清空 / 打开日志文件夹   │
└────────────────────────────────────────────┘
```

### 14.3 主操作卡片

包含：

- 文件夹路径输入框，只读。
- “选择文件夹”按钮。
- 最终文件夹名输入框。
- 默认名称提示：
  ```text
  留空则使用所选文件夹当前名称
  ```
- “管理密码表”按钮。
- “预检查”按钮。
- “开始处理”按钮。
- “取消任务”按钮。

按钮状态：

- 未选择文件夹时，“开始处理”禁用。
- 任务运行中，“选择文件夹”和“开始处理”禁用。
- 任务运行中，“取消任务”可用。
- 任务结束后，可以重新开始。

### 14.4 任务状态卡片

显示内容：

- 当前状态标题。
- 当前处理文件名。
- 总进度条。
- 子步骤进度。
- 已耗时。
- 当前循环次数。
- 当前尝试密码序号。
- 轻量动画图标。

示例：

```text
正在解压 RAR 文件
当前文件：stage_02.rar
正在尝试密码 3 / 8
████████████████░░░░ 72%
```

### 14.5 日志卡片

日志展示示例：

```text
[10:30:12] INFO    已选择文件夹：D:\Downloads\archive
[10:30:13] INFO    已找到 3 个分卷文件
[10:30:15] SUCCESS 分卷文件已移动到根目录
[10:30:20] INFO    正在解压 Archive.7z.001
[10:31:01] SUCCESS 解压成功
[10:31:02] INFO    已删除原始分卷文件
[10:31:03] INFO    已找到 txt 文件：stage_01.txt
[10:31:04] SUCCESS 已改名为 stage_01.rar
```

### 14.6 弹窗设计

#### 初始异常文件警告

标题：

```text
检测到额外文件
```

内容：

```text
所选文件夹中检测到除 7z 分卷文件以外的其他文件或非空文件夹。
继续处理可能会在后续清理步骤中删除这些内容。

建议先备份该文件夹。
```

按钮：

```text
取消
继续处理
```

默认按钮：取消。

#### 多个 `.txt` 文件选择弹窗

标题：

```text
找到多个 txt 文件
```

内容：

```text
请选择需要改名为 RAR 并继续解压的文件。
```

列表展示：

- 文件名
- 路径
- 文件大小
- 修改时间

按钮：

```text
取消任务
确认选择
```

#### 密码失败弹窗

标题：

```text
解压失败
```

内容：

```text
密码表中的所有密码均尝试失败，无法解压当前压缩包。
请更新密码表后重新处理。
```

按钮：

```text
打开密码表
结束任务
```

#### 完成弹窗

标题：

```text
处理完成
```

内容：

```text
文件已整理完成，并已重命名为：xxx
```

按钮：

```text
打开文件夹
完成
```

---

## 15. UI 组件设计

### 15.1 AppShell

职责：

- 提供整体页面结构。
- 管理主题背景。
- 管理全局弹窗挂载点。
- 管理窗口标题栏区域。

### 15.2 FolderPicker

职责：

- 选择根文件夹。
- 展示路径。
- 自动推断默认最终文件夹名。
- 校验危险目录。

### 15.3 PasswordManager

职责：

- 加载密码表。
- 展示密码列表。
- 新增、删除、排序密码。
- 导入导出 JSON。
- 保存密码表。

### 15.4 TaskProgress

职责：

- 展示当前任务状态。
- 展示进度条。
- 展示当前步骤。
- 展示当前文件。
- 展示耗时。

### 15.5 LogPanel

职责：

- 展示滚动日志。
- 自动滚动。
- 暂停滚动。
- 复制日志。
- 清空日志。
- 打开日志文件夹。

### 15.6 WarningDialog

职责：

- 展示危险操作确认。
- 展示多选项确认。
- 接收用户选择结果。
- 把用户选择传回任务流程。

### 15.7 ResultDialog

职责：

- 展示任务成功或失败。
- 提供打开文件夹按钮。
- 提供复制错误详情按钮。

---

## 16. 颜色与样式建议

### 16.1 主题变量

```css
:root {
  --color-bg-start: #f7fbff;
  --color-bg-end: #eef3ff;
  --color-card: rgba(255, 255, 255, 0.82);
  --color-card-border: rgba(130, 150, 180, 0.18);
  --color-text-main: #172033;
  --color-text-muted: #6b7280;
  --color-primary: #6c8cff;
  --color-primary-hover: #5c7df2;
  --color-success: #20b486;
  --color-warning: #f5a524;
  --color-danger: #f05252;
  --radius-card: 22px;
  --radius-button: 14px;
  --shadow-card: 0 18px 50px rgba(80, 100, 140, 0.14);
}
```

### 16.2 背景

可使用柔和渐变：

```css
.app-bg {
  background:
    radial-gradient(circle at 12% 18%, rgba(108, 140, 255, 0.18), transparent 28%),
    radial-gradient(circle at 88% 12%, rgba(32, 180, 134, 0.14), transparent 24%),
    linear-gradient(135deg, var(--color-bg-start), var(--color-bg-end));
}
```

### 16.3 卡片

```css
.card {
  background: var(--color-card);
  border: 1px solid var(--color-card-border);
  border-radius: var(--radius-card);
  box-shadow: var(--shadow-card);
  backdrop-filter: blur(16px);
}
```

### 16.4 按钮

主按钮：

```css
.primary-button {
  background: linear-gradient(135deg, #6c8cff, #7c6cff);
  color: white;
  border-radius: var(--radius-button);
  box-shadow: 0 10px 24px rgba(108, 140, 255, 0.28);
}
```

次按钮：

```css
.secondary-button {
  background: rgba(255, 255, 255, 0.78);
  color: var(--color-text-main);
  border: 1px solid rgba(130, 150, 180, 0.22);
}
```

警告按钮不使用大面积红色，除非是明确删除确认。

---

## 17. 错误类型设计

Rust 后端统一错误类型：

```rust
pub enum AppError {
    InvalidRootDir(String),
    DangerousRootDir(String),
    PermissionDenied(String),
    VolumeNotFound,
    MultipleVolumeGroups(Vec<String>),
    MissingFirstVolume,
    MissingVolumeIndexes(Vec<u32>),
    DuplicateVolumeIndexes(Vec<u32>),
    ExtraFilesDetected(Vec<PathBuf>),
    MoveFailed(String),
    DeleteFailed(String),
    ExtractToolNotFound,
    ExtractFailed(String),
    PasswordFailed,
    TxtNotFound,
    MultipleTxtFound(Vec<PathBuf>),
    InvalidArchive(String),
    RenameFailed(String),
    Cancelled,
    Unknown(String),
}
```

前端展示时需要转换为用户友好文案。

示例：

```text
技术错误：
ExtractFailed("7zz exit code 2: Data Error")

用户文案：
解压失败：压缩包可能损坏，或密码不正确。
```

---

## 18. 任务主流程伪代码

```rust
async fn run_task(input: TaskInput, app: AppHandle) -> Result<(), AppError> {
    let mut ctx = TaskContext::from_input(input)?;

    validate_root_dir(&ctx.root_dir)?;
    validate_final_folder_name(&ctx.final_folder_name)?;

    emit_progress("scanning", 5, "正在扫描文件夹");
    let scan_result = scan_root_recursively(&ctx.root_dir)?;

    validate_scan_result_or_request_frontend_confirmation(&scan_result)?;

    let volume_group = resolve_volume_group(scan_result.volume_groups)?;
    ctx.selected_volume_group = Some(volume_group.clone());

    emit_progress("moving_volumes", 15, "正在归集分卷文件");
    move_volume_files_to_root(&volume_group, &ctx.root_dir)?;

    emit_progress("cleaning_folders", 25, "正在清理空文件夹");
    remove_empty_source_folders(&ctx.root_dir)?;

    emit_progress("extracting_7z", 35, "正在解压 7z 分卷压缩包");
    let first_volume = find_first_volume_in_root(&ctx.root_dir)?;
    extract_with_passwords(&first_volume, &ctx.root_dir, &ctx.password_list)?;

    emit_progress("deleting_volumes", 45, "正在删除原始分卷文件");
    delete_volume_files(&ctx.root_dir)?;

    for iteration in 1..=ctx.max_iterations {
        ctx.current_iteration = iteration;

        emit_progress("finding_txt", 50, "正在查找 txt 文件");
        let txt_candidates = find_txt_candidates(&ctx.root_dir)?;

        if txt_candidates.is_empty() {
            emit_progress("renaming_root", 90, "正在重命名根文件夹");
            rename_root_folder_if_needed(&ctx.root_dir, &ctx.final_folder_name)?;
            emit_progress("completed", 100, "处理完成");
            return Ok(());
        }

        let txt_file = resolve_txt_candidate(txt_candidates)?;

        emit_progress("renaming_txt_to_rar", 55, "正在将 txt 改名为 rar");
        let rar_file = rename_txt_to_rar(txt_file)?;

        emit_progress("cleaning_except_rar", 60, "正在清理无关文件");
        validate_archive(&rar_file)?;
        clean_root_except(&ctx.root_dir, &rar_file)?;

        emit_progress("extracting_rar", 70, "正在解压 rar 文件");
        extract_with_passwords(&rar_file, &ctx.root_dir, &ctx.password_list)?;

        emit_progress("deleting_rar", 80, "正在删除 rar 文件");
        delete_file(&rar_file)?;
    }

    Err(AppError::Unknown("超过最大嵌套解压次数".to_string()))
}
```

---

## 19. 取消任务设计

用户点击“取消任务”后：

- 前端调用 `cancel_task()`。
- 后端设置取消标记。
- 文件扫描、移动、删除、解压前后均检查取消标记。
- 如果外部解压进程正在运行，应终止该进程。
- 取消后不再继续删除或重命名。
- 弹窗提示当前状态可能是不完整的。
- 日志记录“用户取消任务”。

不建议实现复杂回滚，因为文件移动和解压涉及大量文件，回滚容易引入新的风险。应在界面上明确提示用户任务执行前建议备份。

---

## 20. 跨平台适配

### 20.1 路径处理

必须使用 Rust 的 `Path` 和 `PathBuf`，不要手动拼接路径字符串。

错误示例：

```rust
let path = root + "/" + file_name;
```

正确示例：

```rust
let path = root.join(file_name);
```

### 20.2 文件名大小写

Windows 文件系统通常大小写不敏感，macOS 取决于磁盘格式。程序应避免在同一目录生成仅大小写不同的文件名。

例如：

```text
stage.txt
Stage.txt
```

这种情况应视为冲突。

### 20.3 长路径

Windows 可能存在长路径限制。程序应：

- 尽量减少临时目录层级。
- 临时目录命名简短。
- 捕获路径过长错误。
- 提示用户将文件夹移动到更短路径下重试。

### 20.4 文件占用

Windows 上文件被占用时删除或重命名可能失败。

应提示：

```text
文件可能正在被其他程序占用，请关闭相关窗口或软件后重试。
```

### 20.5 macOS 权限

macOS 可能需要用户授权访问：

- 下载目录。
- 桌面。
- 文稿。
- 外接磁盘。

如遇权限错误，应提示用户：

```text
请在系统设置中允许本应用访问该文件夹，或将文件移动到普通用户目录后重试。
```

---

## 21. Sidecar 打包设计

### 21.1 二进制文件命名

Tauri sidecar 需要针对不同平台准备不同二进制。

推荐命名：

```text
src-tauri/binaries/7zz-x86_64-pc-windows-msvc.exe
src-tauri/binaries/7zz-x86_64-apple-darwin
src-tauri/binaries/7zz-aarch64-apple-darwin
```

### 21.2 tauri.conf.json 配置

示例：

```json
{
  "bundle": {
    "externalBin": [
      "binaries/7zz"
    ]
  }
}
```

实际命名需根据 Tauri sidecar 规则调整，确保不同 target triple 下能正确解析。

### 21.3 启动时校验

程序启动时应检查：

- 当前平台。
- 当前架构。
- 对应 7zz 是否存在。
- macOS 下 7zz 是否有执行权限。
- 7zz 是否能正常运行。

执行：

```bash
7zz
```

或：

```bash
7zz --help
```

如果失败，界面提示：

```text
内置解压组件不可用，请重新安装本程序。
```

### 21.4 macOS 执行权限

打包前需要确保：

```bash
chmod +x src-tauri/binaries/7zz-aarch64-apple-darwin
chmod +x src-tauri/binaries/7zz-x86_64-apple-darwin
```

### 21.5 许可证文件

需要在应用中附带：

```text
docs/THIRD_PARTY_NOTICES.md
```

内容包括：

- 7-Zip 名称。
- 7-Zip 官方来源。
- 7-Zip 许可证说明。
- unRAR 限制说明。
- 本工具仅调用 7-Zip 进行解压，不提供 RAR 压缩能力。

---

## 22. 数据流设计

```text
用户选择文件夹
        │
        ▼
前端 FolderPicker
        │ invoke preview_task
        ▼
Rust 后端扫描目录
        │
        ├─ 返回分卷组
        ├─ 返回额外文件
        ├─ 返回警告
        ▼
前端展示预检查结果
        │
        ├─ 用户取消
        │
        └─ 用户确认继续
                │ invoke start_task
                ▼
        Rust 后端执行任务
                │
                ├─ emit progress
                ├─ emit log
                ├─ 必要时请求用户选择
                ▼
        前端更新进度、日志、弹窗
                │
                ▼
        任务完成 / 失败 / 取消
```

---

## 23. 关键边界情况

### 23.1 初始文件夹没有分卷文件

处理方式：

- 阻止开始。
- 提示：
  ```text
  未找到 7z 分卷文件，请确认选择的文件夹是否正确。
  ```

### 23.2 分卷文件分散在多个子文件夹

处理方式：

- 扫描全部子目录。
- 识别同一分卷组。
- 移动到根目录。
- 移动后删除空子文件夹。

### 23.3 根目录已有同名分卷文件

处理方式：

- 阻止自动覆盖。
- 弹窗提示冲突。
- 用户需要手动处理后重试。

### 23.4 解压后找不到 `.txt`

处理方式：

- 如果这是 RAR 循环过程中的正常结束，进入根目录重命名。
- 如果这是第一次 7z 解压后就找不到 `.txt`，仍可认为流程结束，但应给出轻提示：
  ```text
  未找到需要继续转换的 txt 文件，已保留当前解压结果。
  ```

### 23.5 找到多个 `.txt`

处理方式：

- 弹窗让用户选择。
- 如果用户取消，则停止任务。
- 不自动按文件名或时间猜测。

### 23.6 `.txt` 改名后不是有效 RAR

处理方式：

- 停止任务。
- 保留该文件。
- 不删除其他文件。
- 提示用户文件可能并非压缩包。

### 23.7 密码全部失败

处理方式：

- 停止任务。
- 保留现场。
- 提示用户更新密码表。
- 日志中不记录明文密码。

### 23.8 解压过程中磁盘空间不足

处理方式：

- 捕获解压失败。
- 提示磁盘空间不足。
- 保留原始压缩包。
- 清理临时目录。

### 23.9 用户选择外接磁盘或网络磁盘

处理方式：

- 允许。
- 但日志提示：
  ```text
  当前目录位于外接磁盘或网络路径，处理速度可能较慢。
  ```
- 如权限不足，提示用户更换目录或授权。

---

## 24. 测试用例设计

### 24.1 正常流程

```text
root/
├─ sub1/
│  ├─ Archive.7z.001
│  └─ Archive.7z.002
└─ sub2/
   └─ Archive.7z.003
```

预期：

- 三个分卷移动到 root。
- sub1、sub2 被删除。
- Archive.7z.001 解压成功。
- 分卷文件删除。
- txt 改 rar。
- rar 解压。
- rar 删除。
- 根目录重命名成功。

### 24.2 初始存在额外文件

```text
root/
├─ Archive.7z.001
├─ Archive.7z.002
└─ readme.md
```

预期：

- 弹窗警告。
- 用户取消则任务停止。
- 用户继续则执行后续流程。

### 24.3 分卷缺失

```text
root/
├─ Archive.7z.001
└─ Archive.7z.003
```

预期：

- 检测缺失 `002`。
- 阻止任务。
- 提示用户补充分卷。

### 24.4 多组分卷

```text
root/
├─ A.7z.001
├─ A.7z.002
├─ B.7z.001
└─ B.7z.002
```

预期：

- 弹窗选择处理 A 或 B。
- 不自动同时处理。

### 24.5 多个 txt 文件

```text
root/
├─ a.txt
└─ b.txt
```

预期：

- 弹窗让用户选择。
- 不自动选择。

### 24.6 密码错误

预期：

- 依次尝试密码表。
- 全部失败后停止。
- 不删除原压缩包。
- 日志不输出明文密码。

### 24.7 最终文件夹名冲突

```text
parent/
├─ root/
└─ final_name/
```

用户指定最终文件夹名为 `final_name`。

预期：

- 任务开始前或最终重命名前提示冲突。
- 不自动覆盖。

---

## 25. 交互文案

### 25.1 顶部说明

```text
自动整理 7z 分卷压缩包，并连续处理伪装为 txt 的 rar 文件。
```

### 25.2 文件夹选择提示

```text
请选择包含 7z 分卷文件的文件夹。分卷文件可以位于子文件夹中。
```

### 25.3 最终文件夹名提示

```text
留空则使用当前所选文件夹名称。
```

### 25.4 开始前提示

```text
处理过程中会移动、删除和重命名文件。建议提前备份原始文件夹。
```

### 25.5 完成提示

```text
处理完成，文件夹已整理完毕。
```

### 25.6 失败提示

```text
处理失败，已停止后续操作。请查看日志了解原因。
```

---

## 26. 非功能要求

### 26.1 性能

- 扫描目录时应避免阻塞前端。
- 大文件解压时前端仍应保持响应。
- 日志事件应节流，避免大量输出卡顿。
- 文件移动优先使用 rename；跨磁盘时使用 copy + 校验 + delete。

### 26.2 稳定性

- 每个关键步骤前后都要校验文件是否存在。
- 外部命令调用必须设置工作目录。
- 解压失败不能继续清理。
- 删除失败不能继续重命名。
- 异常必须进入统一错误处理。

### 26.3 可维护性

- 文件扫描、解压、清理、重命名分别独立模块。
- 不在 UI 中写业务规则。
- 不在业务模块中写 UI 文案。
- 错误类型集中管理。
- 日志事件结构统一。

### 26.4 隐私

- 程序不联网。
- 不上传文件名。
- 不上传密码。
- 密码表只保存在本地。
- 日志不包含明文密码。

---

## 27. 可直接执行的核心实现要点

### 27.1 扫描函数

输入：

```rust
root_dir: PathBuf
```

输出：

```rust
ScanResult {
    volume_groups: Vec<VolumeGroup>,
    extra_files: Vec<PathBuf>,
    extra_dirs: Vec<PathBuf>,
    warnings: Vec<WarningItem>,
}
```

扫描规则：

- 递归遍历。
- 跳过符号链接。
- 记录所有文件。
- 使用正则识别分卷。
- 其他文件记为 extra_files。
- 文件夹是否为空单独记录。

### 27.2 移动分卷函数

输入：

```rust
volume_group: VolumeGroup
root_dir: PathBuf
```

行为：

- 将所有分卷移动到根目录。
- 如果目标存在同名文件，报错。
- 移动后更新 VolumeGroup 中的路径。
- 跨磁盘移动时复制并校验文件大小。

### 27.3 解压函数

输入：

```rust
archive_path: PathBuf
output_dir: PathBuf
passwords: Vec<String>
```

行为：

- 创建临时输出目录。
- 逐个密码尝试。
- 成功后将临时目录内容移动到 output_dir。
- 失败后删除临时目录。
- 返回成功密码索引，不返回密码明文。

### 27.4 清理函数

输入：

```rust
root_dir: PathBuf
keep_paths: Vec<PathBuf>
```

行为：

- 遍历 root_dir 当前层级。
- 不删除 keep_paths。
- 不删除日志路径。
- 不跟随符号链接。
- 删除前校验路径在 root_dir 内。
- 删除失败立刻停止。

### 27.5 根目录重命名函数

输入：

```rust
root_dir: PathBuf
final_folder_name: String
```

行为：

- 如果名称相同，跳过。
- 校验名称合法。
- 校验父目录下无同名文件夹。
- 执行重命名。
- 返回新路径。

---

## 28. 不建议采用的方案

### 28.1 不建议 PySide / PyQt

原因：

- 默认界面风格较传统。
- 实现现代化卡片界面成本较高。
- 打包体积和跨平台细节不一定更轻。
- 本项目更适合 Web UI + 原生后端。

### 28.2 不建议 Electron

原因：

- 对该小工具而言运行时偏重。
- 安装包体积较大。
- 文件操作仍需 Node 后端处理。
- Tauri 更适合轻量桌面工具。

### 28.3 不建议要求用户安装 WinRAR / 7-Zip

原因：

- 增加用户使用门槛。
- 不利于跨平台一致性。
- 用户环境不可控。
- 本工具应提供开箱即用体验。

### 28.4 不建议直接用前端处理文件

原因：

- 文件系统权限复杂。
- 删除和移动操作风险高。
- 解压命令调用需要后端管理。
- 错误处理和日志不容易统一。

---

## 29. 任务完成判定

任务完成必须同时满足：

- 没有待处理的 `.txt` 候选文件。
- 当前无 `.rar` 临时文件。
- 原始分卷文件已删除。
- 临时目录已清理。
- 根目录已按用户指定名称处理。
- 日志中记录完成状态。
- 前端收到 `completed` 状态事件。

完成提示：

```text
处理完成
```

失败提示：

```text
处理失败，已停止后续操作。
```

取消提示：

```text
任务已取消，当前文件夹可能处于中间状态，请查看日志。
```

---

## 30. 第三方依赖说明

### 30.1 7-Zip

本工具使用 7-Zip 命令行程序作为内置解压组件。

用途：

- 解压 `7z` 分卷压缩包。
- 解压 `.rar` 文件。
- 测试压缩包是否可打开。
- 读取压缩包列表。

注意：

- 仅使用解压能力。
- 不提供 RAR 压缩能力。
- 应保留 7-Zip 许可证文件。
- 应在 `THIRD_PARTY_NOTICES.md` 中说明 unRAR 限制。

### 30.2 前端依赖

建议依赖：

```json
{
  "dependencies": {
    "@tauri-apps/api": "latest",
    "vue": "latest",
    "@vueuse/core": "latest"
  },
  "devDependencies": {
    "@tauri-apps/cli": "latest",
    "@vitejs/plugin-vue": "latest",
    "typescript": "latest",
    "vite": "latest",
    "unocss": "latest"
  }
}
```

### 30.3 Rust 依赖

建议依赖：

```toml
[dependencies]
tauri = { version = "2", features = [] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
tokio = { version = "1", features = ["process", "fs", "rt-multi-thread"] }
walkdir = "2"
regex = "1"
chrono = "0.4"
trash = "5"
```

说明：

- `walkdir` 用于递归扫描。
- `regex` 用于分卷文件识别。
- `tokio::process` 用于异步调用 7zz。
- `trash` 用于移动到回收站 / 废纸篓。
- `thiserror` 用于统一错误类型。
- `serde` 用于前后端数据结构序列化。

---

## 31. 开发时需要特别注意的点

1. 所有删除操作必须在后端进行路径安全校验。
2. 解压失败时不能删除原始压缩包。
3. 密码不能明文写入日志。
4. 多个 `.txt` 文件不能自动选择。
5. 多组分卷不能自动全部处理。
6. 分卷缺失不能继续解压。
7. `.txt` 改 `.rar` 后必须先校验压缩包有效性，再清理其他文件。
8. 根目录重命名前必须检查同名冲突。
9. macOS sidecar 必须有执行权限。
10. Windows 下需要处理文件占用和长路径问题。
11. 任务取消时需要终止正在运行的解压进程。
12. 程序不应联网。
13. 用户未填写最终文件夹名时，不应强制重命名。
14. 初始发现额外文件时必须弹窗确认。
15. 日志和进度要实时反馈，避免用户误以为程序卡死。

---

## 32. 推荐 README 简述

```text
sjs-unzip-tool 是一个用于自动整理 7z 分卷压缩包并连续解包伪装 RAR 文件的桌面工具。

用户只需选择包含分卷文件的文件夹，程序会自动归集分卷、解压、识别 txt 伪装压缩包、改名为 rar、继续解压，并在完成后将文件夹重命名为指定名称。

本工具内置解压组件，不需要用户安装 7-Zip、WinRAR 或其他命令行工具。
```

---

## 33. 最终设计结论

本项目推荐使用：

```text
Tauri 2 + Vue 3 + Vite + TypeScript + Rust + 内置 7-Zip sidecar
```

前端负责：

- 文件夹选择。
- 最终文件夹名输入。
- 密码表维护。
- 进度展示。
- 日志展示。
- 弹窗交互。
- 任务结果展示。

后端负责：

- 目录扫描。
- 分卷识别。
- 文件移动。
- 文件删除。
- 解压调用。
- 密码尝试。
- `.txt` 改 `.rar`。
- 根目录重命名。
- 安全校验。
- 日志和进度事件发送。

该设计能够满足跨平台、轻量、现代化、美观、无需用户额外安装解压工具、支持密码表、支持风险提示和任务日志的需求。
