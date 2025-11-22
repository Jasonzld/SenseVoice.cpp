# SenseVoice Desktop - 智能语音工具设计方案

## 项目概述

基于 SenseVoice.cpp 构建的全功能桌面端智能语音工具，支持音频转录、实时字幕、语音输入法等功能。

## 一、核心功能模块

### 1.1 音频处理模块
- **多媒体文件处理**
  - 支持格式：MP4, AVI, MKV, MP3, WAV, FLAC, AAC, OGG, M4A
  - 音频流提取与转换
  - 批量处理支持

- **实时音频捕获**
  - 系统音频流捕获（loopback）
  - 麦克风输入捕获
  - 应用程序级别音频隔离（可选特定应用）
  - 混音支持（系统音频 + 麦克风）

### 1.2 语音识别模块
- **离线识别引擎**：SenseVoice.cpp (C++ core)
- **支持语言**：中文、粤语、英语、日语、韩语
- **识别能力**：
  - 语音转文本 (ASR)
  - 语种自动识别 (LID)
  - 情感识别 (SER)
  - 声学事件检测 (AED)

### 1.3 文本后处理模块
- **离线错别字修正**
  - 基于 N-gram 语言模型
  - 拼音纠错
  - 同音字替换
  - 上下文语义分析

- **在线大模型修正**（可选）
  - OpenAI API
  - Claude API
  - 本地 Ollama
  - 通用 OpenAI Compatible API

- **文本格式化**
  - 自动标点符号
  - 段落分割
  - 时间戳注入
  - 说话人分离（diarization）

### 1.4 输出模块
- **字幕文件**：SRT, ASS, VTT, LRC
- **文本文件**：TXT, Markdown, JSON
- **音频文件**：MP3, WAV, FLAC
- **实时输出**：
  - 系统剪贴板
  - 光标位置注入（语音输入法）
  - WebSocket 推流
  - 悬浮窗实时字幕

### 1.5 语音输入法模块
- **全局热键**：可自定义录音快捷键
- **录音模式**：
  - 按住说话（Push-to-Talk）
  - 点击开始/结束
  - 语音激活检测（VAD 自动断句）

- **文本注入**：
  - 系统级输入法注入（Windows IME / macOS Input Method）
  - 模拟键盘输入（跨平台兼容方案）
  - 光标位置自动检测

### 1.6 实时字幕模块
- **悬浮窗显示**
  - 透明背景
  - 可拖动、缩放
  - 自定义字体、颜色、大小
  - 多屏支持

- **直播/录屏集成**
  - OBS 插件支持
  - 虚拟摄像头叠加
  - 浏览器扩展（Chrome/Edge）

---

## 二、技术架构

### 2.1 技术栈选型

| 层次 | 技术选型 | 说明 |
|------|---------|------|
| **GUI 框架** | Tauri + Rust | 轻量级、跨平台、安全 |
| **前端** | Vue 3 + TypeScript | 响应式、组件化 |
| **UI 库** | Element Plus / Naive UI | 成熟的组件库 |
| **核心引擎** | SenseVoice.cpp (C++) | 语音识别核心 |
| **音频处理** | FFmpeg (通过 ffmpeg-rust) | 多媒体处理 |
| **音频捕获** | cpal (Rust) | 跨平台音频 I/O |
| **系统集成** | rdev (全局热键) | 键盘监听 |
| **文本注入** | enigo (Rust) | 模拟键盘输入 |
| **错别字修正** | jieba-rs + 自定义模型 | 中文分词与纠错 |

### 2.2 系统架构图

