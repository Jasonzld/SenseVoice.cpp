//! 任务队列管理模块
//!
//! 管理转录任务队列，支持并发处理

use super::task::{TranscriptionTask, TaskStatus};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 任务队列管理器
pub struct TaskQueue {
    /// 任务列表
    tasks: Arc<RwLock<VecDeque<TranscriptionTask>>>,
    /// 最大并发数
    max_concurrent: usize,
}

impl TaskQueue {
    /// 创建新的任务队列
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(VecDeque::new())),
            max_concurrent: 2, // 默认同时处理 2 个任务
        }
    }

    /// 创建指定并发数的任务队列
    pub fn with_max_concurrent(max_concurrent: usize) -> Self {
        Self {
            tasks: Arc::new(RwLock::new(VecDeque::new())),
            max_concurrent: max_concurrent.max(1),
        }
    }

    /// 添加任务
    pub async fn add_task(&self, task: TranscriptionTask) {
        let mut tasks = self.tasks.write().await;
        tasks.push_back(task);
    }

    /// 获取所有任务
    pub async fn get_all_tasks(&self) -> Vec<TranscriptionTask> {
        let tasks = self.tasks.read().await;
        tasks.iter().cloned().collect()
    }

    /// 获取待处理任务数
    pub async fn pending_count(&self) -> usize {
        let tasks = self.tasks.read().await;
        tasks
            .iter()
            .filter(|t| matches!(t.status, TaskStatus::Pending))
            .count()
    }

    /// 获取正在处理的任务数
    pub async fn processing_count(&self) -> usize {
        let tasks = self.tasks.read().await;
        tasks
            .iter()
            .filter(|t| matches!(t.status, TaskStatus::Processing))
            .count()
    }

    /// 获取已完成任务数
    pub async fn completed_count(&self) -> usize {
        let tasks = self.tasks.read().await;
        tasks
            .iter()
            .filter(|t| matches!(t.status, TaskStatus::Completed))
            .count()
    }

    /// 按 ID 查找任务
    pub async fn find_task(&self, id: &str) -> Option<TranscriptionTask> {
        let tasks = self.tasks.read().await;
        tasks.iter().find(|t| t.id == id).cloned()
    }

    /// 更新任务
    pub async fn update_task(&self, updated_task: TranscriptionTask) {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.iter_mut().find(|t| t.id == updated_task.id) {
            *task = updated_task;
        }
    }

    /// 取消任务
    pub async fn cancel_task(&self, id: &str) -> bool {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            if !task.is_finished() {
                task.cancel();
                return true;
            }
        }
        false
    }

    /// 清除已完成的任务
    pub async fn clear_completed(&self) {
        let mut tasks = self.tasks.write().await;
        tasks.retain(|t| !t.is_finished());
    }

    /// 清除所有任务
    pub async fn clear_all(&self) {
        let mut tasks = self.tasks.write().await;
        tasks.clear();
    }
}

impl Default for TaskQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_task_queue() {
        let queue = TaskQueue::new();

        // 添加任务
        let task = TranscriptionTask::new(PathBuf::from("test.mp3"));
        queue.add_task(task.clone()).await;

        assert_eq!(queue.pending_count().await, 1);
        assert_eq!(queue.get_all_tasks().await.len(), 1);

        // 查找任务
        let found = queue.find_task(&task.id).await;
        assert!(found.is_some());
    }

    #[tokio::test]
    async fn test_cancel_task() {
        let queue = TaskQueue::new();
        let task = TranscriptionTask::new(PathBuf::from("test.mp3"));
        let task_id = task.id.clone();

        queue.add_task(task).await;

        // 取消任务
        let cancelled = queue.cancel_task(&task_id).await;
        assert!(cancelled);

        // 验证状态
        let task = queue.find_task(&task_id).await.unwrap();
        assert_eq!(task.status, TaskStatus::Cancelled);
    }

    #[tokio::test]
    async fn test_clear_completed() {
        let queue = TaskQueue::new();

        let mut task1 = TranscriptionTask::new(PathBuf::from("test1.mp3"));
        task1.cancel();

        let task2 = TranscriptionTask::new(PathBuf::from("test2.mp3"));

        queue.add_task(task1).await;
        queue.add_task(task2).await;

        assert_eq!(queue.get_all_tasks().await.len(), 2);

        queue.clear_completed().await;

        assert_eq!(queue.get_all_tasks().await.len(), 1);
    }
}
