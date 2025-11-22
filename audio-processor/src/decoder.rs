//! 音频解码器 - 支持多种格式（MP3/WAV/FLAC/AAC/OGG）

use anyhow::{Context, Result};
use std::path::Path;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

pub struct AudioDecoder;

impl AudioDecoder {
    /// 解码音频文件
    ///
    /// # 返回
    /// (samples, sample_rate, channels)
    pub fn decode_file<P: AsRef<Path>>(path: P) -> Result<(Vec<f32>, u32, u16)> {
        let path = path.as_ref();

        // 1. 打开文件
        let file = std::fs::File::open(path)
            .with_context(|| format!("无法打开文件: {}", path.display()))?;

        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        // 2. 探测格式
        let mut hint = Hint::new();
        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                hint.with_extension(ext_str);
            }
        }

        let format_opts = FormatOptions::default();
        let metadata_opts = MetadataOptions::default();

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &format_opts, &metadata_opts)
            .context("无法探测音频格式")?;

        let mut format = probed.format;

        // 3. 获取默认音频轨
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
            .context("未找到音频轨")?;

        let track_id = track.id;
        let sample_rate = track
            .codec_params
            .sample_rate
            .context("无法获取采样率")?;
        let channels = track
            .codec_params
            .channels
            .context("无法获取声道信息")?;

        // 4. 创建解码器
        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &DecoderOptions::default())
            .context("无法创建解码器")?;

        // 5. 解码所有音频数据
        let mut samples = Vec::new();
        let mut sample_buf = None;

        loop {
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(_) => break,
            };

            if packet.track_id() != track_id {
                continue;
            }

            let decoded = decoder.decode(&packet).context("解码失败")?;

            // 初始化样本缓冲区
            if sample_buf.is_none() {
                let spec = *decoded.spec();
                let duration = decoded.capacity() as u64;
                sample_buf = Some(SampleBuffer::<f32>::new(duration, spec));
            }

            if let Some(ref mut buf) = sample_buf {
                buf.copy_interleaved_ref(decoded);
                samples.extend_from_slice(buf.samples());
            }
        }

        log::info!(
            "解码完成: {} - {} Hz, {} 声道, {} 样本",
            path.display(),
            sample_rate,
            channels.count(),
            samples.len()
        );

        Ok((samples, sample_rate, channels.count() as u16))
    }

    /// 解码 WAV 文件（快速路径）
    pub fn decode_wav<P: AsRef<Path>>(path: P) -> Result<(Vec<f32>, u32, u16)> {
        let mut reader = hound::WavReader::open(path.as_ref())
            .context("无法打开 WAV 文件")?;

        let spec = reader.spec();
        let sample_rate = spec.sample_rate;
        let channels = spec.channels;

        // 转换为 f32
        let samples: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Int => {
                let bits = spec.bits_per_sample;
                let max_value = (1 << (bits - 1)) as f32;

                reader
                    .samples::<i32>()
                    .map(|s| s.unwrap() as f32 / max_value)
                    .collect()
            }
            hound::SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap()).collect(),
        };

        log::info!(
            "解码 WAV: {} Hz, {} 声道, {} 样本",
            sample_rate,
            channels,
            samples.len()
        );

        Ok((samples, sample_rate, channels))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_wav() {
        // 创建一个测试 WAV 文件
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create("/tmp/test.wav", spec).unwrap();

        for i in 0..44100 {
            let sample = (i as f32 * 440.0 * 2.0 * std::f32::consts::PI / 44100.0).sin();
            let amplitude = i16::MAX as f32;
            writer.write_sample((sample * amplitude) as i16).unwrap();
            writer.write_sample((sample * amplitude) as i16).unwrap();
        }

        writer.finalize().unwrap();

        // 测试解码
        let (samples, rate, channels) = AudioDecoder::decode_wav("/tmp/test.wav").unwrap();

        assert_eq!(rate, 44100);
        assert_eq!(channels, 2);
        assert_eq!(samples.len(), 44100 * 2);

        // 清理
        std::fs::remove_file("/tmp/test.wav").ok();
    }
}
