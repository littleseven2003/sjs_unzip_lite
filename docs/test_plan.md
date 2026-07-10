# 测试方案文档

## 1. 测试概述

本文档描述 sjs-unzip-tool 的功能测试方案，包括测试用例、测试方法和预期结果。

### 1.1 测试环境

- 操作系统：macOS (Apple Silicon)
- 测试数据位置：`/tmp/test_unzip/`
- 应用启动方式：`pnpm tauri dev`

### 1.2 测试工具

- 7zz 命令行工具（用于创建测试数据）
- 应用内置日志面板
- 系统终端（查看调试输出）

---

## 2. 测试用例

### 2.1 测试1：基本分卷解压

#### 测试目的

验证基本的分卷压缩包解压功能，包括：
- 分卷文件识别
- 额外文件检测和清理
- 解压成功

#### 测试数据

**路径**：`/tmp/test_unzip/test1`

**数据内容**：
```
test1/
├─ archive.7z.001 (512KB)  ← 分卷1
├─ archive.7z.002 (512KB)  ← 分卷2
├─ archive.7z.003 (512KB)  ← 分卷3
├─ archive.7z.004 (285B)   ← 分卷4
├─ file1.bin (512KB)       ← 额外文件
├─ file2.bin (512KB)       ← 额外文件
└─ file3.bin (512KB)       ← 额外文件
```

**压缩包内容**：file1.bin, file2.bin, file3.bin

**创建命令**：
```bash
cd /tmp/test_unzip/test1
dd if=/dev/urandom of=file1.bin bs=1024 count=500
dd if=/dev/urandom of=file2.bin bs=1024 count=500
dd if=/dev/urandom of=file3.bin bs=1024 count=500
7zz a -v500k archive.7z file1.bin file2.bin file3.bin
```

#### 测试步骤

1. 点击"选择文件夹" → 选择 `/tmp/test_unzip/test1`
2. 点击"预检查"
3. 确认显示警告"检测到额外文件"
4. 点击"继续处理"
5. 确认显示分卷组信息（4个分卷）
6. 点击"开始处理"
7. 等待任务完成

#### 预期结果

| 检查项 | 预期结果 |
|--------|----------|
| 预检查警告 | 显示"检测到额外文件"警告 |
| 分卷识别 | 识别1个分卷组，4个分卷文件 |
| 额外文件清理 | file1.bin, file2.bin, file3.bin 在解压前被清理 |
| 解压成功 | 日志显示"解压成功" |
| 分卷删除 | archive.7z.001~004 被删除 |
| 最终状态 | 目录中包含解压出的 file1.bin, file2.bin, file3.bin |

#### 恢复命令

```bash
cd /tmp/test_unzip && rm -rf test1 && mkdir -p test1 && cd test1
dd if=/dev/urandom of=file1.bin bs=1024 count=500
dd if=/dev/urandom of=file2.bin bs=1024 count=500
dd if=/dev/urandom of=file3.bin bs=1024 count=500
7zz a -v500k archive.7z file1.bin file2.bin file3.bin
```

---

### 2.2 测试2：分卷分散在子目录

#### 测试目的

验证分卷文件分散在多个子目录时的处理功能，包括：
- 跨目录分卷识别
- 分卷归集到根目录
- 空子目录清理

#### 测试数据

**路径**：`/tmp/test_unzip/test2`

**数据内容**：
```
test2/
├─ archive.7z.005 (268B)        ← 根目录分卷
├─ large_file.bin (2MB)          ← 额外文件
├─ sub1/
│  ├─ archive.7z.001 (512KB)    ← 子目录分卷
│  └─ archive.7z.002 (512KB)    ← 子目录分卷
└─ sub2/
   ├─ archive.7z.003 (512KB)    ← 子目录分卷
   └─ archive.7z.004 (512KB)    ← 子目录分卷
```

**压缩包内容**：large_file.bin

**创建命令**：
```bash
cd /tmp/test_unzip && mkdir -p test2/sub1 test2/sub2 && cd test2
dd if=/dev/urandom of=large_file.bin bs=1024 count=2000
7zz a -v500k archive.7z large_file.bin
mv archive.7z.001 sub1/
mv archive.7z.002 sub1/
mv archive.7z.003 sub2/
mv archive.7z.004 sub2/
```

#### 测试步骤

1. 点击"选择文件夹" → 选择 `/tmp/test_unzip/test2`
2. 点击"预检查"
3. 确认显示警告"检测到额外文件"
4. 点击"继续处理"
5. 确认显示分卷组信息（5个分卷）
6. 点击"开始处理"
7. 等待任务完成

#### 预期结果

| 检查项 | 预期结果 |
|--------|----------|
| 分卷识别 | 识别1个分卷组，5个分卷文件 |
| 分卷归集 | sub1/sub2 中的分卷移动到根目录 |
| 子目录清理 | sub1/ 和 sub2/ 被删除 |
| 额外文件清理 | large_file.bin 被清理 |
| 解压成功 | 日志显示"解压成功" |
| 最终状态 | 目录中包含解压出的 large_file.bin |

