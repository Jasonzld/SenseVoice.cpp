# SenseVoice Desktop

高性能语音识别桌面应用，基于 SenseVoice.cpp 和纯 Rust 技术栈。

## 功能特性

- 📁 **文件转录**: 支持音频/视频文件转录
- 🎤 **实时字幕**: 系统音频实时识别和字幕显示
- ⌨️ **语音输入法**: 语音转文字自动注入光标位置
- 📊 **历史记录**: 任务历史管理和搜索
- 🤖 **AI 修正**: 可选的 LLM 文本后处理
- 🎨 **现代 UI**: 基于 gpui 的高性能界面

## 技术栈

- **GUI**: gpui (计划中)
- **ASR Engine**: SenseVoice.cpp (C++ FFI)
- **Audio Processing**: sensevoice-audio-processor (Rust + SIMD)
- **Async Runtime**: Tokio
- **Platform Integration**: global-hotkey, tray-icon, enigo

## 项目结构

```
sensevoice-desktop/
├── src/
│   ├── main.rs              # 应用入口
│   ├── core/                # 业务逻辑层
│   │   ├── settings.rs      # 设置管理
│   │   ├── task.rs          # 任务定义
│   │   └── task_queue.rs    # 任务队列
│   ├── engine/              # 核心处理层
│   │   └── sensevoice.rs    # SenseVoice 引擎封装
│   ├── platform/            # 平台特定实现
│   │   ├── windows.rs       # Windows 实现
│   │   ├── macos.rs         # macOS 实现
│   │   └── linux.rs         # Linux 实现
│   └── utils/               # 工具函数
│       └── mod.rs           # 格式化、文件处理等
├── Cargo.toml
└── README.md
```

## 构建

### 前提条件

1. Rust 1.75+
2. SenseVoice.cpp 已编译（位于 `../build/`）

### 开发构建

```bash
cargo build
```

### 发布构建

```bash
cargo build --release
```

### 运行

```bash
cargo run
```

## 测试

```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test --package sensevoice-desktop --lib core::settings
```

## 配置

配置文件位置：
- **Windows**: `%APPDATA%\sensevoice\config.toml`
- **macOS**: `~/Library/Application Support/sensevoice/config.toml`
- **Linux**: `~/.config/sensevoice/config.toml`

示例配置：

```toml
[hotkeys]
record = "Ctrl+Shift+Space"
voice_input = "Ctrl+Alt+V"
realtime_subtitle = "Ctrl+Shift+S"

[model]
model_path = "/models/sensevoice-q4k.gguf"
model_type = "Q4K"
use_gpu = true
num_threads = 0  # 0 表示自动检测

[llm]
enabled = false
provider = "OpenAI"
api_endpoint = "https://api.openai.com/v1"
api_key = ""
model_name = "gpt-4-turbo"

[output]
output_dir = "~/Documents/SenseVoice/Output"
auto_open = true
formats = ["SRT", "TXT"]
include_timestamps = true
```

## 开发路线图

### Phase 1: 基础框架 ✅
- [x] 项目结构搭建
- [x] 核心数据模型
- [x] 设置管理
- [x] 任务队列

### Phase 2: SenseVoice 集成 (进行中)
- [ ] sensevoice-sys FFI 绑定
- [ ] 引擎封装和测试
- [ ] 音频预处理集成

### Phase 3: GUI 实现 (计划中)
- [ ] gpui 集成
- [ ] 主窗口和导航
- [ ] 文件转录界面
- [ ] 设置界面

### Phase 4: 高级功能 (计划中)
- [ ] 实时字幕
- [ ] 语音输入法
- [ ] LLM 集成
- [ ] 历史记录

### Phase 5: 打磨发布 (计划中)
- [ ] 性能优化
- [ ] UI/UX 改进
- [ ] 跨平台打包
- [ ] 文档完善

## 文档

- [实现方案](../docs/GPUI_IMPLEMENTATION_PLAN.md)
- [UI 原型设计](../docs/UI_PROTOTYPES_SUMMARY.md)
- [桌面应用设计](../docs/DESKTOP_APP_DESIGN.md)

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！
