//! 音频处理器 - 整合所有处理步骤

use crate::{simd::SimdProcessor, resampler::SimdResampler, AudioConfig};
use anyhow::Result;

/// 音频处理器
pub struct AudioProcessor {
    config: AudioConfig,
}

impl AudioProcessor {
    /// 创建音频处理器
    pub fn new(config: AudioConfig) -> Self {
        Self { config }
    }

    /// 完整的音频处理流程
    ///
    /// 步骤：
    /// 1. 格式转换（i16 → f32 如果需要）
    /// 2. 多声道 → 单声道
    /// 3. 重采样到目标采样率
    /// 4. 音量归一化
    ///
    /// # 参数
    /// - `samples`: 输入音频样本（交错格式）
    /// - `sample_rate`: 输入采样率
    /// - `channels`: 输入声道数
    ///
    /// # 返回
    /// 处理后的音频数据（单声道，目标采样率）
    pub fn process(
        &self,
        samples: &[f32],
        sample_rate: u32,
        channels: u16,
    ) -> Result<Vec<f32>> {
        log::info!(
            "开始处理音频: {} Hz, {} 声道, {} 样本",
            sample_rate,
            channels,
            samples.len()
        );

        let start = std::time::Instant::now();

        // 1. 转换为单声道
        let mono = if channels > 1 {
            if self.config.use_simd {
                SimdProcessor::multi_channel_to_mono(samples, channels as usize)
            } else {
                self.multi_channel_to_mono_scalar(samples, channels as usize)
            }
        } else {
            samples.to_vec()
        };

        log::debug!("单声道转换完成: {} 样本", mono.len());

        // 2. 重采样
        let resampled = if sample_rate != self.config.target_sample_rate {
            let mut resampler = SimdResampler::new(sample_rate, self.config.target_sample_rate)?;
            resampler.resample(&mono)?
        } else {
            mono
        };

        log::debug!(
            "重采样完成: {} Hz → {} Hz, {} 样本",
            sample_rate,
            self.config.target_sample_rate,
            resampled.len()
        );

        // 3. 归一化
        let mut output = resampled;
        if self.config.normalize {
            if self.config.use_simd {
                SimdProcessor::normalize(&mut output, self.config.target_peak);
            } else {
                self.normalize_scalar(&mut output, self.config.target_peak);
            }
        }

        let elapsed = start.elapsed();
        let audio_duration = output.len() as f64 / self.config.target_sample_rate as f64;
        let rtf = elapsed.as_secs_f64() / audio_duration;

        log::info!(
            "音频处理完成: {} 样本, 耗时 {:.2}ms, RTF = {:.4}",
            output.len(),
            elapsed.as_secs_f64() * 1000.0,
            rtf
        );

        Ok(output)
    }

    /// i16 → f32 并处理
    pub fn process_i16(
        &self,
        samples: &[i16],
        sample_rate: u32,
        channels: u16,
    ) -> Result<Vec<f32>> {
        // 转换为 f32
        let f32_samples = if self.config.use_simd {
            SimdProcessor::i16_to_f32(samples)
        } else {
            samples.iter().map(|&x| x as f32 / 32768.0).collect()
        };

        self.process(&f32_samples, sample_rate, channels)
    }

    /// 标量实现：多声道 → 单声道
    fn multi_channel_to_mono_scalar(&self, audio: &[f32], channels: usize) -> Vec<f32> {
        if channels == 1 {
            return audio.to_vec();
        }

        let output_len = audio.len() / channels;
        let mut mono = Vec::with_capacity(output_len);

        for chunk in audio.chunks_exact(channels) {
            let sum: f32 = chunk.iter().sum();
            mono.push(sum / channels as f32);
        }

        mono
    }

    /// 标量实现：归一化
    fn normalize_scalar(&self, audio: &mut [f32], target_peak: f32) {
        let peak = audio.iter().map(|x| x.abs()).fold(0.0f32, f32::max);

        if peak == 0.0 || peak == target_peak {
            return;
        }

        let scale = target_peak / peak;
        for sample in audio.iter_mut() {
            *sample *= scale;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_mono() {
        let config = AudioConfig {
            target_sample_rate: 16000,
            target_channels: 1,
            use_simd: true,
            normalize: true,
            target_peak: 0.95,
        };

        let processor = AudioProcessor::new(config);

        // 生成 1 秒 44.1kHz 单声道测试音频
        let input: Vec<f32> = (0..44100)
            .map(|i| (i as f32 * 440.0 * 2.0 * std::f32::consts::PI / 44100.0).sin() * 0.5)
            .collect();

        let output = processor.process(&input, 44100, 1).unwrap();

        // 应该输出 16kHz 音频
        assert!((output.len() as i32 - 16000).abs() < 100);

        // 应该归一化到接近 0.95
        let peak = output.iter().map(|x| x.abs()).fold(0.0f32, f32::max);
        assert!((peak - 0.95).abs() < 0.01);
    }

    #[test]
    fn test_process_stereo() {
        let config = AudioConfig::default();
        let processor = AudioProcessor::new(config);

        // 生成立体声测试音频
        let mut input = Vec::new();
        for i in 0..44100 {
            let sample = (i as f32 * 440.0 * 2.0 * std::f32::consts::PI / 44100.0).sin();
            input.push(sample); // 左声道
            input.push(sample * 0.5); // 右声道
        }

        let output = processor.process(&input, 44100, 2).unwrap();

        // 应该输出 16kHz 单声道
        assert!((output.len() as i32 - 16000).abs() < 100);
    }

    #[test]
    fn test_process_i16() {
        let config = AudioConfig::default();
        let processor = AudioProcessor::new(config);

        // 生成 i16 测试音频
        let input: Vec<i16> = (0..44100)
            .map(|i| {
                let sample = (i as f32 * 440.0 * 2.0 * std::f32::consts::PI / 44100.0).sin();
                (sample * 16384.0) as i16
            })
            .collect();

        let output = processor.process_i16(&input, 44100, 1).unwrap();

        assert!((output.len() as i32 - 16000).abs() < 100);
    }
}