#### 恢复命令

```bash
cd /tmp/test_unzip && rm -rf test2 && mkdir -p test2/sub1 test2/sub2 && cd test2
dd if=/dev/urandom of=large_file.bin bs=1024 count=2000
7zz a -v500k archive.7z large_file.bin
mv archive.7z.001 sub1/
mv archive.7z.002 sub1/
mv archive.7z.003 sub2/
mv archive.7z.004 sub2/
```

---

### 2.3 测试3：密码保护

#### 测试目的

验证密码保护压缩包的解压功能，包括：
- 密码表管理
- 密码自动尝试
- 加密分卷解压

#### 测试数据

**路径**：`/tmp/test_unzip/test3`

**数据内容**：
```
test3/
├─ archive.7z.001 (200KB)  ← 分卷1
├─ archive.7z.002 (200KB)  ← 分卷2
└─ archive.7z.003 (100KB)  ← 分卷3
```

**压缩包内容**：secret.txt (27B), data.bin (500KB)

**密码**：`mypassword`

**创建命令**：
```bash
cd /tmp/test_unzip && mkdir -p test3 && cd test3
echo "Secret content for testing" > secret.txt
dd if=/dev/urandom of=data.bin bs=1024 count=500
7zz a -v200k -p"mypassword" archive.7z secret.txt data.bin
rm secret.txt data.bin
```

#### 测试步骤

1. 点击"管理密码表"
2. 确认密码表中有 `mypassword`（如果没有则添加）
3. 点击"保存"关闭密码管理
4. 点击"选择文件夹" → 选择 `/tmp/test_unzip/test3`
5. 点击"预检查"
6. 确认显示分卷组信息（3个分卷，无警告）
7. 点击"开始处理"
8. 等待任务完成

#### 预期结果

| 检查项 | 预期结果 |
|--------|----------|
| 密码尝试 | 依次尝试空密码、其他密码、mypassword |
| 解压成功 | 日志显示"解压成功，使用的密码序号：2" |
| 最终状态 | 目录中包含解压出的 secret.txt 和 data.bin |

#### 恢复命令

```bash
cd /tmp/test_unzip && rm -rf test3 && mkdir -p test3 && cd test3
echo "Secret content for testing" > secret.txt
dd if=/dev/urandom of=data.bin bs=1024 count=500
7zz a -v200k -p"mypassword" archive.7z secret.txt data.bin
rm secret.txt data.bin
```

---

### 2.4 测试4：伪装txt文件（普通文本）

#### 测试目的

验证当解压后的 txt 文件不是伪装压缩包时的处理：
- 识别 txt 文件
- 校验是否为有效压缩包
- 非压缩包时停止处理

#### 测试数据

**路径**：`/tmp/test_unzip/test4`

**数据内容**：
```
test4/
├─ archive.7z.001 (100KB)  ← 分卷1
└─ archive.7z.002 (40KB)   ← 分卷2
```

**压缩包内容**：inner.txt（普通文本文件，非伪装压缩包）

**创建命令**：
```bash
cd /tmp/test_unzip && mkdir -p test4 && cd test4
echo "This is a normal text file, not a disguised archive" > inner.txt
dd if=/dev/urandom of=padding.bin bs=1024 count=100
7zz a -v100k archive.7z inner.txt padding.bin
rm inner.txt padding.bin
```

#### 测试步骤

1. 点击"选择文件夹" → 选择 `/tmp/test_unzip/test4`
2. 点击"预检查"
3. 确认显示分卷组信息
4. 点击"开始处理"
5. 等待任务完成

#### 预期结果

| 检查项 | 预期结果 |
|--------|----------|
| 解压成功 | 7z 解压成功 |
| txt 识别 | 找到 inner.txt |
| 压缩包校验 | 校验发现不是有效压缩包 |
| 处理停止 | 停止处理，保留文件 |
| 最终状态 | 目录中包含 inner.txt |

#### 恢复命令

```bash
cd /tmp/test_unzip && rm -rf test4 && mkdir -p test4 && cd test4
echo "This is a normal text file, not a disguised archive" > inner.txt
dd if=/dev/urandom of=padding.bin bs=1024 count=100
7zz a -v100k archive.7z inner.txt padding.bin
rm inner.txt padding.bin
```

---

### 2.5 测试5：伪装txt文件（有效压缩包）

#### 测试目的

验证当解压后的 txt 文件是伪装压缩包时的完整流程：
- 识别 txt 文件
- 校验为有效压缩包
- 改名 rar 并继续解压

#### 测试数据

**路径**：`/tmp/test_unzip/test5`

**数据内容**：
```
test5/
├─ archive.7z.001 (100KB)  ← 分卷1
└─ archive.7z.002 (40KB)   ← 分卷2
```