```
┌─────────────────────────────────────────────────────────┐
│                    Tauri GUI Layer (Rust)               │
│  ┌───────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │ 主窗口管理器   │  │  系统托盘    │  │  悬浮窗管理   │ │
│  └───────────────┘  └──────────────┘  └──────────────┘ │
└─────────────────────────────────────────────────────────┘
           │                    │                    │
           ▼                    ▼                    ▼
┌─────────────────────────────────────────────────────────┐
│                  Frontend (Vue 3 + TS)                  │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────┐ │
│  │ 文件转录  │  │ 实时字幕 │  │ 语音输入 │  │  设置   │ │
│  └──────────┘  └──────────┘  └──────────┘  └─────────┘ │
└─────────────────────────────────────────────────────────┘
           │                    │                    │
           ▼                    ▼                    ▼
┌─────────────────────────────────────────────────────────┐
│                    Rust Backend Core                    │
│  ┌────────────────┐  ┌───────────────┐  ┌────────────┐ │
│  │  FFmpeg 音频   │  │  Audio Capture│  │  Hot Key   │ │
│  │    处理器      │  │   (cpal/wasapi)│  │  Manager   │ │
│  └────────────────┘  └───────────────┘  └────────────┘ │
│  ┌────────────────┐  ┌───────────────┐  ┌────────────┐ │
│  │  SenseVoice    │  │  Text Post-   │  │  Input     │ │
│  │   C++ Bridge   │  │   Processor   │  │  Injector  │ │
│  └────────────────┘  └───────────────┘  └────────────┘ │
└─────────────────────────────────────────────────────────┘
           │                                      │
           ▼                                      ▼
┌──────────────────────┐              ┌────────────────────┐
│  SenseVoice.cpp Core │              │  Optional Services │
│  ┌────────────────┐  │              │  ┌──────────────┐  │
│  │  Model Engine  │  │              │  │  Online LLM  │  │
│  │  (GGML)        │  │              │  │  API Client  │  │
│  └────────────────┘  │              │  └──────────────┘  │
└──────────────────────┘              └────────────────────┘
```

### 2.3 Rust 项目结构

```
sense-voice-desktop/
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── main.rs              # Tauri 主入口
│   │   ├── audio/
│   │   │   ├── capture.rs       # 音频捕获（麦克风/系统音频）
│   │   │   ├── ffmpeg.rs        # FFmpeg 音频处理
│   │   │   └── vad.rs           # 语音活动检测
│   │   ├── recognition/
│   │   │   ├── engine.rs        # SenseVoice C++ 绑定
│   │   │   ├── models.rs        # 模型管理
│   │   │   └── processor.rs     # 实时处理流水线
│   │   ├── text/
│   │   │   ├── corrector.rs     # 错别字修正
│   │   │   ├── formatter.rs     # 文本格式化
│   │   │   └── llm_client.rs    # 在线 LLM 客户端
│   │   ├── input/
│   │   │   ├── hotkey.rs        # 全局热键监听
│   │   │   └── injector.rs      # 文本注入
│   │   ├── export/
│   │   │   ├── subtitle.rs      # 字幕导出（SRT/ASS/VTT）
│   │   │   └── audio.rs         # 音频导出
│   │   └── utils/
│   │       ├── config.rs        # 配置管理
│   │       └── logger.rs        # 日志系统
│   ├── build.rs                 # 构建脚本（链接 C++）
│   └── Cargo.toml
├── src/                         # Vue 前端
│   ├── views/
│   │   ├── FileTranscription.vue   # 文件转录界面
│   │   ├── LiveCaption.vue         # 实时字幕界面
│   │   ├── VoiceInput.vue          # 语音输入界面
│   │   └── Settings.vue            # 设置界面
│   ├── components/
│   │   ├── AudioVisualizer.vue     # 音频波形可视化
│   │   ├── TextEditor.vue          # 文本编辑器
│   │   └── ModelSelector.vue       # 模型选择器
│   ├── App.vue
│   └── main.ts
├── sense-voice-cpp/             # C++ 引擎（git submodule）
└── package.json
```

---

## 三、UI/UX 界面设计原型

### 3.1 主窗口 - 标签页布局

