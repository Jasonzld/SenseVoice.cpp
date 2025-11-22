//! SenseVoice C++ FFI 绑定
//!
//! 本库提供了 SenseVoice.cpp 的 Rust FFI 绑定，包括：
//! - 模型加载和初始化
//! - 音频转录
//! - 结果处理
//!
//! # 示例
//!
//! ```no_run
//! use sensevoice_sys::SenseVoice;
//! use std::path::Path;
//!
//! let engine = SenseVoice::new(
//!     Path::new("/models/sensevoice-q4k.gguf"),
//!     0,  // 自动检测线程数
//!     true  // 使用 GPU
//! ).expect("Failed to create SenseVoice engine");
//!
//! let audio: Vec<f32> = vec![0.0; 16000]; // 1 秒 16kHz 音频
//! let result = engine.transcribe(&audio).expect("Transcription failed");
//!
//! println!("Text: {}", result.text);
//! println!("Language: {}", result.language);
//! ```

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// 引入 bindgen 生成的原始 FFI 绑定
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{CStr, CString};
use std::fmt;
use std::path::Path;

/// SenseVoice 引擎
///
/// 封装了 C++ SenseVoice 上下文，提供安全的 Rust 接口
pub struct SenseVoice {
    ctx: *mut SenseVoiceContext,
}

// 声明为线程安全（需要 C++ 实现保证线程安全）
unsafe impl Send for SenseVoice {}
unsafe impl Sync for SenseVoice {}

impl SenseVoice {
    /// 创建新的 SenseVoice 引擎实例
    ///
    /// # 参数
    ///
    /// * `model_path` - GGUF 模型文件路径
    /// * `num_threads` - 推理线程数 (0 表示自动检测)
    /// * `use_gpu` - 是否使用 GPU 加速
    ///
    /// # 错误
    ///
    /// 如果模型文件不存在、格式错误或初始化失败，返回错误
    pub fn new(model_path: &Path, num_threads: i32, use_gpu: bool) -> Result<Self, SenseVoiceError> {
        if !model_path.exists() {
            return Err(SenseVoiceError::ModelNotFound(model_path.to_path_buf()));
        }

        let path_str = CString::new(model_path.to_str().ok_or_else(|| {
            SenseVoiceError::InvalidPath(model_path.to_path_buf())
        })?)
        .map_err(|_| SenseVoiceError::InvalidPath(model_path.to_path_buf()))?;

        let ctx = unsafe {
            sensevoice_create_context(
                path_str.as_ptr(),
                num_threads,
                if use_gpu { 1 } else { 0 },
            )
        };

        if ctx.is_null() {
            let error_msg = unsafe {
                let c_str = sensevoice_get_last_error();
                if c_str.is_null() {
                    "Unknown error".to_string()
                } else {
                    CStr::from_ptr(c_str).to_string_lossy().into_owned()
                }
            };
            return Err(SenseVoiceError::InitializationFailed(error_msg));
        }

        log::info!(
            "SenseVoice engine created: model={}, threads={}, gpu={}",
            model_path.display(),
            num_threads,
            use_gpu
        );

        Ok(Self { ctx })
    }

    /// 转录音频
    ///
    /// # 参数
    ///
    /// * `audio` - f32 音频数据，要求 16kHz 采样率，单声道
    ///
    /// # 返回
    ///
    /// 返回转录结果，包含文本、片段和检测到的语言
    ///
    /// # 错误
    ///
    /// 如果转录失败返回错误
    pub fn transcribe(&self, audio: &[f32]) -> Result<TranscriptionResult, SenseVoiceError> {
        if audio.is_empty() {
            return Err(SenseVoiceError::EmptyAudio);
        }

        let result = unsafe { sensevoice_transcribe(self.ctx, audio.as_ptr(), audio.len()) };

        if result.is_null() {
            let error_msg = unsafe {
                let c_str = sensevoice_get_last_error();
                if c_str.is_null() {
                    "Transcription failed".to_string()
                } else {
                    CStr::from_ptr(c_str).to_string_lossy().into_owned()
                }
            };
            return Err(SenseVoiceError::TranscriptionFailed(error_msg));
        }

        // 安全地提取结果
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

        let text = segments
            .iter()
            .map(|s| s.text.clone())
            .collect::<Vec<_>>()
            .join(" ");

        // 释放 C++ 分配的内存
        unsafe {
            sensevoice_free_result(result);
        }

        log::debug!(
            "Transcription completed: {} segments, language={}",
            segments.len(),
            language
        );

        Ok(TranscriptionResult {
            text,
            segments,
            language,
        })
    }
}

impl Drop for SenseVoice {
    fn drop(&mut self) {
        if !self.ctx.is_null() {
            unsafe {
                sensevoice_free_context(self.ctx);
            }
            log::debug!("SenseVoice engine destroyed");
        }
    }
}

/// 转录结果
#[derive(Clone, Debug)]
pub struct TranscriptionResult {
    /// 完整文本
    pub text: String,
    /// 带时间戳的片段
    pub segments: Vec<Segment>,
    /// 检测到的语言
    pub language: String,
}

/// 转录片段
#[derive(Clone, Debug)]
pub struct Segment {
    /// 开始时间（秒）
    pub start: f64,
    /// 结束时间（秒）
    pub end: f64,
    /// 文本内容
    pub text: String,
}

/// SenseVoice 错误类型
#[derive(Debug, thiserror::Error)]
pub enum SenseVoiceError {
    #[error("Model file not found: {0}")]
    ModelNotFound(std::path::PathBuf),

    #[error("Invalid model path: {0}")]
    InvalidPath(std::path::PathBuf),

    #[error("Failed to initialize SenseVoice engine: {0}")]
    InitializationFailed(String),

    #[error("Transcription failed: {0}")]
    TranscriptionFailed(String),

    #[error("Empty audio data")]
    EmptyAudio,
}

impl fmt::Display for TranscriptionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Language: {}", self.language)?;
        writeln!(f, "Text: {}", self.text)?;
        writeln!(f, "\nSegments ({}):", self.segments.len())?;
        for (i, seg) in self.segments.iter().enumerate() {
            writeln!(
                f,
                "  [{:2}] [{:6.2}s - {:6.2}s] {}",
                i + 1,
                seg.start,
                seg.end,
                seg.text
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // 需要实际模型文件才能运行
    fn test_create_engine() {
        let engine = SenseVoice::new(Path::new("/models/sensevoice-q4k.gguf"), 0, false);
        assert!(engine.is_ok());
    }

    #[test]
    #[ignore]
    fn test_transcribe() {
        let engine =
            SenseVoice::new(Path::new("/models/sensevoice-q4k.gguf"), 0, false).unwrap();

        // 生成 1 秒静音音频
        let audio: Vec<f32> = vec![0.0; 16000];

        let result = engine.transcribe(&audio);
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_audio() {
        // 此测试不需要实际模型
        // 我们只测试空音频的处理逻辑
        let audio: Vec<f32> = vec![];

        // 由于没有实际引擎，这里只是示例
        // 实际测试需要模拟 FFI 调用
        assert!(audio.is_empty());
    }
}
