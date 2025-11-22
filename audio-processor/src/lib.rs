//! SenseVoice 高性能音频处理库
//!
//! 特性：
//! - 纯 Rust 实现，内存安全
//! - SIMD 加速（AVX2/AVX-512/NEON）
//! - 支持多种音频格式（MP3/WAV/FLAC/AAC/OGG）
//! - 高质量音频重采样
//! - 并行处理支持

pub mod decoder;
pub mod encoder;
pub mod processor;
pub mod resampler;
pub mod simd;
pub mod types;

pub use decoder::AudioDecoder;
pub use encoder::AudioEncoder;
pub use processor::AudioProcessor;
pub use resampler::SimdResampler;
pub use types::*;

use anyhow::Result;
use std::path::Path;

/// 音频处理器配置
#[derive(Debug, Clone)]
pub struct AudioConfig {
    /// 目标采样率（Hz）
    pub target_sample_rate: u32,
    /// 目标声道数
    pub target_channels: u16,
    /// 是否启用 SIMD 加速
    pub use_simd: bool,
    /// 是否归一化音量
    pub normalize: bool,
    /// 目标峰值（0.0-1.0）
    pub target_peak: f32,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            target_sample_rate: 16000,
            target_channels: 1,
            use_simd: true,
            normalize: true,
            target_peak: 0.95,
        }
    }
}

/// 一站式音频处理：从文件到 16kHz 单声道 f32
///
/// # 参数
/// - `path`: 输入音频文件路径
/// - `config`: 音频处理配置
///
/// # 返回
/// - `Result<Vec<f32>>`: 处理后的音频数据
///
/// # 示例
/// ```rust
/// use sensevoice_audio_processor::{process_audio_file, AudioConfig};
///
/// let audio = process_audio_file("audio.mp3", &AudioConfig::default())?;
/// println!("处理了 {} 个样本", audio.len());
/// ```
pub fn process_audio_file<P: AsRef<Path>>(
    path: P,
    config: &AudioConfig,
) -> Result<Vec<f32>> {
    // 1. 解码音频文件
    let (samples, original_rate, channels) = AudioDecoder::decode_file(path)?;

    // 2. 创建处理器
    let processor = AudioProcessor::new(config.clone());

    // 3. 处理音频
    processor.process(&samples, original_rate, channels)
}

/// 批量处理音频文件
///
/// # 参数
/// - `paths`: 输入文件路径列表
/// - `config`: 音频处理配置
///
/// # 返回
/// - `Result<Vec<(String, Vec<f32>)>>`: (文件名, 音频数据) 列表
pub fn batch_process_audio_files<P: AsRef<Path>>(
    paths: &[P],
    config: &AudioConfig,
) -> Result<Vec<(String, Vec<f32>)>> {
    use rayon::prelude::*;

    paths
        .par_iter()
        .map(|path| {
            let path_ref = path.as_ref();
            let file_name = path_ref
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            let audio = process_audio_file(path_ref, config)?;
            Ok((file_name, audio))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AudioConfig::default();
        assert_eq!(config.target_sample_rate, 16000);
        assert_eq!(config.target_channels, 1);
        assert!(config.use_simd);
    }
}
