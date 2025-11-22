//! 工具函数模块
//!
//! 提供各种辅助工具函数

use std::path::Path;

/// 格式化时间戳为 SRT 格式
///
/// # 示例
///
/// ```
/// # use sensevoice_desktop::utils::format_srt_timestamp;
/// let timestamp = format_srt_timestamp(65.5);
/// assert_eq!(timestamp, "00:01:05,500");
/// ```
pub fn format_srt_timestamp(seconds: f64) -> String {
    let hours = (seconds / 3600.0) as u32;
    let minutes = ((seconds % 3600.0) / 60.0) as u32;
    let secs = (seconds % 60.0) as u32;
    let millis = ((seconds % 1.0) * 1000.0) as u32;

    format!("{:02}:{:02}:{:02},{:03}", hours, minutes, secs, millis)
}

/// 格式化时间戳为 VTT 格式
pub fn format_vtt_timestamp(seconds: f64) -> String {
    let hours = (seconds / 3600.0) as u32;
    let minutes = ((seconds % 3600.0) / 60.0) as u32;
    let secs = (seconds % 60.0) as u32;
    let millis = ((seconds % 1.0) * 1000.0) as u32;

    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, secs, millis)
}

/// 获取文件扩展名
pub fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}

/// 检查是否为音频文件
pub fn is_audio_file(path: &Path) -> bool {
    matches!(
        get_file_extension(path).as_deref(),
        Some("mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a")
    )
}

/// 检查是否为视频文件
pub fn is_video_file(path: &Path) -> bool {
    matches!(
        get_file_extension(path).as_deref(),
        Some("mp4" | "avi" | "mkv" | "mov" | "flv" | "wmv")
    )
}

/// 格式化文件大小
pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// 格式化音频时长
pub fn format_duration(seconds: f64) -> String {
    let hours = (seconds / 3600.0) as u32;
    let minutes = ((seconds % 3600.0) / 60.0) as u32;
    let secs = (seconds % 60.0) as u32;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{:02}:{:02}", minutes, secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_srt_timestamp() {
        assert_eq!(format_srt_timestamp(0.0), "00:00:00,000");
        assert_eq!(format_srt_timestamp(65.5), "00:01:05,500");
        assert_eq!(format_srt_timestamp(3661.123), "01:01:01,123");
    }

    #[test]
    fn test_format_vtt_timestamp() {
        assert_eq!(format_vtt_timestamp(0.0), "00:00:00.000");
        assert_eq!(format_vtt_timestamp(65.5), "00:01:05.500");
    }

    #[test]
    fn test_is_audio_file() {
        assert!(is_audio_file(Path::new("test.mp3")));
        assert!(is_audio_file(Path::new("test.wav")));
        assert!(!is_audio_file(Path::new("test.mp4")));
        assert!(!is_audio_file(Path::new("test.txt")));
    }

    #[test]
    fn test_is_video_file() {
        assert!(is_video_file(Path::new("test.mp4")));
        assert!(is_video_file(Path::new("test.avi")));
        assert!(!is_video_file(Path::new("test.mp3")));
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(500), "500 B");
        assert_eq!(format_file_size(1024), "1.00 KB");
        assert_eq!(format_file_size(1_048_576), "1.00 MB");
        assert_eq!(format_file_size(1_073_741_824), "1.00 GB");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(30.0), "00:30");
        assert_eq!(format_duration(90.0), "01:30");
        assert_eq!(format_duration(3661.0), "01:01:01");
    }
}
