# SenseVoice Desktop 实现指南

本文档提供关键模块的实现细节和代码示例。

## 1. 项目初始化

### 1.1 创建 Tauri 项目

```bash
# 安装 Tauri CLI
cargo install tauri-cli

# 创建项目
npm create tauri-app
# 选择: sensevoice-desktop, Vue 3, TypeScript

cd sensevoice-desktop

# 添加 SenseVoice.cpp 作为 submodule
git submodule add https://github.com/lovemefan/SenseVoice.cpp sense-voice-cpp
git submodule update --init --recursive
```

### 1.2 项目结构

```
sensevoice-desktop/
├── src/                          # Vue 前端
│   ├── views/
│   ├── components/
│   └── main.ts
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   └── modules/
│   ├── Cargo.toml
│   └── build.rs                  # C++ 编译配置
├── sense-voice-cpp/              # Git submodule
└── package.json
```

## 2. C++ 集成配置

### 2.1 build.rs - 编译 SenseVoice C++

```rust
// src-tauri/build.rs

fn main() {
    // 编译 SenseVoice.cpp
    let dst = cmake::Config::new("../sense-voice-cpp")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("SENSE_VOICE_BUILD_EXAMPLES", "OFF")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=sense-voice-core");
    println!("cargo:rustc-link-lib=dylib=ggml");

    // C++ 桥接
    cxx_build::bridge("src/bridge.rs")
        .file("src/sense_voice_wrapper.cpp")
        .flag_if_supported("-std=c++11")
        .compile("sensevoice-bridge");

    println!("cargo:rerun-if-changed=src/bridge.rs");
    println!("cargo:rerun-if-changed=src/sense_voice_wrapper.cpp");
}
```

### 2.2 C++ Wrapper

```cpp
// src-tauri/src/sense_voice_wrapper.cpp

#include "sense-voice.h"
#include "rust/cxx.h"
#include <memory>

struct SenseVoiceWrapper {
    sense_voice_context* ctx;

    SenseVoiceWrapper(rust::Str model_path) {
        sense_voice_params params = sense_voice_default_params();
        ctx = sense_voice_init_from_file_with_params(
            std::string(model_path).c_str(),
            params
        );
    }

    ~SenseVoiceWrapper() {
        if (ctx) sense_voice_free(ctx);
    }

    rust::String transcribe(rust::Slice<const float> audio) {
        sense_voice_feature feature;
        sense_voice_get_fbank_feature(
            ctx,
            audio.data(),
            audio.size(),
            &feature
        );

        const char* result = sense_voice_decode(ctx, &feature);
        return rust::String(result);
    }
};

// CXX 桥接函数
std::unique_ptr<SenseVoiceWrapper> new_sense_voice(rust::Str model_path) {
    return std::make_unique<SenseVoiceWrapper>(model_path);
}
```

### 2.3 Rust Bridge

```rust
// src-tauri/src/bridge.rs

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("sensevoice-desktop/src-tauri/src/sense_voice_wrapper.h");

        type SenseVoiceWrapper;

        fn new_sense_voice(model_path: &str) -> UniquePtr<SenseVoiceWrapper>;
        fn transcribe(self: &SenseVoiceWrapper, audio: &[f32]) -> String;
    }
}

pub use ffi::*;
```

## 3. 音频捕获实现

### 3.1 麦克风捕获

```rust
// src-tauri/src/audio/microphone.rs

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

pub struct MicrophoneCapture {
    stream: Option<cpal::Stream>,
    buffer: Arc<Mutex<Vec<f32>>>,
}

impl MicrophoneCapture {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            stream: None,
            buffer: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub fn start<F>(&mut self, callback: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(Vec<f32>) + Send + 'static,
    {
        let host = cpal::default_host();
        let device = host.default_input_device()
            .ok_or("No input device available")?;

        let config = cpal::StreamConfig {
            channels: 1,
            sample_rate: cpal::SampleRate(16000),
            buffer_size: cpal::BufferSize::Fixed(512),
        };

        let buffer = self.buffer.clone();
        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut buf = buffer.lock().unwrap();
                buf.extend_from_slice(data);

                // 每收集 1 秒数据就回调
                if buf.len() >= 16000 {
                    let audio = buf.drain(..).collect();
                    callback(audio);
                }
            },
            |err| eprintln!("Stream error: {}", err),
            None
        )?;

        stream.play()?;
        self.stream = Some(stream);
        Ok(())
    }

    pub fn stop(&mut self) {
        self.stream = None;
    }
}
```