```
┌─────────────────────────────────────────────────────────────────┐
│  SenseVoice Desktop                            [_] [□] [X]       │
├─────────────────────────────────────────────────────────────────┤
│  [📁 文件转录] [🎙️ 实时字幕] [⌨️ 语音输入] [⚙️ 设置]            │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│                         (标签页内容区域)                          │
│                                                                   │
│                                                                   │
│                                                                   │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
│  状态栏: 模型: SenseVoice-Q4K | 内存: 245MB | GPU: 启用         │
└─────────────────────────────────────────────────────────────────┘
```

### 3.2 标签页 1: 📁 文件转录

```
┌─────────────────────────────────────────────────────────────────┐
│  📁 文件转录                                                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │  🎬 拖拽文件到此处或点击选择                              │    │
│  │                                                           │    │
│  │     支持格式: MP4, AVI, MKV, MP3, WAV, FLAC...           │    │
│  │                                                           │    │
│  │              [📂 选择文件] [📂 选择文件夹]                │    │
│  └─────────────────────────────────────────────────────────┘    │
│                                                                   │
│  已添加文件列表:                                                  │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │ ✓ video1.mp4           02:34:12   [转录中... 45%] ████▒▒│    │
│  │ ○ audio.wav            00:15:30   [等待中]              │    │
│  │ ✓ meeting.m4a          01:02:45   [已完成]   [📄查看]   │    │
│  └─────────────────────────────────────────────────────────┘    │
│                                                                   │
│  识别选项:                                                        │
│  ┌──────────────┬──────────────┬──────────────┬─────────────┐   │
│  │ 语言: [自动▼]│ 模型: [Q4K▼] │ 线程: [4▼]   │ ☑ GPU加速  │   │
│  └──────────────┴──────────────┴──────────────┴─────────────┘   │
│                                                                   │
│  输出选项:                                                        │
│  ☑ 字幕文件(SRT)  ☑ 纯文本(TXT)  ☐ JSON格式  ☑ 时间戳           │
│  ☑ 错别字修正     ☐ 在线LLM优化  ☑ 自动标点                     │
│                                                                   │
│  输出目录: /Users/xxx/Documents/transcripts   [📂 选择]          │
│                                                                   │
│                         [🚀 开始批量转录]                        │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### 3.3 标签页 2: 🎙️ 实时字幕

```
┌─────────────────────────────────────────────────────────────────┐
│  🎙️ 实时字幕                                                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  音频源配置:                                                      │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ 🎤 麦克风:    [默认麦克风 - Realtek Audio  ▼]    ☑ 启用  │   │
│  │ 🔊 系统音频:  [扬声器 (Loopback)           ▼]    ☑ 启用  │   │
│  │ 🎯 应用筛选:  [所有应用                    ▼]    ☐ 启用  │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                   │
│  音频可视化:                                                      │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ 🎤 ▂▃▅▇█▇▅▃▂▁▁▂▃▅▆▅▄▃▂▁  音量: ████████▒▒ 78%          │   │
│  │ 🔊 ▁▁▂▃▄▅▆▇█▇▆▅▄▃▂▁▁▁▁▁  音量: ██████▒▒▒▒ 58%          │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                   │
│  实时转录结果:                                                    │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                                                            │   │
│  │  [00:01:23] 你好，欢迎大家来体验达摩院推出的语音识别模型。│   │
│  │  [00:01:28] 这个模型支持中文、英文、粤语等多种语言。      │   │
│  │  [00:01:35] 现在让我们来测试一下实时转录的效果...        │   │
│  │  [00:01:42] ▊ (正在识别中...)                            │   │
│  │                                                            │   │
│  │                                                            │   │
│  │                                                            │   │
│  └──────────────────────────────────────────────────────────┘   │
│  [🗑️ 清空] [📋 复制] [💾 保存] [🪟 悬浮窗]  语言: 中文 | 情感: 😐 │
│                                                                   │
│                    [⏸️ 暂停]      [⏹️ 停止]                      │
│                                                                   │
│  悬浮窗设置:                                                      │
│  透明度: ▓▓▓▓▓▓▓▓▒▒ 80%    字体大小: 24px    颜色: [⚪白色]    │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### 3.4 标签页 3: ⌨️ 语音输入

