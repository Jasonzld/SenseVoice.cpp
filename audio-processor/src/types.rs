//! 类型定义

use serde::{Deserialize, Serialize};

/// 音频格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioFormat {
    /// WAV
    Wav,
    /// MP3
    Mp3,
    /// FLAC
    Flac,
    /// AAC
    Aac,
    /// OGG Vorbis
    Ogg,
    /// M4A
    M4a,
    /// 未知格式
    Unknown,
}

impl AudioFormat {
    /// 从文件扩展名推断格式
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "wav" => Self::Wav,
            "mp3" => Self::Mp3,
            "flac" => Self::Flac,
            "aac" => Self::Aac,
            "ogg" => Self::Ogg,
            "m4a" => Self::M4a,
            _ => Self::Unknown,
        }
    }
}

/// 音频元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioMetadata {
    /// 采样率（Hz）
    pub sample_rate: u32,
    /// 声道数
    pub channels: u16,
    /// 样本数
    pub samples: usize,
    /// 时长（秒）
    pub duration: f64,
    /// 比特深度
    pub bits_per_sample: Option<u16>,
    /// 格式
    pub format: AudioFormat,
}

impl AudioMetadata {
    /// 计算时长
    pub fn calculate_duration(samples: usize, sample_rate: u32, channels: u16) -> f64 {
        samples as f64 / (sample_rate as f64 * channels as f64)
    }
}

/// 音频数据
#[derive(Debug, Clone)]
pub struct AudioData {
    /// 样本数据（交错格式）
    pub samples: Vec<f32>,
    /// 元数据
    pub metadata: AudioMetadata,
}

impl AudioData {
    /// 创建新的音频数据
    pub fn new(samples: Vec<f32>, sample_rate: u32, channels: u16) -> Self {
        let duration =
            AudioMetadata::calculate_duration(samples.len(), sample_rate, channels);

        Self {
            samples: samples.clone(),
            metadata: AudioMetadata {
                sample_rate,
                channels,
                samples: samples.len(),
                duration,
                bits_per_sample: None,
                format: AudioFormat::Unknown,
            },
        }
    }

    /// 获取单声道样本
    pub fn get_mono(&self) -> Vec<f32> {
        if self.metadata.channels == 1 {
            return self.samples.clone();
        }

        crate::simd::SimdProcessor::multi_channel_to_mono(
            &self.samples,
            self.metadata.channels as usize,
        )
    }
}

/// 处理进度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessProgress {
    /// 当前处理的文件
    pub file_name: String,
    /// 进度百分比（0.0-1.0）
    pub progress: f32,
    /// 状态消息
    pub message: String,
}

/// 处理结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessResult {
    /// 文件名
    pub file_name: String,
    /// 是否成功
    pub success: bool,
    /// 输出文件路径
    pub output_path: Option<String>,
    /// 错误消息
    pub error: Option<String>,
    /// 处理时间（秒）
    pub process_time: f64,
    /// 音频时长（秒）
    pub audio_duration: f64,
    /// 实时因子（RTF）
    pub rtf: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_format() {
        assert_eq!(AudioFormat::from_extension("wav"), AudioFormat::Wav);
        assert_eq!(AudioFormat::from_extension("MP3"), AudioFormat::Mp3);
        assert_eq!(AudioFormat::from_extension("unknown"), AudioFormat::Unknown);
    }

    #[test]
    fn test_audio_metadata() {
        let duration = AudioMetadata::calculate_duration(44100, 44100, 1);
        assert!((duration - 1.0).abs() < 0.001);

        let duration_stereo = AudioMetadata::calculate_duration(88200, 44100, 2);
        assert!((duration_stereo - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_audio_data() {
        let samples = vec![0.1, 0.2, 0.3, 0.4];
        let data = AudioData::new(samples.clone(), 16000, 1);

        assert_eq!(data.samples, samples);
        assert_eq!(data.metadata.sample_rate, 16000);
        assert_eq!(data.metadata.channels, 1);
    }
}
