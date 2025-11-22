//! 任务定义模块
//!
//! 定义转录任务的数据结构和状态

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// 转录任务
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TranscriptionTask {
    /// 唯一 ID
    pub id: String,
    /// 文件路径
    pub file_path: PathBuf,
    /// 任务状态
    pub status: TaskStatus,
    /// 进度 (0.0 - 1.0)
    pub progress: f32,
    /// 转录结果
    pub result: Option<TranscriptionResult>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 完成时间
    pub completed_at: Option<DateTime<Utc>>,
    /// 错误信息
    pub error: Option<String>,
}

/// 任务状态
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    /// 等待中
    Pending,
    /// 处理中
    Processing,
    /// 已完成
    Completed,
    /// 失败
    Failed,
    /// 已取消
    Cancelled,
}

/// 转录结果
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TranscriptionResult {
    /// 完整文本
    pub text: String,
    /// 带时间戳的片段
    pub segments: Vec<Segment>,
    /// 检测到的语言
    pub language: String,
    /// 处理时长（秒）
    pub duration: f64,
}

/// 转录片段
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Segment {
    /// 开始时间（秒）
    pub start: f64,
    /// 结束时间（秒）
    pub end: f64,
    /// 文本内容
    pub text: String,
}

impl TranscriptionTask {
    /// 创建新任务
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            file_path,
            status: TaskStatus::Pending,
            progress: 0.0,
            result: None,
            created_at: Utc::now(),
            completed_at: None,
            error: None,
        }
    }

    /// 开始处理
    pub fn start(&mut self) {
        self.status = TaskStatus::Processing;
        self.progress = 0.0;
    }

    /// 更新进度
    pub fn update_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }

    /// 完成任务
    pub fn complete(&mut self, result: TranscriptionResult) {
        self.status = TaskStatus::Completed;
        self.progress = 1.0;
        self.result = Some(result);
        self.completed_at = Some(Utc::now());
    }

    /// 任务失败
    pub fn fail(&mut self, error: String) {
        self.status = TaskStatus::Failed;
        self.error = Some(error);
        self.completed_at = Some(Utc::now());
    }

    /// 取消任务
    pub fn cancel(&mut self) {
        self.status = TaskStatus::Cancelled;
        self.completed_at = Some(Utc::now());
    }

    /// 是否已完成（包括成功、失败、取消）
    pub fn is_finished(&self) -> bool {
        matches!(
            self.status,
            TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled
        )
    }

    /// 获取文件名
    pub fn file_name(&self) -> String {
        self.file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string()
    }
}

impl TaskStatus {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "等待中",
            TaskStatus::Processing => "处理中",
            TaskStatus::Completed => "已完成",
            TaskStatus::Failed => "失败",
            TaskStatus::Cancelled => "已取消",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = TranscriptionTask::new(PathBuf::from("test.mp3"));
        assert_eq!(task.status, TaskStatus::Pending);
        assert_eq!(task.progress, 0.0);
        assert!(task.result.is_none());
    }

    #[test]
    fn test_task_lifecycle() {
        let mut task = TranscriptionTask::new(PathBuf::from("test.mp3"));

        // 开始
        task.start();
        assert_eq!(task.status, TaskStatus::Processing);

        // 更新进度
        task.update_progress(0.5);
        assert_eq!(task.progress, 0.5);

        // 完成
        let result = TranscriptionResult {
            text: "测试文本".to_string(),
            segments: vec![],
            language: "zh".to_string(),
            duration: 1.0,
        };
        task.complete(result);
        assert_eq!(task.status, TaskStatus::Completed);
        assert_eq!(task.progress, 1.0);
        assert!(task.result.is_some());
        assert!(task.is_finished());
    }

    #[test]
    fn test_task_failure() {
        let mut task = TranscriptionTask::new(PathBuf::from("test.mp3"));
        task.fail("Test error".to_string());

        assert_eq!(task.status, TaskStatus::Failed);
        assert!(task.is_finished());
        assert_eq!(task.error, Some("Test error".to_string()));
    }
}
