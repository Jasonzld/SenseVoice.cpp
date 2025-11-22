//! 音频编码器 - WAV 写入

use anyhow::Result;
use std::path::Path;

pub struct AudioEncoder;

impl AudioEncoder {
    /// 编码为 WAV 文件
    ///
    /// # 参数
    /// - `samples`: 音频样本（f32 格式）
    /// - `sample_rate`: 采样率
    /// - `channels`: 声道数
    /// - `output_path`: 输出文件路径
    pub fn encode_wav<P: AsRef<Path>>(
        samples: &[f32],
        sample_rate: u32,
        channels: u16,
        output_path: P,
    ) -> Result<()> {
        let spec = hound::WavSpec {
            channels,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(output_path.as_ref(), spec)?;

        // f32 → i16
        for &sample in samples {
            let sample_i16 = (sample * 32767.0).clamp(-32768.0, 32767.0) as i16;
            writer.write_sample(sample_i16)?;
        }

        writer.finalize()?;

        log::info!(
            "WAV 编码完成: {} - {} Hz, {} 声道, {} 样本",
            output_path.as_ref().display(),
            sample_rate,
            channels,
            samples.len()
        );

        Ok(())
    }

    /// 编码为 WAV（f32 格式）
    pub fn encode_wav_f32<P: AsRef<Path>>(
        samples: &[f32],
        sample_rate: u32,
        channels: u16,
        output_path: P,
    ) -> Result<()> {
        let spec = hound::WavSpec {
            channels,
            sample_rate,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };

        let mut writer = hound::WavWriter::create(output_path.as_ref(), spec)?;

        for &sample in samples {
            writer.write_sample(sample)?;
        }

        writer.finalize()?;

        log::info!(
            "WAV (f32) 编码完成: {} - {} Hz, {} 声道, {} 样本",
            output_path.as_ref().display(),
            sample_rate,
            channels,
            samples.len()
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoder::AudioDecoder;

    #[test]
    fn test_encode_decode_wav() {
        // 生成测试音频
        let samples: Vec<f32> = (0..16000)
            .map(|i| (i as f32 * 440.0 * 2.0 * std::f32::consts::PI / 16000.0).sin())
            .collect();

        let path = "/tmp/test_encode.wav";

        // 编码
        AudioEncoder::encode_wav(&samples, 16000, 1, path).unwrap();

        // 解码验证
        let (decoded, rate, channels) = AudioDecoder::decode_wav(path).unwrap();

        assert_eq!(rate, 16000);
        assert_eq!(channels, 1);
        assert_eq!(decoded.len(), samples.len());

        // 验证数据正确性（允许量化误差）
        for (i, (&original, &decoded_sample)) in
            samples.iter().zip(decoded.iter()).enumerate().take(100)
        {
            assert!(
                (original - decoded_sample).abs() < 0.01,
                "Sample {} mismatch: {} vs {}",
                i,
                original,
                decoded_sample
            );
        }

        // 清理
        std::fs::remove_file(path).ok();
    }
}
