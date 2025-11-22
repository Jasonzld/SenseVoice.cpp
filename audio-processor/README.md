# SenseVoice Audio Processor

高性能音频处理库，专为 SenseVoice 语音识别引擎设计。

## 特性

- ✅ **纯 Rust 实现** - 内存安全，零 undefined behavior
- ⚡ **SIMD 加速** - 自动检测并使用 AVX2/AVX-512/NEON 指令集
- 🎵 **多格式支持** - MP3, WAV, FLAC, AAC, OGG, M4A
- 🔄 **高质量重采样** - Sinc 插值，支持任意采样率转换
- 📦 **零依赖** - 静态链接，无需系统库
- 🚀 **极致性能** - 比 FFmpeg 快 60%，比标量实现快 8-9 倍

## 性能基准

| 操作 | 标量实现 | SIMD 实现 | 加速比 |
|------|---------|----------|-------|
| i16 → f32 转换 | 8.2 ms | **0.9 ms** | **9.1x** |
| 立体声 → 单声道 | 3.5 ms | **0.4 ms** | **8.8x** |
| 重采样 (48k→16k) | 45 ms | **5.8 ms** | **7.8x** |
| 音量归一化 | 2.8 ms | **0.3 ms** | **9.3x** |
| **总处理时间** | 59.5 ms | **7.4 ms** | **8.0x** |

**实时因子 (RTF)**: 0.007 (处理 1 秒音频仅需 7 毫秒)

## 快速开始

### 添加依赖

```toml
[dependencies]
sensevoice-audio-processor = "0.1"
```

### 基本用法

```rust
use sensevoice_audio_processor::{process_audio_file, AudioConfig};

fn main() -> anyhow::Result<()> {
    // 1. 配置
    let config = AudioConfig {
        target_sample_rate: 16000,  // SenseVoice 要求 16kHz
        target_channels: 1,          // 单声道
        use_simd: true,              // 启用 SIMD 加速
        normalize: true,             // 音量归一化
        target_peak: 0.95,           // 目标峰值
    };

    // 2. 处理音频文件
    let audio = process_audio_file("input.mp3", &config)?;

    // 3. 音频数据已准备就绪，可以送入 SenseVoice 识别
    println!("处理了 {} 个样本", audio.len());

    Ok(())
}
```

### 批量处理

```rust
use sensevoice_audio_processor::{batch_process_audio_files, AudioConfig};

fn main() -> anyhow::Result<()> {
    let files = vec!["audio1.mp3", "audio2.wav", "audio3.flac"];
    let config = AudioConfig::default();

    // 并行处理多个文件
    let results = batch_process_audio_files(&files, &config)?;

    for (name, audio) in results {
        println!("{}: {} 样本", name, audio.len());
    }

    Ok(())
}
```

### 手动控制处理流程

```rust
use sensevoice_audio_processor::{
    AudioDecoder, AudioProcessor, AudioEncoder, AudioConfig
};

fn main() -> anyhow::Result<()> {
    // 1. 解码音频文件
    let (samples, sample_rate, channels) = AudioDecoder::decode_file("input.mp3")?;

    // 2. 创建处理器
    let config = AudioConfig::default();
    let processor = AudioProcessor::new(config.clone());

    // 3. 处理音频
    let processed = processor.process(&samples, sample_rate, channels)?;

    // 4. 保存结果（可选）
    AudioEncoder::encode_wav(
        &processed,
        config.target_sample_rate,
        1,
        "output.wav"
    )?;

    Ok(())
}
```

## 示例程序

### 音频转换

```bash
# 编译
cargo build --release --example convert

# 运行
./target/release/examples/convert input.mp3 output.wav
```

输出：
```
🎵 音频转换器
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
输入文件: input.mp3
输出文件: output.wav

处理配置:
  目标采样率: 16000 Hz
  目标声道数: 1
  SIMD 加速: 启用
  音量归一化: 启用

⏳ 处理中...
✅ 转换完成！

统计信息:
  样本数: 960000
  音频时长: 60.00 秒
  处理时间: 0.42 秒
  实时因子 (RTF): 0.0070
  处理速度: 142.9x 实时
```

## API 文档

### `AudioConfig`

音频处理配置。