```
┌─────────────────────────────────────────────────────────────────┐
│  ⌨️ 语音输入法                                                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  快捷键设置:                                                      │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ 开始/停止录音:  [ Ctrl + Shift + Space ] [🔄 修改]       │   │
│  │ 清除结果:       [ Ctrl + Shift + C     ] [🔄 修改]       │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                   │
│  输入模式:                                                        │
│  ⦿ 按下说话 (Push-to-Talk)      按住快捷键期间录音              │
│  ○ 点击切换                     按一次开始，再按一次停止         │
│  ○ 自动断句 (VAD)               自动检测说话停顿                │
│                                                                   │
│  后处理选项:                                                      │
│  ☑ 自动标点                     ☑ 错别字修正                     │
│  ☑ 首字母大写(英文)             ☐ LLM润色                        │
│  ☑ 自动去除"嗯""啊"等语气词                                      │
│                                                                   │
│  录音状态:                                                        │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                                                            │   │
│  │              🎤 按 Ctrl+Shift+Space 开始录音               │   │
│  │                                                            │   │
│  │              状态: 就绪                                    │   │
│  │                                                            │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                   │
│  最近识别:                                                        │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  "欢迎大家来体验语音输入法"            [📋] [↩️] [🗑️]     │   │
│  │  "现在让我们测试一下效果"              [📋] [↩️] [🗑️]     │   │
│  │  "这个功能非常实用"                    [📋] [↩️] [🗑️]     │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                   │
│  提示: 使用快捷键在任何应用程序中输入，文字将自动插入光标位置     │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### 3.5 标签页 4: ⚙️ 设置

```
┌─────────────────────────────────────────────────────────────────┐
│  ⚙️ 设置                                                         │
├─────────────────────────────────────────────────────────────────┤
│  [通用] [模型] [音频] [文本处理] [LLM] [高级]                   │
│  ────────────────────────────────────────────────────────────   │
│                                                                   │
│  【通用设置】                                                     │
│                                                                   │
│  界面语言:        [简体中文 ▼]                                   │
│  主题:            ⦿ 浅色    ○ 深色    ○ 跟随系统                │
│  开机自启动:      ☑ 是                                           │
│  最小化到托盘:    ☑ 是                                           │
│                                                                   │
│  【模型设置】                                                     │
│                                                                   │
│  当前模型:        sense-voice-small-q4k.gguf                     │
│  模型目录:        /Users/xxx/.sensevoice/models  [📂]            │
│                                                                   │
│  可用模型:                                                        │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ ⦿ sense-voice-small-q4k.gguf    (145 MB) [推荐]          │   │
│  │ ○ sense-voice-small-q8.gguf     (198 MB) [高质量]        │   │
│  │ ○ sense-voice-small-fp16.gguf   (469 MB)                 │   │
│  │                                  [➕ 添加模型]            │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                   │
│  【音频设置】                                                     │
│                                                                   │
│  采样率:          [16000 Hz ▼]   (推荐: 16000)                   │
│  声道:            ⦿ 单声道   ○ 立体声                            │
│  降噪:            ☑ 启用    强度: ▓▓▓▒▒ 60%                     │
│                                                                   │
│  【文本处理】                                                     │
│                                                                   │
│  错别字修正:      ⦿ 离线模型   ○ 在线LLM   ○ 关闭               │
│  自定义词典:      /path/to/custom_dict.txt  [📂]                 │
│  过滤语气词:      ☑ 启用 (嗯、啊、呃、那个...)                   │
│                                                                   │
│  【LLM 设置】                                                     │
│                                                                   │
│  服务商:          [OpenAI Compatible ▼]                          │
│  API Endpoint:    https://api.openai.com/v1                      │
│  API Key:         sk-xxxxx [👁️ 显示]                            │
│  模型:            gpt-4                                           │
│  温度:            ▓▓▒▒▒ 0.3                                      │
│                                                                   │
│  【高级设置】                                                     │
│                                                                   │
│  线程数:          [4 ▼]    (CPU: 8 核)                           │
│  GPU加速:         ☑ 启用   设备: [CUDA - RTX 3060 ▼]            │
│  日志级别:        [INFO ▼]                                       │
│  缓存目录:        /Users/xxx/.sensevoice/cache  [🗑️ 清理]       │
│                                                                   │
│                   [💾 保存设置]    [🔄 重置默认]                 │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### 3.6 悬浮窗 - 实时字幕

