# SenseVoice Desktop App - gpui 实现方案

## 目录

1. [gpui 框架概述](#gpui-框架概述)
2. [为什么选择 gpui](#为什么选择-gpui)
3. [项目架构设计](#项目架构设计)
4. [技术栈](#技术栈)
5. [项目结构](#项目结构)
6. [核心组件设计](#核心组件设计)
7. [与 SenseVoice.cpp 集成](#与-sensevoicecpp-集成)
8. [平台特定实现](#平台特定实现)
9. [构建配置](#构建配置)
10. [开发路线图](#开发路线图)

---

## gpui 框架概述

**gpui** 是由 Zed Industries 开发的高性能 Rust GUI 框架，用于构建 Zed 代码编辑器。

### 核心特性

- **GPU 加速渲染**: 使用 GPU 进行高效渲染
- **声明式 UI**: React-like 组件模型
- **类型安全**: 完全的 Rust 类型系统保障
- **高性能**: 零成本抽象，接近原生性能
- **跨平台**: macOS, Linux, Windows
- **现代化**: 支持 Flexbox 布局、动画、手势等

### 架构模式

```rust
// gpui 的核心概念
struct AppState {
    files: Vec<TranscriptionTask>,
    settings: Settings,
}

impl Render for MyView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap_4()
            .child(text("Hello"))
    }
}
```

---

## 为什么选择 gpui

### 对比其他方案

| 框架 | 语言 | 性能 | 成熟度 | 生态 | 适合 SenseVoice? |
|------|------|------|-------|------|-----------------|
| **gpui** | Rust | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ✅ **最推荐** |
| egui | Rust | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ✅ 备选方案 |
| Tauri | Rust+Web | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⚠️ 性能较低 |
| iced | Rust | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ✅ 备选方案 |
| Qt (Rust bindings) | C++ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⚠️ 绑定复杂 |

### gpui 的优势

1. **纯 Rust 生态**
   - 与 `sensevoice-audio-processor` 无缝集成
   - 与 SenseVoice.cpp FFI 集成简单
   - 类型安全，无运行时错误

2. **极致性能**
   - GPU 加速渲染
   - 高效的重绘机制
   - 适合实时音频可视化

3. **现代化 UI**
   - 声明式组件模型
   - Flexbox 布局
   - 流畅动画

4. **实战验证**
   - Zed 编辑器已证明其生产可用性
   - 活跃的开发和社区

### 潜在挑战

1. **文档不足**: gpui 文档相对较少，需要参考 Zed 源码
2. **API 变动**: 框架仍在快速迭代
3. **学习曲线**: 需要理解 gpui 的状态管理和渲染模型
4. **组件库**: 需要自行实现大部分 UI 组件

---

## 项目架构设计

### 三层架构

```
┌───────────────────────────────────────────────────────┐
│                   Presentation Layer                  │
│                     (gpui Views)                      │
│  ┌──────────────┐  ┌──────────────┐  ┌─────────────┐ │
│  │ MainWindow   │  │ SettingsView │  │ SubtitleHUD │ │
│  │              │  │              │  │             │ │
│  │ - FileView   │  │ - HotkeyTab  │  │ (Overlay)   │ │
│  │ - RealtimeV  │  │ - ModelTab   │  │             │ │
│  │ - InputView  │  │ - LLMTab     │  │             │ │
│  │ - HistoryV   │  │ - OutputTab  │  │             │ │
│  └──────────────┘  └──────────────┘  └─────────────┘ │
└───────────────────────────────────────────────────────┘
                         ↓ ↑
┌───────────────────────────────────────────────────────┐
│                   Business Logic Layer                │
│                   (Application State)                 │
│  ┌────────────────────────────────────────────────┐   │
│  │ AppState (Model)                               │   │
│  │  ├─ tasks: Vec<TranscriptionTask>              │   │
│  │  ├─ settings: Settings                         │   │
│  │  ├─ realtime_session: Option<RealtimeSession>  │   │
│  │  └─ history: Vec<CompletedTask>                │   │
│  └────────────────────────────────────────────────┘   │
│  ┌────────────────────────────────────────────────┐   │
│  │ Controllers / Services                         │   │
│  │  ├─ TaskQueueManager                           │   │
│  │  ├─ HotkeyManager                              │   │
│  │  ├─ LLMClient                                  │   │
│  │  ├─ ConfigManager                              │   │
│  │  └─ NotificationService                        │   │
│  └────────────────────────────────────────────────┘   │
└───────────────────────────────────────────────────────┘
                         ↓ ↑
┌───────────────────────────────────────────────────────┐
│                   Core Processing Layer               │
│  ┌────────────────────────────────────────────────┐   │
│  │ SenseVoice Engine (C++ FFI)                    │   │
│  │  ├─ Model loading                              │   │
│  │  ├─ Inference                                  │   │
│  │  └─ Language detection                         │   │
│  └────────────────────────────────────────────────┘   │
│  ┌────────────────────────────────────────────────┐   │
│  │ Audio Processor (Rust + SIMD)                  │   │
│  │  ├─ File decoding (Symphonia)                  │   │
│  │  ├─ Real-time capture (cpal)                   │   │
│  │  ├─ Resampling (Rubato)                        │   │
│  │  └─ SIMD processing                            │   │
│  └────────────────────────────────────────────────┘   │
│  ┌────────────────────────────────────────────────┐   │
│  │ Platform Services                              │   │
│  │  ├─ Global hotkeys (global-hotkey)             │   │
│  │  ├─ Text injection (enigo)                     │   │
│  │  ├─ System tray (tray-icon)                    │   │
│  │  └─ File system (notify)                       │   │
│  └────────────────────────────────────────────────┘   │
└───────────────────────────────────────────────────────┘
```

### 数据流

```
用户操作 → View 事件 → Action → AppState 更新 → View 重新渲染
     ↑                                              ↓
     └────────────── 后台任务完成 ──────────────────┘
```

---

## 技术栈

### 核心依赖

```toml
[dependencies]
# GUI Framework
gpui = { git = "https://github.com/zed-industries/zed", branch = "main" }

# SenseVoice 集成
sensevoice-sys = { path = "./sensevoice-sys" }  # C++ FFI 绑定
sensevoice-audio-processor = { path = "./audio-processor" }

# 平台集成
global-hotkey = "0.5"          # 全局热键
tray-icon = "0.14"             # 系统托盘
enigo = "0.2"                  # 文本注入
cpal = "0.15"                  # 音频捕获
notify = "6.1"                 # 文件系统监控

# 异步运行时
tokio = { version = "1.35", features = ["full"] }
async-trait = "0.1"

# LLM 集成
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 日志和错误处理
log = "0.4"
env_logger = "0.11"
anyhow = "1.0"
thiserror = "1.0"

# 实用工具
chrono = "0.4"                 # 时间处理
uuid = { version = "1.6", features = ["v4"] }
dirs = "5.0"                   # 系统目录
```

### 开发依赖

```toml
[dev-dependencies]
criterion = "0.5"              # 性能基准测试
proptest = "1.4"               # 属性测试
```

---

## 项目结构

```
sensevoice-desktop/
├── Cargo.toml                 # 工作空间配置
├── build.rs                   # 构建脚本（链接 C++）
├── README.md
├── LICENSE
│
├── src/
│   ├── main.rs                # 应用入口
│   ├── app.rs                 # 应用主结构
│   ├── state.rs               # 全局状态
│   │
│   ├── ui/                    # UI 层
│   │   ├── mod.rs
│   │   ├── main_window.rs     # 主窗口
│   │   ├── theme.rs           # 主题系统
│   │   ├── components/        # 可复用组件
│   │   │   ├── mod.rs
│   │   │   ├── button.rs
│   │   │   ├── input.rs
│   │   │   ├── select.rs
│   │   │   ├── progress.rs
│   │   │   ├── modal.rs
│   │   │   └── file_item.rs
│   │   ├── views/             # 主要视图
│   │   │   ├── mod.rs
│   │   │   ├── file_transcription.rs
│   │   │   ├── realtime_subtitle.rs
│   │   │   ├── voice_input.rs
│   │   │   ├── history.rs
│   │   │   └── settings.rs
│   │   └── overlays/          # 浮层
│   │       ├── mod.rs
│   │       ├── subtitle_hud.rs
│   │       └── notification.rs
│   │
│   ├── core/                  # 业务逻辑层
│   │   ├── mod.rs
│   │   ├── task.rs            # 任务定义
│   │   ├── task_queue.rs      # 任务队列管理
│   │   ├── settings.rs        # 设置管理
│   │   ├── hotkey.rs          # 全局热键
│   │   ├── llm_client.rs      # LLM API 客户端
│   │   └── history.rs         # 历史记录
│   │
│   ├── engine/                # 核心处理层
│   │   ├── mod.rs
│   │   ├── sensevoice.rs      # SenseVoice 引擎封装
│   │   ├── audio.rs           # 音频处理
│   │   ├── realtime.rs        # 实时音频捕获
│   │   └── subtitle.rs        # 字幕生成
│   │
│   ├── platform/              # 平台特定代码
│   │   ├── mod.rs
│   │   ├── windows.rs         # Windows 特定
│   │   ├── macos.rs           # macOS 特定
│   │   └── linux.rs           # Linux 特定
│   │
│   └── utils/                 # 工具函数
│       ├── mod.rs
│       ├── file.rs            # 文件操作
│       ├── time.rs            # 时间处理
│       └── format.rs          # 格式转换
│
├── sensevoice-sys/            # SenseVoice C++ FFI 绑定
│   ├── Cargo.toml
│   ├── build.rs               # bindgen 配置
│   ├── src/
│   │   └── lib.rs             # FFI 接口
│   └── wrapper.h              # C 头文件
│
├── audio-processor/           # 音频处理库（已存在）
│   └── ...
│
├── assets/                    # 资源文件
│   ├── icons/
│   │   ├── app.icns           # macOS 图标
│   │   ├── app.ico            # Windows 图标
│   │   └── app.png            # Linux 图标
│   ├── fonts/
│   │   └── ...
│   └── models/                # 默认模型（可选）
│       └── ...
│
├── scripts/                   # 构建脚本
│   ├── build-macos.sh
│   ├── build-windows.sh
│   └── build-linux.sh
│
└── docs/
    ├── GPUI_IMPLEMENTATION_PLAN.md  # 本文档
    ├── UI_PROTOTYPES_SUMMARY.md
    └── API.md
```

---

## 核心组件设计

### 1. 应用入口 (main.rs)

```rust
use gpui::*;

fn main() {
    env_logger::init();

    App::new().run(|cx: &mut AppContext| {
        // 初始化全局服务
        cx.set_global(Settings::load());

        // 创建主窗口
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx| MainWindow::new(cx))
        });

        // 初始化系统托盘
        cx.spawn(|cx| async move {
            platform::init_system_tray(cx).await
        }).detach();

        // 初始化全局热键
        cx.spawn(|cx| async move {
            hotkey::init_global_hotkeys(cx).await
        }).detach();
    });
}
```

### 2. 应用状态 (state.rs)

```rust
use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    pub tasks: Model<TaskQueue>,
    pub settings: Model<Settings>,
    pub realtime_session: Option<Model<RealtimeSession>>,
    pub history: Model<History>,
}

impl AppState {
    pub fn new(cx: &mut AppContext) -> Self {
        Self {
            tasks: cx.new_model(|_| TaskQueue::new()),
            settings: cx.new_model(|_| Settings::load()),
            realtime_session: None,
            history: cx.new_model(|_| History::load()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TranscriptionTask {
    pub id: String,
    pub file_path: PathBuf,
    pub status: TaskStatus,
    pub progress: f32,
    pub result: Option<TranscriptionResult>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Processing,
    Completed,
    Failed(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TranscriptionResult {
    pub text: String,
    pub segments: Vec<Segment>,
    pub language: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Segment {
    pub start: f64,
    pub end: f64,
    pub text: String,
}
```

### 3. 主窗口 (ui/main_window.rs)

```rust
use gpui::*;

pub struct MainWindow {
    state: Model<AppState>,
    current_tab: Tab,
}

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    FileTranscription,
    RealtimeSubtitle,
    VoiceInput,
    History,
}

impl MainWindow {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let state = cx.new_model(|cx| AppState::new(cx));
        Self {
            state,
            current_tab: Tab::FileTranscription,
        }
    }

    fn switch_tab(&mut self, tab: Tab, cx: &mut ViewContext<Self>) {
        self.current_tab = tab;
        cx.notify();
    }
}

impl Render for MainWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme().background)
            .child(self.render_header(cx))
            .child(
                div()
                    .flex()
                    .flex_1()
                    .child(self.render_sidebar(cx))
                    .child(self.render_content(cx))
            )
            .child(self.render_status_bar(cx))
    }
}

impl MainWindow {
    fn render_header(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .justify_between()
            .p_4()
            .bg(theme().primary)
            .child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme().on_primary)
                    .child("🎙️ SenseVoice")
            )
            .child(
                Button::new("settings", cx)
                    .label("⚙️ 设置")
                    .on_click(|_, cx| {
                        cx.emit(Action::OpenSettings)
                    })
            )
    }

    fn render_sidebar(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_20()
            .bg(theme().surface)
            .p_2()
            .gap_2()
            .child(self.nav_item(Tab::FileTranscription, "📁", cx))
            .child(self.nav_item(Tab::RealtimeSubtitle, "🎤", cx))
            .child(self.nav_item(Tab::VoiceInput, "⌨️", cx))
            .child(self.nav_item(Tab::History, "🕐", cx))
    }

    fn nav_item(&self, tab: Tab, icon: &str, cx: &ViewContext<Self>) -> impl IntoElement {
        let is_active = self.current_tab == tab;

        div()
            .w_full()
            .h_14()
            .flex()
            .items_center()
            .justify_center()
            .rounded_lg()
            .cursor_pointer()
            .when(is_active, |this| {
                this.bg(theme().primary_container)
            })
            .on_click(cx.listener(move |this, _, cx| {
                this.switch_tab(tab, cx);
            }))
            .child(
                div()
                    .text_2xl()
                    .child(icon)
            )
    }

    fn render_content(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        match self.current_tab {
            Tab::FileTranscription => {
                div().child(cx.new_view(|cx| FileTranscriptionView::new(cx, self.state.clone())))
            }
            Tab::RealtimeSubtitle => {
                div().child(cx.new_view(|cx| RealtimeSubtitleView::new(cx, self.state.clone())))
            }
            Tab::VoiceInput => {
                div().child(cx.new_view(|cx| VoiceInputView::new(cx, self.state.clone())))
            }
            Tab::History => {
                div().child(cx.new_view(|cx| HistoryView::new(cx, self.state.clone())))
            }
        }
    }

    fn render_status_bar(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .justify_between()
            .p_2()
            .bg(theme().surface_variant)
            .text_sm()
            .child("就绪")
            .child("模型: Q4_K")
            .child("GPU: 已启用")
    }
}
```

### 4. 文件转录视图 (ui/views/file_transcription.rs)

```rust
use gpui::*;
use std::path::PathBuf;

pub struct FileTranscriptionView {
    state: Model<AppState>,
    drag_active: bool,
}

impl FileTranscriptionView {
    pub fn new(cx: &mut ViewContext<Self>, state: Model<AppState>) -> Self {
        Self {
            state,
            drag_active: false,
        }
    }

    fn handle_file_drop(&mut self, paths: Vec<PathBuf>, cx: &mut ViewContext<Self>) {
        for path in paths {
            self.state.update(cx, |state, cx| {
                state.tasks.update(cx, |queue, _| {
                    queue.add_task(TranscriptionTask {
                        id: uuid::Uuid::new_v4().to_string(),
                        file_path: path,
                        status: TaskStatus::Pending,
                        progress: 0.0,
                        result: None,
                    });
                });
            });
        }

        self.start_processing(cx);
    }

    fn start_processing(&mut self, cx: &mut ViewContext<Self>) {
        cx.spawn(|this, mut cx| async move {
            // 异步处理任务队列
            // ...
        }).detach();
    }
}

impl Render for FileTranscriptionView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .p_8()
            .gap_4()
            .child(self.render_upload_area(cx))
            .child(self.render_options(cx))
            .child(self.render_file_list(cx))
    }
}

impl FileTranscriptionView {
    fn render_upload_area(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .h_48()
            .border_2()
            .border_dashed()
            .border_color(theme().primary)
            .rounded_xl()
            .bg(theme().primary_container)
            .cursor_pointer()
            .when(self.drag_active, |this| {
                this.bg(theme().secondary_container)
            })
            .on_drop(cx.listener(|this, paths, cx| {
                this.handle_file_drop(paths, cx);
            }))
            .child(
                div()
                    .text_5xl()
                    .mb_4()
                    .child("📁")
            )
            .child(
                div()
                    .text_lg()
                    .child("点击或拖拽文件到此处")
            )
            .child(
                div()
                    .text_sm()
                    .text_color(theme().on_surface_variant)
                    .child("支持 MP3, WAV, MP4, AVI, MKV 等格式")
            )
    }

    fn render_options(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap_4()
            .child(
                Select::new("language", cx)
                    .label("识别语言")
                    .options(vec!["自动检测", "中文", "粤语", "英语", "日语", "韩语"])
            )
            .child(
                Select::new("model", cx)
                    .label("模型选择")
                    .options(vec!["Q4_K (推荐)", "Q8_0 (高质量)", "FP16 (最高质量)"])
            )
    }

    fn render_file_list(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let tasks = self.state.read(cx).tasks.read(cx).tasks.clone();

        div()
            .flex()
            .flex_col()
            .gap_3()
            .children(tasks.iter().map(|task| {
                FileItem::new(task.clone(), cx)
            }))
    }
}
```

### 5. 设置管理 (core/settings.rs)

```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub hotkeys: HotkeySettings,
    pub model: ModelSettings,
    pub llm: LLMSettings,
    pub output: OutputSettings,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HotkeySettings {
    pub record: String,              // "Ctrl+Shift+Space"
    pub voice_input: String,         // "Ctrl+Alt+V"
    pub realtime_subtitle: String,   // "Ctrl+Shift+S"
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelSettings {
    pub model_path: PathBuf,
    pub model_type: ModelType,
    pub use_gpu: bool,
    pub num_threads: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ModelType {
    Q3K,
    Q4K,
    Q8,
    FP16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LLMSettings {
    pub enabled: bool,
    pub provider: LLMProvider,
    pub api_endpoint: String,
    pub api_key: String,
    pub model_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LLMProvider {
    OpenAI,
    Anthropic,
    Ollama,
    Custom,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputSettings {
    pub output_dir: PathBuf,
    pub auto_open: bool,
    pub formats: Vec<OutputFormat>,
    pub include_timestamps: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OutputFormat {
    SRT,
    ASS,
    VTT,
    TXT,
    Markdown,
    JSON,
}

impl Settings {
    pub fn load() -> Self {
        // 从配置文件加载
        let config_path = dirs::config_dir()
            .unwrap()
            .join("sensevoice")
            .join("config.toml");

        if config_path.exists() {
            let content = std::fs::read_to_string(config_path).unwrap();
            toml::from_str(&content).unwrap()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let config_dir = dirs::config_dir()
            .unwrap()
            .join("sensevoice");

        std::fs::create_dir_all(&config_dir)?;

        let config_path = config_dir.join("config.toml");
        let content = toml::to_string_pretty(self)?;
        std::fs::write(config_path, content)?;

        Ok(())
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            hotkeys: HotkeySettings {
                record: "Ctrl+Shift+Space".to_string(),
                voice_input: "Ctrl+Alt+V".to_string(),
                realtime_subtitle: "Ctrl+Shift+S".to_string(),
            },
            model: ModelSettings {
                model_path: PathBuf::from("/models/sensevoice-q4k.gguf"),
                model_type: ModelType::Q4K,
                use_gpu: true,
                num_threads: None,
            },
            llm: LLMSettings {
                enabled: false,
                provider: LLMProvider::OpenAI,
                api_endpoint: "https://api.openai.com/v1".to_string(),
                api_key: String::new(),
                model_name: "gpt-4-turbo".to_string(),
            },
            output: OutputSettings {
                output_dir: dirs::document_dir()
                    .unwrap()
                    .join("SenseVoice/Output"),
                auto_open: true,
                formats: vec![OutputFormat::SRT, OutputFormat::TXT],
                include_timestamps: true,
            },
        }
    }
}
```

---

## 与 SenseVoice.cpp 集成

### FFI 绑定 (sensevoice-sys)

**sensevoice-sys/wrapper.h**:

```c
#ifndef SENSEVOICE_WRAPPER_H
#define SENSEVOICE_WRAPPER_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct SenseVoiceContext SenseVoiceContext;

// 创建上下文
SenseVoiceContext* sensevoice_create_context(const char* model_path, int num_threads, int use_gpu);

// 销毁上下文
void sensevoice_free_context(SenseVoiceContext* ctx);

// 转录音频
typedef struct {
    char* text;
    float start_time;
    float end_time;
} SenseVoiceSegment;

typedef struct {
    SenseVoiceSegment* segments;
    size_t num_segments;
    char* language;
} SenseVoiceResult;

SenseVoiceResult* sensevoice_transcribe(
    SenseVoiceContext* ctx,
    const float* audio_data,
    size_t audio_length
);

void sensevoice_free_result(SenseVoiceResult* result);

#ifdef __cplusplus
}
#endif

#endif
```

**sensevoice-sys/build.rs**:

```rust
use std::env;
use std::path::PathBuf;

fn main() {
    // 链接 SenseVoice.cpp 库
    println!("cargo:rustc-link-search=../build");
    println!("cargo:rustc-link-lib=static=sensevoice");

    // 链接系统库
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=Accelerate");
        println!("cargo:rustc-link-lib=framework=Metal");
    }

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=dylib=ole32");
    }

    // 生成 Rust 绑定
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
```

**sensevoice-sys/src/lib.rs**:

```rust
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{CStr, CString};
use std::path::Path;

pub struct SenseVoice {
    ctx: *mut SenseVoiceContext,
}

unsafe impl Send for SenseVoice {}
unsafe impl Sync for SenseVoice {}

impl SenseVoice {
    pub fn new(model_path: &Path, num_threads: i32, use_gpu: bool) -> anyhow::Result<Self> {
        let path_str = CString::new(model_path.to_str().unwrap())?;

        let ctx = unsafe {
            sensevoice_create_context(
                path_str.as_ptr(),
                num_threads,
                if use_gpu { 1 } else { 0 },
            )
        };

        if ctx.is_null() {
            anyhow::bail!("Failed to create SenseVoice context");
        }

        Ok(Self { ctx })
    }

    pub fn transcribe(&self, audio: &[f32]) -> anyhow::Result<TranscriptionResult> {
        let result = unsafe {
            sensevoice_transcribe(self.ctx, audio.as_ptr(), audio.len())
        };

        if result.is_null() {
            anyhow::bail!("Transcription failed");
        }

        let result_ref = unsafe { &*result };

        let segments: Vec<Segment> = unsafe {
            std::slice::from_raw_parts(result_ref.segments, result_ref.num_segments)
                .iter()
                .map(|seg| Segment {
                    start: seg.start_time as f64,
                    end: seg.end_time as f64,
                    text: CStr::from_ptr(seg.text).to_string_lossy().into_owned(),
                })
                .collect()
        };

        let language = unsafe {
            CStr::from_ptr(result_ref.language)
                .to_string_lossy()
                .into_owned()
        };

        let text = segments.iter().map(|s| s.text.clone()).collect::<Vec<_>>().join(" ");

        unsafe {
            sensevoice_free_result(result);
        }

        Ok(TranscriptionResult {
            text,
            segments,
            language,
        })
    }
}

impl Drop for SenseVoice {
    fn drop(&mut self) {
        unsafe {
            sensevoice_free_context(self.ctx);
        }
    }
}

#[derive(Clone, Debug)]
pub struct TranscriptionResult {
    pub text: String,
    pub segments: Vec<Segment>,
    pub language: String,
}

#[derive(Clone, Debug)]
pub struct Segment {
    pub start: f64,
    pub end: f64,
    pub text: String,
}
```

### 引擎封装 (src/engine/sensevoice.rs)

```rust
use sensevoice_sys::SenseVoice;
use std::path::Path;
use tokio::sync::mpsc;

pub struct SenseVoiceEngine {
    engine: SenseVoice,
}

impl SenseVoiceEngine {
    pub fn new(model_path: &Path, num_threads: i32, use_gpu: bool) -> anyhow::Result<Self> {
        let engine = SenseVoice::new(model_path, num_threads, use_gpu)?;
        Ok(Self { engine })
    }

    pub async fn transcribe_file(
        &self,
        file_path: &Path,
        progress_tx: mpsc::Sender<f32>,
    ) -> anyhow::Result<TranscriptionResult> {
        // 1. 解码音频文件
        progress_tx.send(0.1).await?;
        let audio = sensevoice_audio_processor::process_audio_file(
            file_path,
            &AudioConfig::default(),
        )?;

        // 2. 转录
        progress_tx.send(0.5).await?;
        let result = self.engine.transcribe(&audio)?;

        // 3. 完成
        progress_tx.send(1.0).await?;

        Ok(result)
    }

    pub async fn transcribe_realtime(
        &self,
        audio_rx: mpsc::Receiver<Vec<f32>>,
        result_tx: mpsc::Sender<String>,
    ) -> anyhow::Result<()> {
        let mut buffer = Vec::new();
        let chunk_size = 16000 * 3; // 3 秒

        while let Some(chunk) = audio_rx.recv().await {
            buffer.extend_from_slice(&chunk);

            if buffer.len() >= chunk_size {
                let result = self.engine.transcribe(&buffer)?;
                result_tx.send(result.text).await?;
                buffer.clear();
            }
        }

        Ok(())
    }
}
```

---

## 平台特定实现

### Windows 实现 (src/platform/windows.rs)

```rust
#[cfg(target_os = "windows")]
use windows::Win32::Media::Audio::*;

pub fn capture_system_audio() -> anyhow::Result<AudioCapture> {
    // WASAPI Loopback 实现
    // 参考 docs/rust-audio-architecture.md
    todo!()
}

pub fn inject_text(text: &str) -> anyhow::Result<()> {
    use enigo::*;

    let mut enigo = Enigo::new();
    enigo.text(text)?;

    Ok(())
}
```

### macOS 实现 (src/platform/macos.rs)

```rust
#[cfg(target_os = "macos")]
pub fn capture_system_audio() -> anyhow::Result<AudioCapture> {
    // ScreenCaptureKit 实现
    todo!()
}

pub fn inject_text(text: &str) -> anyhow::Result<()> {
    use enigo::*;

    let mut enigo = Enigo::new();
    enigo.text(text)?;

    Ok(())
}
```

### Linux 实现 (src/platform/linux.rs)

```rust
#[cfg(target_os = "linux")]
pub fn capture_system_audio() -> anyhow::Result<AudioCapture> {
    // PulseAudio Monitor 实现
    todo!()
}

pub fn inject_text(text: &str) -> anyhow::Result<()> {
    use enigo::*;

    let mut enigo = Enigo::new();
    enigo.text(text)?;

    Ok(())
}
```

---

## 构建配置

### Cargo.toml (工作空间)

```toml
[workspace]
members = [
    "sensevoice-desktop",
    "sensevoice-sys",
    "audio-processor",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["SenseVoice Team"]
license = "MIT"

[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
strip = true

[profile.release-with-debug]
inherits = "release"
strip = false
debug = true
```

### sensevoice-desktop/Cargo.toml

```toml
[package]
name = "sensevoice-desktop"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[[bin]]
name = "sensevoice"
path = "src/main.rs"

[dependencies]
# GUI
gpui = { git = "https://github.com/zed-industries/zed", branch = "main" }

# 内部依赖
sensevoice-sys = { path = "../sensevoice-sys" }
sensevoice-audio-processor = { path = "../audio-processor" }

# 平台集成
global-hotkey = "0.5"
tray-icon = "0.14"
enigo = "0.2"
cpal = "0.15"
notify = "6.1"

# 异步
tokio = { version = "1.35", features = ["full"] }
async-trait = "0.1"

# HTTP / LLM
reqwest = { version = "0.11", features = ["json"] }

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# 日志
log = "0.4"
env_logger = "0.11"

# 错误处理
anyhow = "1.0"
thiserror = "1.0"

# 实用工具
chrono = "0.4"
uuid = { version = "1.6", features = ["v4"] }
dirs = "5.0"

[build-dependencies]
cc = "1.0"

[dev-dependencies]
criterion = "0.5"
proptest = "1.4"
```

### 构建脚本 (sensevoice-desktop/build.rs)

```rust
fn main() {
    // 链接 SenseVoice.cpp 库
    println!("cargo:rustc-link-search=../build");
    println!("cargo:rustc-link-lib=static=sensevoice");

    // 平台特定链接
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=Accelerate");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=AppKit");
    }

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=dylib=ole32");
        println!("cargo:rustc-link-lib=dylib=user32");
    }

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=dylib=X11");
        println!("cargo:rustc-link-lib=dylib=Xtst");
    }
}
```

---

## 开发路线图

### Phase 1: 项目基础搭建（1-2 周）

- ✅ 创建项目结构
- ✅ 配置 gpui 依赖
- ✅ 创建 sensevoice-sys FFI 绑定
- ✅ 实现基础主窗口和导航
- ✅ 设置主题系统

### Phase 2: 核心功能实现（3-4 周）

**Week 1-2**: 文件转录功能
- ✅ 实现文件拖拽上传
- ✅ 集成 audio-processor 库
- ✅ 集成 SenseVoice 引擎
- ✅ 实现任务队列管理
- ✅ 显示转录进度

**Week 3-4**: 设置和配置
- ✅ 实现设置界面
- ✅ 配置持久化（TOML）
- ✅ 全局热键注册
- ✅ 模型管理

### Phase 3: 高级功能（3-4 周）

**Week 1**: 实时字幕
- ✅ 系统音频捕获（WASAPI/ScreenCaptureKit/PulseAudio）
- ✅ 流式转录
- ✅ 字幕悬浮窗

**Week 2**: 语音输入法
- ✅ 热键触发
- ✅ 麦克风录音
- ✅ 文本注入（enigo）

**Week 3**: LLM 集成
- ✅ LLM API 客户端
- ✅ 文本后处理
- ✅ 错误修正

**Week 4**: 历史记录
- ✅ 任务历史管理
- ✅ 搜索和过滤
- ✅ 批量导出

### Phase 4: 打磨和优化（2-3 周）

- ✅ 性能优化
- ✅ 内存管理
- ✅ 错误处理
- ✅ UI/UX 改进
- ✅ 国际化（i18n）
- ✅ 单元测试和集成测试

### Phase 5: 发布准备（1-2 周）

- ✅ 打包（macOS .app, Windows .exe, Linux AppImage）
- ✅ 代码签名
- ✅ 安装程序
- ✅ 文档编写
- ✅ 发布到 GitHub Releases

---

## 总结

本实现方案使用 **gpui** 作为 GUI 框架，结合已有的 **sensevoice-audio-processor** 音频处理库和 **SenseVoice.cpp** 语音识别引擎，构建一个高性能、跨平台的桌面应用。

### 核心优势

1. **纯 Rust 生态**: 类型安全，性能优秀
2. **GPU 加速**: 流畅的 UI 渲染
3. **模块化设计**: 易于维护和扩展
4. **跨平台支持**: Windows, macOS, Linux

### 下一步行动

1. 创建 `sensevoice-desktop` 项目结构
2. 配置 gpui 依赖并验证基础渲染
3. 实现 sensevoice-sys FFI 绑定
4. 按照路线图逐步实现功能