**压缩包内容**：
- inner.txt（伪装的 7z 压缩包）
- padding.bin（填充文件）

**inner.txt 实际内容**：7z 压缩包，包含 final.txt

**创建命令**：
```bash
cd /tmp/test_unzip && mkdir -p test5 && cd test5
echo "Final content after all extractions" > final.txt
7zz a inner.7z final.txt
mv inner.7z inner.txt
dd if=/dev/urandom of=padding.bin bs=1024 count=100
7zz a -v100k archive.7z inner.txt padding.bin
rm final.txt padding.bin
```

#### 测试步骤

1. 点击"选择文件夹" → 选择 `/tmp/test_unzip/test5`
2. 点击"预检查"
3. 确认显示分卷组信息
4. 点击"开始处理"
5. 等待任务完成

#### 预期结果

| 检查项 | 预期结果 |
|--------|----------|
| 7z 解压成功 | 第一层解压成功 |
| txt 识别 | 找到 inner.txt |
| 压缩包校验 | 校验通过（是有效压缩包） |
| 改名 rar | inner.txt 改名为 inner.rar |
| rar 解压成功 | 第二层解压成功 |
| 最终状态 | 目录中包含解压出的 final.txt |

#### 恢复命令

```bash
cd /tmp/test_unzip && rm -rf test5 && mkdir -p test5 && cd test5
echo "Final content after all extractions" > final.txt
7zz a inner.7z final.txt
mv inner.7z inner.txt
dd if=/dev/urandom of=padding.bin bs=1024 count=100
7zz a -v100k archive.7z inner.txt padding.bin
rm final.txt padding.bin
```

---

## 3. 测试流程

### 3.1 测试前准备

1. 确认应用已启动（`pnpm tauri dev`）
2. 确认测试数据已恢复到初始状态
3. 确认密码表配置正确（测试3需要）

### 3.2 测试执行

1. 在应用窗口中执行测试步骤
2. 观察日志面板输出
3. 观察终端调试输出（如需要）
4. 记录测试结果

### 3.3 测试后检查

1. 检查目标目录内容是否符合预期
2. 检查日志文件内容
3. 记录任何异常情况

---

## 4. 测试恢复

每个测试用例执行后，需要恢复到初始状态才能重新测试。

### 4.1 批量恢复命令

```bash
# 测试1
cd /tmp/test_unzip && rm -rf test1 && mkdir -p test1 && cd test1
dd if=/dev/urandom of=file1.bin bs=1024 count=500
dd if=/dev/urandom of=file2.bin bs=1024 count=500
dd if=/dev/urandom of=file3.bin bs=1024 count=500
7zz a -v500k archive.7z file1.bin file2.bin file3.bin

# 测试2
cd /tmp/test_unzip && rm -rf test2 && mkdir -p test2/sub1 test2/sub2 && cd test2
dd if=/dev/urandom of=large_file.bin bs=1024 count=2000
7zz a -v500k archive.7z large_file.bin
mv archive.7z.001 sub1/ && mv archive.7z.002 sub1/
mv archive.7z.003 sub2/ && mv archive.7z.004 sub2/

# 测试3
cd /tmp/test_unzip && rm -rf test3 && mkdir -p test3 && cd test3
echo "Secret content for testing" > secret.txt
dd if=/dev/urandom of=data.bin bs=1024 count=500
7zz a -v200k -p"mypassword" archive.7z secret.txt data.bin
rm secret.txt data.bin

# 测试4
cd /tmp/test_unzip && rm -rf test4 && mkdir -p test4 && cd test4
echo "This is a normal text file, not a disguised archive" > inner.txt
dd if=/dev/urandom of=padding.bin bs=1024 count=100
7zz a -v100k archive.7z inner.txt padding.bin
rm inner.txt padding.bin

# 测试5
cd /tmp/test_unzip && rm -rf test5 && mkdir -p test5 && cd test5
echo "Final content after all extractions" > final.txt
7zz a inner.7z final.txt
mv inner.7z inner.txt
dd if=/dev/urandom of=padding.bin bs=1024 count=100
7zz a -v100k archive.7z inner.txt padding.bin
rm final.txt padding.bin
```

---

## 5. 测试检查清单

| 测试 | 用例 | 关键检查点 | 状态 |
|------|------|------------|------|
| 测试1 | 基本分卷解压 | 额外文件清理、解压成功 | ☐ |
| 测试2 | 分卷分散在子目录 | 分卷归集、子目录清理 | ☐ |
| 测试3 | 密码保护 | 密码尝试、加密解压 | ☐ |
| 测试4 | 伪装txt（普通文本） | 压缩包校验、停止处理 | ☐ |
| 测试5 | 伪装txt（有效压缩包） | txt→rar转换、嵌套解压 | ☐ |

---

## 6. 问题记录

| 日期 | 测试 | 问题描述 | 状态 |
|------|------|----------|------|
| | | | |
