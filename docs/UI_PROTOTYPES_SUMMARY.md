# SenseVoice Desktop App - UI 界面原型设计总结

## 概述

本文档总结了 5 种不同风格的 SenseVoice 桌面应用界面原型设计。每种设计都实现了完整的功能需求和设置界面，但采用了不同的视觉风格和交互模式。

## 5 种界面风格对比

### Style 1: Glassmorphism（玻璃拟态）

**文件**: `ui-prototypes/style1-glassmorphism.html`

**视觉特点**:
- 现代玻璃效果（`backdrop-filter: blur()`）
- 渐变背景和半透明卡片
- 柔和的阴影和圆角
- 轻盈、通透的视觉感受

**交互设计**:
- 悬浮模态对话框
- 平滑动画过渡
- 视觉层次清晰

**适用场景**: 现代化、时尚的桌面应用，追求视觉美感的用户

**优点**:
- ✅ 视觉冲击力强
- ✅ 现代、时尚
- ✅ 界面层次分明

**缺点**:
- ⚠️ 性能开销较大（blur 效果）
- ⚠️ 低端设备可能卡顿
- ⚠️ 文字对比度需要仔细调优

---

### Style 2: Minimalist（极简主义）

**文件**: `ui-prototypes/style2-minimalist.html`

**视觉特点**:
- 极简设计，强调排版
- 细线条边框
- 大量留白
- 黑白灰配色为主

**交互设计**:
- 简洁的表单布局
- 最小化视觉干扰
- 清晰的信息层级

**适用场景**: 专注工作流程，追求效率的专业用户

**优点**:
- ✅ 性能优秀
- ✅ 专注内容
- ✅ 易于维护和扩展
- ✅ 适合长时间使用

**缺点**:
- ⚠️ 视觉吸引力较低
- ⚠️ 缺乏品牌个性

---

### Style 3: Professional Dark（专业深色）

**文件**: `ui-prototypes/style3-professional-dark.html`

**视觉特点**:
- 深色主题
- 侧边栏导航
- 表格式文件列表
- 专业工作区美学

**交互设计**:
- 多标签设置面板
- 侧边栏导航
- 状态徽章和进度条

**适用场景**: 专业编辑、长时间工作、暗光环境

**优点**:
- ✅ 护眼，适合长时间使用
- ✅ 专业感强
- ✅ 信息密度高
- ✅ 适合多任务管理

**缺点**:
- ⚠️ 深色主题不适合所有场景
- ⚠️ 需要提供浅色主题切换

---

### Style 4: Futuristic（未来科幻）

**文件**: `ui-prototypes/style4-futuristic.html`