```
┌─────────────────────────────────────────────────┐
│  🎙️ 实时字幕                        [_] [⚙️] [X] │
├─────────────────────────────────────────────────┤
│                                                  │
│   欢迎大家来体验达摩院推出的语音识别模型。      │
│                                                  │
│   这个模型支持中文、英文、粤语等多种语言。      │
│                                                  │
│   现在让我们来测试一下实时转录的效果...▊        │
│                                                  │
└─────────────────────────────────────────────────┘
```
(半透明、可拖动、置顶、自动滚动)

### 3.7 系统托盘菜单

```
┌──────────────────────────┐
│ 🎙️ SenseVoice Desktop    │
├──────────────────────────┤
│ ▶️ 开始实时字幕           │
│ ⏸️ 暂停                  │
│ 🪟 显示悬浮窗             │
│ ⌨️ 语音输入: 开启 ✓      │
├──────────────────────────┤
│ 📂 打开主窗口             │
│ ⚙️ 设置                  │
│ 📊 使用统计               │
├──────────────────────────┤
│ 🚪 退出                  │
└──────────────────────────┘
```

---

## 四、依赖库清单

### 4.1 Rust 依赖 (Cargo.toml)

```toml
[dependencies]
# GUI 框架
tauri = { version = "2.1", features = ["dialog", "notification", "global-shortcut"] }
tauri-plugin-store = "2.0"

# 音频处理
cpal = "0.15"                    # 跨平台音频 I/O
hound = "3.5"                    # WAV 读写
minimp3 = "0.5"                  # MP3 解码
symphonia = "0.5"                # 全格式音频解码
ffmpeg-next = "7.0"              # FFmpeg 绑定

# 文本处理
jieba-rs = "0.7"                 # 中文分词
pinyin = "0.10"                  # 拼音转换
regex = "1.10"

# LLM 客户端
reqwest = { version = "0.11", features = ["json", "blocking"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.35", features = ["full"] }

# 系统集成
rdev = "0.5"                     # 全局热键
enigo = "0.2"                    # 键盘模拟输入
clipboard = "0.5"                # 剪贴板

# C++ 绑定
cxx = "1.0"                      # Rust-C++ 互操作

# 工具库
anyhow = "1.0"                   # 错误处理
log = "0.4"
env_logger = "0.11"
chrono = "0.4"                   # 时间处理
uuid = { version = "1.6", features = ["v4"] }
```

### 4.2 系统依赖

**Windows:**
- Visual Studio 2019+ (C++ 编译器)
- Windows SDK
- WASAPI (系统音频捕获)

**macOS:**
- Xcode Command Line Tools
- CoreAudio Framework
- Accessibility API (输入法注入)

**Linux:**
- GCC/Clang
- ALSA / PulseAudio / PipeWire
- X11 / Wayland (输入模拟)

---

## 五、核心技术实现方案

### 5.1 音频流处理管道