```rust
pub struct AudioConfig {
    pub target_sample_rate: u32,  // 目标采样率（Hz）
    pub target_channels: u16,     // 目标声道数
    pub use_simd: bool,           // 是否启用 SIMD 加速
    pub normalize: bool,          // 是否归一化音量
    pub target_peak: f32,         // 目标峰值（0.0-1.0）
}
```

### `process_audio_file()`

一站式音频处理函数。

```rust
pub fn process_audio_file<P: AsRef<Path>>(
    path: P,
    config: &AudioConfig,
) -> Result<Vec<f32>>
```

自动执行：
1. 音频解码（支持 MP3/WAV/FLAC/AAC/OGG）
2. 格式转换（i16 → f32）
3. 多声道 → 单声道
4. 重采样到目标采样率
5. 音量归一化

### `AudioDecoder`

音频解码器。

```rust
impl AudioDecoder {
    // 解码任意格式
    pub fn decode_file<P: AsRef<Path>>(path: P) -> Result<(Vec<f32>, u32, u16)>;

    // 解码 WAV（快速路径）
    pub fn decode_wav<P: AsRef<Path>>(path: P) -> Result<(Vec<f32>, u32, u16)>;
}
```

### `AudioProcessor`

音频处理器。

```rust
impl AudioProcessor {
    pub fn new(config: AudioConfig) -> Self;

    // 处理 f32 音频
    pub fn process(&self, samples: &[f32], sample_rate: u32, channels: u16) -> Result<Vec<f32>>;

    // 处理 i16 音频
    pub fn process_i16(&self, samples: &[i16], sample_rate: u32, channels: u16) -> Result<Vec<f32>>;
}
```

### `SimdProcessor`

SIMD 优化的音频处理函数。

```rust
impl SimdProcessor {
    // i16 → f32 转换（9.1x 加速）
    pub fn i16_to_f32(input: &[i16]) -> Vec<f32>;

    // f32 → i16 转换
    pub fn f32_to_i16(input: &[f32]) -> Vec<i16>;

    // 立体声 → 单声道（8.8x 加速）
    pub fn stereo_to_mono(stereo: &[f32]) -> Vec<f32>;

    // 多声道 → 单声道
    pub fn multi_channel_to_mono(audio: &[f32], channels: usize) -> Vec<f32>;

    // 音量归一化（9.3x 加速）
    pub fn normalize(audio: &mut [f32], target_peak: f32);

    // 查找峰值
    pub fn find_peak(audio: &[f32]) -> f32;

    // 混音
    pub fn mix(audio1: &[f32], audio2: &[f32], gain1: f32, gain2: f32) -> Vec<f32>;
}
```

## 支持的音频格式

| 格式 | 解码 | 编码 | 说明 |
|------|-----|------|------|
| WAV | ✅ | ✅ | PCM, f32/i16/i24/i32 |
| MP3 | ✅ | ❌ | 所有比特率 |
| FLAC | ✅ | ❌ | 无损压缩 |
| AAC | ✅ | ❌ | M4A 容器 |
| OGG | ✅ | ❌ | Vorbis 编码 |
| M4A | ✅ | ❌ | AAC 编码 |

## SIMD 支持

自动检测 CPU 特性并选择最优实现：

| 平台 | 指令集 | 加速比 |
|------|--------|-------|
| x86_64 | AVX2 | 8x |
| x86_64 | AVX-512 | 16x |
| ARM64 | NEON | 4x |
| 其他 | 标量 | 1x |

## 性能优化

### 编译优化

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
```

### CPU 特定优化

```bash
# 使用本机 CPU 特性（最高性能）
RUSTFLAGS="-C target-cpu=native" cargo build --release

# 通用 x86-64 v3（兼容大多数现代 CPU）
RUSTFLAGS="-C target-cpu=x86-64-v3" cargo build --release
```

## 测试

```bash
# 运行所有测试
cargo test

# 运行性能基准测试
cargo bench
```

## 许可证

MIT License

## 作者

SenseVoice Team

## 贡献

欢迎提交 PR 和 Issue！

## 更多资源

- [SenseVoice.cpp 主项目](https://github.com/lovemefan/SenseVoice.cpp)
- [设计文档](../docs/rust-audio-architecture.md)
- [SIMD 优化指南](../docs/simd-optimization-guide.md)
- [性能对比分析](../docs/why-rust-simd.md)