**视觉特点**:
- 赛博朋克/科幻风格
- 霓虹绿 (#00ff88) 主色调
- 动画网格背景
- 六边形图标和多边形裁切
- 发光效果和动画

**交互设计**:
- 大写文字，宽字间距
- 动态视觉效果
- 沉浸式界面

**适用场景**: 游戏、创意工作、娱乐应用

**优点**:
- ✅ 极强的视觉冲击力
- ✅ 独特的品牌识别度
- ✅ 科技感强

**缺点**:
- ⚠️ 性能开销大（动画、发光效果）
- ⚠️ 视觉疲劳
- ⚠️ 不适合严肃商业场景
- ⚠️ 可读性可能受影响

---

### Style 5: Material Design 3（材料设计 3）

**文件**: `ui-prototypes/style5-material.html`

**视觉特点**:
- Google Material Design 3 规范
- Material You 配色系统
- 层级阴影（elevation）
- 圆角和表面设计
- 渐变背景

**交互设计**:
- Navigation Rail（导航栏）
- FAB（悬浮操作按钮）
- 标准 Material 组件
- Ripple 动画效果

**适用场景**: 跨平台应用、Android 生态、现代 Web 应用

**优点**:
- ✅ 成熟的设计系统
- ✅ 组件丰富，易于扩展
- ✅ 跨平台一致性
- ✅ 用户熟悉度高
- ✅ 有详细的设计规范

**缺点**:
- ⚠️ 过于常见，缺乏独特性
- ⚠️ 在 macOS 上可能感觉不原生

---

## 完整设置界面设计

所有 5 种风格都实现了相同的完整设置功能，包括以下模块：

### 1. ⌨️ 快捷键配置

**功能说明**: 全局热键设置，可在任何应用中触发

**配置项**:
- **开始/停止录音**
  - 默认: `Ctrl + Shift + Space`
  - 功能: 控制音频录制
  - 类型: 全局热键

- **语音输入模式**
  - 默认: `Ctrl + Alt + V`
  - 功能: 启动语音输入法，识别结果自动注入光标位置
  - 类型: 全局热键

- **实时字幕**
  - 默认: `Ctrl + Shift + S`
  - 功能: 开启系统音频实时字幕显示
  - 类型: 全局热键

- **暂停/继续**
  - 默认: `Ctrl + Shift + P`
  - 功能: 暂停或继续当前识别任务

**技术实现**:
- 使用全局热键库（如 `global-hotkey` crate）
- 支持自定义组合键
- 热键冲突检测
- 可禁用全局热键功能

---

### 2. 🤖 本地模型配置

**功能说明**: 配置 SenseVoice GGUF 模型文件和推理参数

**配置项**:

- **模型文件路径**
  - 输入: 文件路径选择器
  - 默认: `/models/sensevoice-q4k.gguf`
  - 验证: 检查文件是否存在，格式是否正确

- **默认模型选择**
  - Q3_K: 最小 (85MB) - 快速，质量略低
  - **Q4_K**: 推荐 (128MB) - 平衡速度和质量 ✅
  - Q8_0: 高质量 (235MB) - 质量高，速度慢
  - FP16: 最高质量 (458MB) - 最高质量，速度最慢

- **GPU 加速**
  - 开关: 启用/禁用
  - 支持: CUDA (NVIDIA), Metal (Apple), ROCm (AMD)
  - 自动检测可用 GPU

- **推理线程数**
  - 自动检测 ✅
  - 4 线程
  - 8 线程
  - 16 线程
  - 32 线程
  - 说明: 自动检测会根据 CPU 核心数选择最优值

- **批处理大小**
  - 范围: 1-32
  - 默认: 1
  - 说明: 增加批处理可提高吞吐量，但增加延迟

**技术实现**:
- 模型文件 MD5 校验
- 内存占用预估
- GPU 可用性检测
- 性能基准测试

---

### 3. 🌐 大模型接口配置

**功能说明**: 配置在线 LLM API 用于文本后处理和错别字修正

**配置项**:

- **启用在线文本修正**
  - 开关: 启用/禁用
  - 功能: 使用 LLM API 修正识别错误和错别字

- **LLM 提供商**
  - OpenAI (GPT-4)
  - Anthropic (Claude)
  - 本地 Ollama
  - 其他 OpenAI 兼容 API

- **API 端点**
  - OpenAI: `https://api.openai.com/v1`
  - Claude: `https://api.anthropic.com/v1`
  - Ollama: `http://localhost:11434/v1`
  - 自定义端点

- **API 密钥**
  - 输入: 密码字段
  - 验证: 连接测试
  - 安全: 本地加密存储

- **模型名称**
  - OpenAI: `gpt-4-turbo`, `gpt-3.5-turbo`
  - Claude: `claude-3-opus`, `claude-3-sonnet`
  - Ollama: `qwen:7b`, `llama2:13b`

- **修正提示词模板**
  - 默认: "请修正以下文本中的错别字和语法错误，保持原意不变：\n\n{text}"
  - 可自定义

- **超时设置**
  - 范围: 5-60 秒
  - 默认: 30 秒

- **失败处理**
  - 返回原文
  - 重试 N 次
  - 跳过修正

**技术实现**:
- API 密钥加密存储（系统 keychain）
- 连接测试和健康检查
- 请求队列和重试逻辑
- 成本估算（token 计数）

---

### 4. 📁 媒体文件操作设置

**功能说明**: 配置文件处理流程和音频转换参数

**配置项**:

- **输出目录**
  - 默认: `~/Documents/SenseVoice/Output`
  - 功能: 转录结果保存位置
  - 自动创建不存在的目录

- **文件命名规则**
  - 保持原名: `原文件名.srt`
  - 添加时间戳: `原文件名_20240115_143022.srt`
  - 自定义模板: `{name}_{date}_{time}.{ext}`

- **完成后自动打开文件**
  - 开关: 启用/禁用
  - 功能: 转录完成后自动打开输出文件

- **保留原始音频**
  - 开关: 启用/禁用
  - 功能: 视频文件处理后保留提取的音频

- **音频格式转换**
  - **WAV**: 无损，标准格式 ✅
  - MP3: 压缩，节省空间
  - FLAC: 无损压缩
  - 不转换: 保持原格式

- **音频参数**
  - 采样率: 16000 Hz（SenseVoice 标准）
  - 声道: 单声道（Mono）
  - 比特深度: 16-bit

- **视频音频分离工具**
  - 纯 Rust 实现 (`sensevoice-audio-processor`)
  - 支持格式: MP4, AVI, MKV, MOV, FLV

**技术实现**:
- 使用 `sensevoice-audio-processor` Rust 库
- SIMD 加速音频处理（9x 性能提升）
- 批量处理队列
- 进度回调

---

### 5. 📄 输出格式选择

**功能说明**: 配置转录结果的输出格式和样式

**配置项**:

- **字幕格式**（可多选）
  - **SRT**: SubRip 字幕 ✅ 最通用
  - **ASS**: Advanced SubStation Alpha ✅ 支持样式
  - VTT: WebVTT（Web 字幕）
  - LRC: 歌词格式

- **文本格式**（可多选）
  - **TXT**: 纯文本 ✅
  - Markdown: 标记语言，支持格式化
  - JSON: 结构化数据，包含时间戳和元数据
  - HTML: 网页格式，可直接查看

- **包含时间戳**
  - 开关: 启用/禁用
  - 格式: `[00:12:34] 文本内容`
  - 适用于: TXT, Markdown, JSON

- **包含说话人分离**（实验性）
  - 开关: 启用/禁用
  - 格式: `[Speaker 1] 文本内容`
  - 技术: 基于声纹的简单分离

- **字幕样式设置**
  - 默认样式: 标准白色字幕
  - 电影字幕: 黄色，底部居中
  - 演讲字幕: 白色，顶部居中
  - 双语字幕: 上下双行
  - 自定义样式: 字体、颜色、大小、位置

- **SRT 字幕参数**
  - 每行最大字符数: 42（默认）
  - 单条字幕最大时长: 7 秒
  - 最小显示时间: 1 秒

- **ASS 字幕高级设置**
  - 字体: Arial, 微软雅黑, Noto Sans
  - 字号: 24-72
  - 颜色: RGB/HEX 选择器
  - 描边宽度: 0-5
  - 阴影深度: 0-5
  - 位置: 顶部/底部/自定义

**技术实现**:
- 字幕格式转换库
- 时间轴优化（避免重叠）
- 文本换行算法
- 样式模板系统

---

## 通用功能模块

所有界面原型都包含以下通用功能模块：

### 文件转录模块

- **拖拽上传**: 支持拖拽文件到指定区域
- **批量处理**: 多文件队列管理
- **进度显示**: 实时显示转录进度
- **格式支持**: MP3, WAV, MP4, AVI, MKV, FLAC, AAC, OGG, M4A

### 实时字幕模块

- **系统音频捕获**:
  - Windows: WASAPI Loopback
  - macOS: BlackHole / ScreenCaptureKit
  - Linux: PulseAudio Monitor
- **实时转录**: 低延迟流式识别
- **字幕悬浮窗**: 可拖拽、调整大小的字幕窗口

### 语音输入法模块

- **文本注入**: 自动插入到光标位置
- **应用兼容**: 支持主流应用（Word, VSCode, 浏览器等）
- **快捷切换**: 热键启动/停止

### 历史记录模块

- **任务列表**: 显示所有转录历史
- **快速搜索**: 按文件名、日期、内容搜索
- **批量导出**: 导出多个结果

---

## 技术架构建议

基于 5 种原型设计，推荐使用 **gpui** (Rust 原生 GUI) 实现最终应用：

### 为什么选择 gpui？

1. **纯 Rust**: 与 `sensevoice-audio-processor` 无缝集成
2. **高性能**: GPU 加速渲染
3. **跨平台**: Windows, macOS, Linux
4. **现代化**: 支持现代 UI 范式
5. **类型安全**: 编译期错误检查

### 架构设计

```
┌─────────────────────────────────────────┐
│          gpui UI Layer                  │
│  ┌─────────────────────────────────┐    │
│  │  Main Window                    │    │
│  │  ├─ Navigation Bar              │    │
│  │  ├─ File Transcription View     │    │
│  │  ├─ Realtime Subtitle View      │    │
│  │  ├─ Voice Input View             │    │
│  │  └─ Settings Modal              │    │
│  └─────────────────────────────────┘    │
└─────────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│       Business Logic Layer              │
│  ├─ Task Queue Manager                  │
│  ├─ Global Hotkey Handler               │
│  ├─ LLM API Client                      │
│  └─ Configuration Manager               │
└─────────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│       Core Processing Layer             │
│  ├─ SenseVoice C++ Bindings             │
│  ├─ Audio Processor (Rust + SIMD)       │
│  ├─ System Audio Capture (cpal)         │
│  └─ Text Injection (platform-specific)  │
└─────────────────────────────────────────┘
```

### 风格实现建议

根据原型设计，推荐实现方案：

| 原型风格 | 实现复杂度 | 性能 | 推荐度 |
|---------|----------|------|--------|
| Style 2 (Minimalist) | ⭐⭐ 低 | ⭐⭐⭐⭐⭐ 优秀 | ⭐⭐⭐⭐⭐ 最推荐 |
| Style 5 (Material) | ⭐⭐⭐ 中 | ⭐⭐⭐⭐ 良好 | ⭐⭐⭐⭐ 推荐 |
| Style 3 (Professional Dark) | ⭐⭐⭐ 中 | ⭐⭐⭐⭐ 良好 | ⭐⭐⭐⭐ 推荐 |
| Style 1 (Glassmorphism) | ⭐⭐⭐⭐ 高 | ⭐⭐⭐ 中等 | ⭐⭐⭐ 可选 |
| Style 4 (Futuristic) | ⭐⭐⭐⭐⭐ 很高 | ⭐⭐ 较差 | ⭐⭐ 不推荐 |

**最终推荐**: **Style 2 (Minimalist)** 或 **Style 5 (Material Design 3)**

**理由**:
1. 实现难度适中
2. 性能优秀
3. 跨平台一致性好
4. 易于维护和扩展
5. 用户体验成熟

---

## 下一步工作

1. ✅ **完成 5 种 HTML 原型设计**
2. ✅ **设计完整的设置界面**
3. ⏳ **规划 gpui 实现方案**
   - 技术栈选型
   - 项目结构设计
   - 组件库规划
4. ⏳ **创建 gpui 项目结构**
   - 初始化项目
   - 配置依赖
   - 搭建基础框架
5. ⏳ **实现核心功能**
   - 文件处理
   - 实时字幕
   - 语音输入法
6. ⏳ **集成 SenseVoice 和音频处理库**
7. ⏳ **测试和优化**

---

## 附录

### 设计规范参考

- **Material Design 3**: https://m3.material.io/
- **Apple Human Interface Guidelines**: https://developer.apple.com/design/human-interface-guidelines/
- **Microsoft Fluent Design**: https://www.microsoft.com/design/fluent/

### 技术栈

- **GUI Framework**: gpui (https://github.com/zed-industries/zed)
- **Audio Processing**: sensevoice-audio-processor (纯 Rust + SIMD)
- **ASR Engine**: SenseVoice.cpp (C++ 绑定)
- **全局热键**: global-hotkey (https://crates.io/crates/global-hotkey)
- **系统托盘**: tray-icon (https://crates.io/crates/tray-icon)
- **文本注入**: enigo (https://crates.io/crates/enigo)

### 文件结构

```
SenseVoice.cpp/
├── ui-prototypes/
│   ├── style1-glassmorphism.html      (玻璃拟态)
│   ├── style2-minimalist.html          (极简主义)
│   ├── style3-professional-dark.html   (专业深色)
│   ├── style4-futuristic.html          (未来科幻)
│   └── style5-material.html            (Material Design 3)
├── docs/
│   ├── UI_PROTOTYPES_SUMMARY.md       (本文档)
│   ├── DESKTOP_APP_DESIGN.md          (桌面应用设计文档)
│   ├── rust-audio-architecture.md     (音频架构文档)
│   └── desktop-app-implementation-guide.md
└── audio-processor/                    (Rust 音频处理库)
```