```rust
// src-tauri/src/audio/capture.rs

pub struct AudioCapture {
    device: cpal::Device,
    config: cpal::StreamConfig,
    buffer: Arc<Mutex<VecDeque<f32>>>,
}

impl AudioCapture {
    pub fn new(source: AudioSource) -> Result<Self> {
        let host = cpal::default_host();
        let device = match source {
            AudioSource::Microphone => host.default_input_device()?,
            AudioSource::SystemAudio => host.default_loopback_device()?, // Windows WASAPI
            AudioSource::Application(name) => // 应用级隔离
        };

        // 配置为 16kHz 单声道 (SenseVoice 要求)
        let config = cpal::StreamConfig {
            channels: 1,
            sample_rate: cpal::SampleRate(16000),
            buffer_size: cpal::BufferSize::Fixed(512),
        };

        Ok(Self { device, config, buffer: Arc::new(Mutex::new(VecDeque::new())) })
    }

    pub fn start_stream(&self, callback: impl Fn(Vec<f32>) + Send + 'static) {
        let buffer = self.buffer.clone();
        let stream = self.device.build_input_stream(
            &self.config,
            move |data: &[f32], _: &_| {
                buffer.lock().unwrap().extend(data);
                // VAD 检测
                if is_speech(data) {
                    callback(data.to_vec());
                }
            },
            |err| eprintln!("Audio error: {}", err),
            None
        ).unwrap();
        stream.play().unwrap();
    }
}
```

### 5.2 SenseVoice C++ 绑定 (CXX)

```rust
// src-tauri/src/recognition/engine.rs

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("sense-voice.h");

        type SenseVoiceContext;

        fn sense_voice_init(model_path: &str) -> UniquePtr<SenseVoiceContext>;
        fn sense_voice_transcribe(ctx: Pin<&mut SenseVoiceContext>, audio: &[f32]) -> String;
        fn sense_voice_free(ctx: UniquePtr<SenseVoiceContext>);
    }
}

pub struct RecognitionEngine {
    context: cxx::UniquePtr<ffi::SenseVoiceContext>,
}

impl RecognitionEngine {
    pub fn new(model_path: &str) -> Result<Self> {
        let context = ffi::sense_voice_init(model_path);
        Ok(Self { context })
    }

    pub fn transcribe(&mut self, audio: &[f32]) -> String {
        unsafe {
            ffi::sense_voice_transcribe(self.context.pin_mut(), audio)
        }
    }
}
```

### 5.3 实时转录流水线

```rust
// src-tauri/src/recognition/processor.rs

pub struct RealtimeProcessor {
    capture: AudioCapture,
    engine: RecognitionEngine,
    text_corrector: TextCorrector,
}

impl RealtimeProcessor {
    pub async fn start(&mut self, tx: mpsc::Sender<TranscriptResult>) {
        self.capture.start_stream(move |audio_chunk| {
            // 1. 语音识别
            let raw_text = self.engine.transcribe(&audio_chunk);

            // 2. 错别字修正
            let corrected = self.text_corrector.correct(&raw_text);

            // 3. 格式化
            let formatted = self.format_text(&corrected);

            // 4. 发送到前端
            tx.send(TranscriptResult {
                text: formatted,
                timestamp: chrono::Utc::now(),
                language: "zh",
                confidence: 0.95,
            }).await.unwrap();
        });
    }
}
```

### 5.4 语音输入法 - 文本注入

```rust
// src-tauri/src/input/injector.rs

use enigo::{Enigo, Key, KeyboardControllable};

pub struct TextInjector {
    enigo: Enigo,
}

impl TextInjector {
    pub fn inject(&mut self, text: &str) {
        // 方法1: 模拟键盘输入（跨平台）
        for ch in text.chars() {
            self.enigo.key_sequence(&ch.to_string());
        }

        // 方法2: 剪贴板粘贴（更快）
        clipboard::set_clipboard(text).unwrap();
        self.enigo.key_down(Key::Control);
        self.enigo.key_click(Key::Layout('v'));
        self.enigo.key_up(Key::Control);
    }
}

// 全局热键监听
pub fn register_hotkey(callback: impl Fn() + Send + 'static) {
    use rdev::{listen, Event, EventType, Key};

    std::thread::spawn(move || {
        listen(move |event| {
            if let Event { event_type: EventType::KeyPress(Key::Space), .. } = event {
                if is_ctrl_shift_pressed() {
                    callback();
                }
            }
        }).unwrap();
    });
}
```