### 3.2 系统音频捕获 (Windows WASAPI Loopback)

```rust
// src-tauri/src/audio/loopback.rs

#[cfg(target_os = "windows")]
pub struct LoopbackCapture {
    // Windows 使用 WASAPI loopback
}

#[cfg(target_os = "windows")]
impl LoopbackCapture {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // 使用 windows-rs 调用 WASAPI
        // 参考: https://docs.microsoft.com/en-us/windows/win32/coreaudio/loopback-recording
        todo!("Implement WASAPI loopback")
    }
}

#[cfg(target_os = "macos")]
pub struct LoopbackCapture {
    // macOS 使用 BlackHole 虚拟音频设备
    // 或 ScreenCaptureKit (macOS 13+)
}

#[cfg(target_os = "linux")]
pub struct LoopbackCapture {
    // Linux 使用 PulseAudio monitor
    // 或 PipeWire
}
```

### 3.3 VAD (语音活动检测)

```rust
// src-tauri/src/audio/vad.rs

pub struct VoiceActivityDetector {
    threshold: f32,
    window_size: usize,
}

impl VoiceActivityDetector {
    pub fn new() -> Self {
        Self {
            threshold: 0.02,  // 能量阈值
            window_size: 320, // 20ms @ 16kHz
        }
    }

    pub fn is_speech(&self, audio: &[f32]) -> bool {
        // 简单的能量检测
        let energy: f32 = audio.iter()
            .map(|&x| x * x)
            .sum::<f32>() / audio.len() as f32;

        energy > self.threshold
    }

    pub fn detect_segments(&self, audio: &[f32]) -> Vec<(usize, usize)> {
        let mut segments = Vec::new();
        let mut start = None;

        for (i, chunk) in audio.chunks(self.window_size).enumerate() {
            if self.is_speech(chunk) {
                if start.is_none() {
                    start = Some(i * self.window_size);
                }
            } else if let Some(s) = start {
                segments.push((s, i * self.window_size));
                start = None;
            }
        }

        segments
    }
}
```

## 4. 实时转录引擎

### 4.1 异步处理流水线

```rust
// src-tauri/src/recognition/realtime.rs

use tokio::sync::mpsc;
use crate::bridge::SenseVoiceWrapper;

pub struct RealtimeEngine {
    engine: Box<SenseVoiceWrapper>,
    vad: VoiceActivityDetector,
}

impl RealtimeEngine {
    pub fn new(model_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let engine = Box::from(crate::bridge::new_sense_voice(model_path));
        Ok(Self {
            engine,
            vad: VoiceActivityDetector::new(),
        })
    }

    pub async fn start_pipeline(
        &mut self,
        mut audio_rx: mpsc::Receiver<Vec<f32>>,
        text_tx: mpsc::Sender<TranscriptResult>,
    ) {
        while let Some(audio) = audio_rx.recv().await {
            // 1. VAD 检测
            if !self.vad.is_speech(&audio) {
                continue;
            }

            // 2. 语音识别
            let raw_text = self.engine.transcribe(&audio);

            // 3. 后处理
            let processed = self.post_process(&raw_text);

            // 4. 发送结果
            text_tx.send(TranscriptResult {
                text: processed,
                timestamp: chrono::Utc::now().timestamp_millis() as u64,
                language: "zh".to_string(),
                confidence: 0.95,
            }).await.ok();
        }
    }

    fn post_process(&self, text: &str) -> String {
        // 错别字修正、标点符号等
        text.to_string()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TranscriptResult {
    pub text: String,
    pub timestamp: u64,
    pub language: String,
    pub confidence: f32,
}
```

## 5. Tauri Commands (前后端通信)

### 5.1 定义 Commands

