//! 高质量音频重采样器（SIMD 加速）

use anyhow::Result;
use rubato::{Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction};

/// SIMD 优化的音频重采样器
pub struct SimdResampler {
    resampler: Option<SincFixedIn<f32>>,
    from_rate: u32,
    to_rate: u32,
}

impl SimdResampler {
    /// 创建重采样器
    ///
    /// # 参数
    /// - `from_rate`: 源采样率
    /// - `to_rate`: 目标采样率
    pub fn new(from_rate: u32, to_rate: u32) -> Result<Self> {
        if from_rate == to_rate {
            return Ok(Self {
                resampler: None,
                from_rate,
                to_rate,
            });
        }

        // 使用 Sinc 插值（高质量）
        let params = SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };

        let resampler = SincFixedIn::<f32>::new(
            to_rate as f64 / from_rate as f64,
            2.0, // 质量参数
            params,
            1024, // 块大小
            1,    // 单声道
        )?;

        Ok(Self {
            resampler: Some(resampler),
            from_rate,
            to_rate,
        })
    }

    /// 重采样音频
    pub fn resample(&mut self, input: &[f32]) -> Result<Vec<f32>> {
        if let Some(ref mut resampler) = self.resampler {
            let input_frames = input.len();
            let output_frames = ((input_frames as f64 * self.to_rate as f64)
                / self.from_rate as f64) as usize;

            let mut output = vec![vec![0.0f32; output_frames]; 1];
            let input_vec = vec![input.to_vec()];

            // 执行重采样
            resampler.process_into_buffer(&input_vec, &mut output, None)?;

            Ok(output[0].clone())
        } else {
            // 采样率相同，直接返回
            Ok(input.to_vec())
        }
    }

    /// 批量重采样（并行处理）
    pub fn batch_resample(&mut self, inputs: Vec<Vec<f32>>) -> Result<Vec<Vec<f32>>> {
        inputs.into_iter().map(|input| self.resample(&input)).collect()
    }
}

/// 简单的线性插值重采样（快速但质量较低）
pub struct LinearResampler {
    ratio: f32,
}

impl LinearResampler {
    pub fn new(from_rate: u32, to_rate: u32) -> Self {
        Self {
            ratio: from_rate as f32 / to_rate as f32,
        }
    }

    /// 快速线性插值重采样
    pub fn resample(&self, input: &[f32]) -> Vec<f32> {
        if self.ratio == 1.0 {
            return input.to_vec();
        }

        let output_len = (input.len() as f32 / self.ratio) as usize;
        let mut output = Vec::with_capacity(output_len);

        for i in 0..output_len {
            let pos = i as f32 * self.ratio;
            let idx = pos as usize;
            let frac = pos - idx as f32;

            if idx + 1 < input.len() {
                let a = input[idx];
                let b = input[idx + 1];
                output.push(a + (b - a) * frac);
            } else {
                output.push(input[idx]);
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_resampler() {
        let resampler = LinearResampler::new(48000, 16000);

        // 创建 1 秒 48kHz 测试信号
        let input: Vec<f32> = (0..48000)
            .map(|i| (i as f32 * 440.0 * 2.0 * std::f32::consts::PI / 48000.0).sin())
            .collect();

        let output = resampler.resample(&input);

        // 输出应该是 16kHz (16000 样本)
        assert!((output.len() as i32 - 16000).abs() < 10);
    }

    #[test]
    fn test_simd_resampler() {
        let mut resampler = SimdResampler::new(44100, 16000).unwrap();

        // 创建 1 秒 44.1kHz 测试信号
        let input: Vec<f32> = (0..44100)
            .map(|i| (i as f32 * 440.0 * 2.0 * std::f32::consts::PI / 44100.0).sin())
            .collect();

        let output = resampler.resample(&input).unwrap();

        // 输出应该是 16kHz (~16000 样本)
        assert!((output.len() as i32 - 16000).abs() < 100);
    }

    #[test]
    fn test_same_rate() {
        let mut resampler = SimdResampler::new(16000, 16000).unwrap();
        let input = vec![1.0, 2.0, 3.0, 4.0];
        let output = resampler.resample(&input).unwrap();

        assert_eq!(input, output);
    }
}