### 5.5 错别字修正引擎

```rust
// src-tauri/src/text/corrector.rs

pub struct TextCorrector {
    jieba: jieba_rs::Jieba,
    confusion_set: HashMap<String, Vec<String>>, // 同音字混淆集
    lm: LanguageModel, // N-gram 语言模型
}

impl TextCorrector {
    pub fn correct(&self, text: &str) -> String {
        // 1. 分词
        let words = self.jieba.cut(text, false);

        // 2. 逐词检查
        let corrected: Vec<String> = words.into_iter().map(|word| {
            if let Some(candidates) = self.confusion_set.get(word) {
                // 使用语言模型选择最佳候选
                self.lm.best_candidate(candidates)
            } else {
                word.to_string()
            }
        }).collect();

        corrected.join("")
    }
}

// 混淆字典示例
// "的" -> ["的", "得", "地"]
// "在" -> ["在", "再"]
// "做" -> ["做", "作"]
```

### 5.6 LLM 在线修正

```rust
// src-tauri/src/text/llm_client.rs

pub struct LLMClient {
    endpoint: String,
    api_key: String,
    client: reqwest::Client,
}

impl LLMClient {
    pub async fn correct(&self, text: &str) -> Result<String> {
        let prompt = format!(
            "请修正以下语音识别结果中的错别字和标点符号，保持原意不变:\n\n{}",
            text
        );

        let response = self.client.post(&self.endpoint)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "model": "gpt-4",
                "messages": [{"role": "user", "content": prompt}],
                "temperature": 0.3,
            }))
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        Ok(result["choices"][0]["message"]["content"].as_str().unwrap().to_string())
    }
}
```

### 5.7 字幕导出

```rust
// src-tauri/src/export/subtitle.rs

pub fn export_srt(segments: &[Segment], path: &Path) -> Result<()> {
    let mut output = String::new();

    for (i, seg) in segments.iter().enumerate() {
        output.push_str(&format!(
            "{}\n{} --> {}\n{}\n\n",
            i + 1,
            format_timestamp(seg.start_time),
            format_timestamp(seg.end_time),
            seg.text
        ));
    }

    std::fs::write(path, output)?;
    Ok(())
}

fn format_timestamp(ms: u64) -> String {
    let hours = ms / 3600000;
    let minutes = (ms % 3600000) / 60000;
    let seconds = (ms % 60000) / 1000;
    let millis = ms % 1000;
    format!("{:02}:{:02}:{:02},{:03}", hours, minutes, seconds, millis)
}
```

---

## 六、体积与性能评估

### 6.1 应用体积估算

| 组件 | 大小 | 说明 |
|------|------|------|
| **Tauri Runtime** | ~3 MB | WebView2 (Windows) / WebKit (macOS/Linux) |
| **Rust 编译后二进制** | ~8 MB | 包含所有 Rust 代码 + FFmpeg 绑定 |
| **SenseVoice C++ 引擎** | ~1.6 MB | 核心推理库 |
| **前端资源 (Vue)** | ~2 MB | HTML/CSS/JS (gzipped) |
| **FFmpeg 动态库** | ~15 MB | 音频/视频解码 |
| **模型文件 (Q4_K)** | ~130 MB | 默认内置 |
| **错别字词典** | ~5 MB | N-gram 模型 + 混淆字典 |
| **其他资源** | ~2 MB | 图标、字体等 |

**总计（不含模型）**: ~37 MB
**总计（含 Q4_K 模型）**: **~167 MB**
**总计（含 Q3_K 模型）**: **~137 MB** ✨ (最小推荐配置)

### 6.2 运行时内存占用

| 场景 | 内存占用 |
|------|---------|
| 空闲状态 | ~80 MB |
| 文件转录（Q4_K） | ~250 MB |
| 实时字幕（Q4_K） | ~300 MB |
| 语音输入（Q4_K） | ~220 MB |
| 在线LLM优化 | +50 MB |

### 6.3 性能指标