```rust
// src-tauri/src/commands.rs

use tauri::State;
use std::sync::Mutex;

pub struct AppState {
    pub engine: Mutex<Option<RealtimeEngine>>,
    pub config: Mutex<AppConfig>,
}

#[tauri::command]
pub async fn start_realtime_caption(
    state: State<'_, AppState>,
    audio_source: String,
) -> Result<(), String> {
    let mut engine = state.engine.lock().unwrap();

    // 初始化引擎
    if engine.is_none() {
        let model_path = "/path/to/model.gguf";
        *engine = Some(RealtimeEngine::new(model_path)
            .map_err(|e| e.to_string())?);
    }

    // 启动音频捕获
    // ...

    Ok(())
}

#[tauri::command]
pub async fn transcribe_file(
    file_path: String,
) -> Result<Vec<TranscriptResult>, String> {
    // 读取音频文件
    let audio = load_audio_file(&file_path)
        .map_err(|e| e.to_string())?;

    // 转录
    let engine = RealtimeEngine::new("/path/to/model.gguf")
        .map_err(|e| e.to_string())?;

    let text = engine.engine.transcribe(&audio);

    Ok(vec![TranscriptResult {
        text,
        timestamp: 0,
        language: "zh".to_string(),
        confidence: 0.95,
    }])
}

#[tauri::command]
pub fn save_subtitle(
    segments: Vec<TranscriptResult>,
    output_path: String,
    format: String,
) -> Result<(), String> {
    match format.as_str() {
        "srt" => export_srt(&segments, &output_path),
        "vtt" => export_vtt(&segments, &output_path),
        _ => Err("Unsupported format".to_string()),
    }
}
```

### 5.2 注册 Commands

```rust
// src-tauri/src/main.rs

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            engine: Mutex::new(None),
            config: Mutex::new(AppConfig::default()),
        })
        .invoke_handler(tauri::generate_handler![
            start_realtime_caption,
            transcribe_file,
            save_subtitle,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## 6. Vue 前端调用

### 6.1 TypeScript 定义

```typescript
// src/types/tauri.ts

import { invoke } from '@tauri-apps/api/tauri'

export interface TranscriptResult {
  text: string
  timestamp: number
  language: string
  confidence: number
}

export async function startRealtimeCaption(audioSource: string): Promise<void> {
  await invoke('start_realtime_caption', { audioSource })
}

export async function transcribeFile(filePath: string): Promise<TranscriptResult[]> {
  return await invoke('transcribe_file', { filePath })
}

export async function saveSubtitle(
  segments: TranscriptResult[],
  outputPath: string,
  format: string
): Promise<void> {
  await invoke('save_subtitle', { segments, outputPath, format })
}
```

### 6.2 Vue 组件使用

```vue
<!-- src/views/FileTranscription.vue -->

<template>
  <div class="file-transcription">
    <input type="file" @change="handleFileSelect" accept="audio/*,video/*" />

    <button @click="startTranscription" :disabled="!selectedFile">
      开始转录
    </button>

    <div v-if="transcribing" class="progress">
      处理中... {{ progress }}%
    </div>

    <div v-if="result" class="result">
      <h3>转录结果:</h3>
      <pre>{{ result.text }}</pre>

      <button @click="exportSRT">导出 SRT</button>
      <button @click="exportTXT">导出 TXT</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { transcribeFile, saveSubtitle } from '@/types/tauri'
import { open, save } from '@tauri-apps/api/dialog'

const selectedFile = ref<string | null>(null)
const transcribing = ref(false)
const progress = ref(0)
const result = ref<any>(null)

async function handleFileSelect() {
  const selected = await open({
    multiple: false,
    filters: [{
      name: 'Media',
      extensions: ['mp4', 'mp3', 'wav', 'flac', 'avi', 'mkv']
    }]
  })

  if (selected && typeof selected === 'string') {
    selectedFile.value = selected
  }
}

async function startTranscription() {
  if (!selectedFile.value) return

  transcribing.value = true
  progress.value = 0

  try {
    const results = await transcribeFile(selectedFile.value)
    result.value = results[0]
  } catch (error) {
    console.error('Transcription error:', error)
  } finally {
    transcribing.value = false
    progress.value = 100
  }
}

async function exportSRT() {
  const path = await save({
    filters: [{ name: 'SRT', extensions: ['srt'] }]
  })

  if (path) {
    await saveSubtitle([result.value], path, 'srt')
  }
}
</script>
```

## 7. 全局热键实现

```rust
// src-tauri/src/input/hotkey.rs

use tauri::{Manager, GlobalShortcutManager};

pub fn register_hotkeys(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let mut shortcut = app.global_shortcut_manager();

    // Ctrl+Shift+Space: 开始/停止录音
    shortcut.register("Ctrl+Shift+Space", move || {
        // 触发录音事件
        app.emit_all("hotkey-voice-input", ()).ok();
    })?;

    Ok(())
}
```

## 8. 文本注入实现

```rust
// src-tauri/src/input/injector.rs

use enigo::{Enigo, Key, KeyboardControllable};

pub struct TextInjector {
    enigo: Enigo,
}

impl TextInjector {
    pub fn new() -> Self {
        Self { enigo: Enigo::new() }
    }