| 指标 | 数值 | 硬件配置 |
|------|------|---------|
| 启动时间 | <2s | SSD |
| 模型加载 | <1s | Q4_K |
| 实时转录延迟 | <200ms | GPU / <500ms CPU |
| RTF (实时率) | 0.02-0.05 | 比实时快 20-50 倍 |
| 文件转录速度 | ~100x | 1小时音频 < 40秒 |

---

## 七、开发路线图

### Phase 1: MVP (4-6 周)
- [x] 架构设计完成
- [ ] Tauri + Vue 项目搭建
- [ ] SenseVoice C++ 绑定
- [ ] 文件转录功能
- [ ] 基础 UI/UX
- [ ] 打包与分发 (Windows/macOS/Linux)

### Phase 2: 实时功能 (3-4 周)
- [ ] 音频捕获（麦克风 + 系统音频）
- [ ] 实时转录引擎
- [ ] 悬浮窗实时字幕
- [ ] 文本后处理（错别字修正）

### Phase 3: 语音输入法 (2-3 周)
- [ ] 全局热键监听
- [ ] 文本自动注入
- [ ] VAD 自动断句
- [ ] 系统托盘集成

### Phase 4: 高级功能 (3-4 周)
- [ ] LLM 在线修正
- [ ] 多格式字幕导出
- [ ] 应用级音频隔离
- [ ] 批量处理优化

### Phase 5: 优化与发布 (2-3 周)
- [ ] 性能优化
- [ ] 安装包优化（体积压缩）
- [ ] 用户文档
- [ ] CI/CD 自动化构建
- [ ] 1.0 正式版发布

---

## 八、安装包分发策略

### 8.1 安装包类型

**Windows:**
- MSI Installer (推荐): ~140 MB
- Portable ZIP: ~170 MB

**macOS:**
- DMG: ~145 MB
- Homebrew Cask: `brew install sensevoice-desktop`

**Linux:**
- AppImage (通用): ~150 MB
- DEB (Debian/Ubuntu): ~140 MB
- RPM (Fedora/RHEL): ~140 MB

### 8.2 模型分发策略

**方案1: 内置默认模型 (推荐)**
- 打包 Q3_K 或 Q4_K 模型
- 用户可在设置中下载其他模型

**方案2: 首次运行下载**
- 安装包仅 ~40 MB
- 首次启动时下载模型
- 适合带宽敏感场景

### 8.3 自动更新
- 使用 Tauri Updater
- 增量更新（仅下载变更部分）
- 后台静默更新

---

## 九、商业化与开源策略

### 开源版 (MIT License)
- ✅ 所有核心功能
- ✅ 本地离线识别
- ✅ 基础错别字修正
- ❌ 在线 LLM 功能（需自己配置 API Key）

### Pro 版 (付费 / 订阅)
- ✅ 内置商业 LLM API 额度
- ✅ 云端模型管理
- ✅ 多设备同步
- ✅ 高级自定义词典
- ✅ 优先技术支持

---

## 十、总结

这是一个**技术可行、用户需求明确、商业潜力巨大**的项目。

### 核心优势
1. **完全离线**: 不依赖网络，隐私安全
2. **轻量高效**: 体积 <200MB，内存占用 <300MB
3. **跨平台**: Windows/macOS/Linux 全支持
4. **功能丰富**: 文件转录、实时字幕、语音输入法三位一体
5. **可扩展**: 支持 LLM 增强、自定义词典

### 技术亮点
- Rust + Tauri: 安全、高性能、跨平台
- SenseVoice.cpp: 工业级语音识别引擎
- 模块化设计: 易于维护和扩展

### 应用场景
- 📹 **视频创作**: 自动生成字幕
- 🎙️ **会议记录**: 实时转录会议内容
- ✍️ **写作辅助**: 语音转文字提升效率
- 🎮 **直播**: OBS 实时字幕插件
- ♿ **无障碍**: 听障人士辅助工具

期待看到这个项目落地! 🚀