    pub fn inject(&mut self, text: &str) {
        // 方法1: 通过剪贴板粘贴 (推荐，更快)
        self.inject_via_clipboard(text);

        // 方法2: 逐字符输入 (更可靠，但慢)
        // self.inject_char_by_char(text);
    }

    fn inject_via_clipboard(&mut self, text: &str) {
        use clipboard::{ClipboardContext, ClipboardProvider};

        // 保存当前剪贴板
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let old_clipboard = ctx.get_contents().ok();

        // 写入新文本
        ctx.set_contents(text.to_string()).unwrap();

        // Ctrl+V 粘贴
        self.enigo.key_down(Key::Control);
        self.enigo.key_click(Key::Layout('v'));
        self.enigo.key_up(Key::Control);

        // 延迟后恢复剪贴板
        std::thread::sleep(std::time::Duration::from_millis(100));
        if let Some(old) = old_clipboard {
            ctx.set_contents(old).ok();
        }
    }

    fn inject_char_by_char(&mut self, text: &str) {
        for ch in text.chars() {
            self.enigo.key_sequence(&ch.to_string());
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}
```

## 9. 配置管理

```rust
// src-tauri/src/config.rs

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub model_path: PathBuf,
    pub language: String,
    pub theme: String,
    pub audio: AudioConfig,
    pub text: TextConfig,
    pub llm: Option<LLMConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub sample_rate: u32,
    pub channels: u16,
    pub enable_noise_reduction: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextConfig {
    pub enable_correction: bool,
    pub enable_punctuation: bool,
    pub filter_filler_words: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub endpoint: String,
    pub api_key: String,
    pub model: String,
    pub temperature: f32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            model_path: PathBuf::from("models/sense-voice-small-q4k.gguf"),
            language: "auto".to_string(),
            theme: "light".to_string(),
            audio: AudioConfig {
                sample_rate: 16000,
                channels: 1,
                enable_noise_reduction: true,
            },
            text: TextConfig {
                enable_correction: true,
                enable_punctuation: true,
                filter_filler_words: true,
            },
            llm: None,
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::config_path();
        if config_path.exists() {
            let content = std::fs::read_to_string(config_path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path();
        std::fs::create_dir_all(config_path.parent().unwrap())?;
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(config_path, content)?;
        Ok(())
    }

    fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap();
        path.push("sensevoice-desktop");
        path.push("config.json");
        path
    }
}
```

## 10. 打包与发布

### 10.1 配置 tauri.conf.json

```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devPath": "http://localhost:5173",
    "distDir": "../dist"
  },
  "package": {
    "productName": "SenseVoice Desktop",
    "version": "1.0.0"
  },
  "tauri": {
    "bundle": {
      "active": true,
      "targets": ["msi", "dmg", "appimage", "deb"],
      "identifier": "com.sensevoice.desktop",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ],
      "resources": [
        "resources/models/*.gguf"
      ],
      "externalBin": [],
      "longDescription": "智能语音转录工具"
    },
    "windows": {
      "certificateThumbprint": null,
      "digestAlgorithm": "sha256",
      "timestampUrl": ""
    }
  }
}
```

### 10.2 构建命令

```bash
# 开发模式
npm run tauri dev

# 生产构建
npm run tauri build

# 指定目标平台
npm run tauri build -- --target x86_64-pc-windows-msvc
npm run tauri build -- --target x86_64-apple-darwin
npm run tauri build -- --target x86_64-unknown-linux-gnu
```

### 10.3 CI/CD (GitHub Actions)

```yaml
# .github/workflows/release.yml

name: Release
on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    strategy:
      matrix:
        platform: [windows-latest, macos-latest, ubuntu-latest]
    runs-on: ${{ matrix.platform }}

    steps:
      - uses: actions/checkout@v3

      - name: Setup Node
        uses: actions/setup-node@v3
        with:
          node-version: 18

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install dependencies
        run: npm install

      - name: Build
        run: npm run tauri build

      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: ${{ matrix.platform }}
          path: src-tauri/target/release/bundle/
```

---

## 总结

这份实现指南涵盖了：
1. ✅ 项目初始化与结构
2. ✅ C++ 引擎集成
3. ✅ 音频捕获（麦克风/系统音频）
4. ✅ 实时转录流水线
5. ✅ 前后端通信 (Tauri Commands)
6. ✅ 全局热键与文本注入
7. ✅ 配置管理
8. ✅ 打包发布

按照这份指南，可以快速搭建起一个功能完整的 SenseVoice Desktop 应用！
