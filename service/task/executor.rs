use async_trait::async_trait;
use serde_json::Value;

use crate::Task;

// ============================================================================
// Task Executor Trait
// ============================================================================

#[async_trait]
pub trait TaskExecutor: Send + Sync {
    async fn execute(&self, task: &Task) -> Result<Option<Value>, String>;
    fn can_handle(&self, task_type: &str) -> bool;
    fn task_types(&self) -> Vec<&str> { vec![] }
}

pub struct LoggingExecutor;

#[async_trait]
impl TaskExecutor for LoggingExecutor {
    async fn execute(&self, task: &Task) -> Result<Option<Value>, String> {
        tracing::info!(
            task_id = %task.id,
            task_name = %task.name,
            task_type = %task.task_type,
            queue = %task.queue,
            "Executing task"
        );
        Ok(Some(serde_json::json!({ "status": "logged" })))
    }

    fn can_handle(&self, _task_type: &str) -> bool {
        true
    }
}

pub struct FnExecutor<F>
where
    F: Fn(&Task) -> Result<Option<Value>, String> + Send + Sync,
{
    task_types: Vec<String>,
    handler: F,
}

impl<F> FnExecutor<F>
where
    F: Fn(&Task) -> Result<Option<Value>, String> + Send + Sync,
{
    pub fn new(task_types: Vec<String>, handler: F) -> Self {
        Self { task_types, handler }
    }

    pub fn for_all_types(handler: F) -> Self {
        Self {
            task_types: vec![],
            handler,
        }
    }
}

#[async_trait]
impl<F> TaskExecutor for FnExecutor<F>
where
    F: Fn(&Task) -> Result<Option<Value>, String> + Send + Sync,
{
    async fn execute(&self, task: &Task) -> Result<Option<Value>, String> {
        (self.handler)(task)
    }

    fn can_handle(&self, task_type: &str) -> bool {
        self.task_types.is_empty() || self.task_types.iter().any(|t| t == task_type)
    }

    fn task_types(&self) -> Vec<&str> {
        self.task_types.iter().map(|s| s.as_str()).collect()
    }
}

pub struct AsyncFnExecutor<F, Fut>
where
    F: Fn(Task) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<Option<Value>, String>> + Send,
{
    task_types: Vec<String>,
    handler: F,
}

impl<F, Fut> AsyncFnExecutor<F, Fut>
where
    F: Fn(Task) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<Option<Value>, String>> + Send,
{
    pub fn new(task_types: Vec<String>, handler: F) -> Self {
        Self { task_types, handler }
    }

    pub fn for_all_types(handler: F) -> Self {
        Self {
            task_types: vec![],
            handler,
        }
    }
}

#[async_trait]
impl<F, Fut> TaskExecutor for AsyncFnExecutor<F, Fut>
where
    F: Fn(Task) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<Option<Value>, String>> + Send,
{
    async fn execute(&self, task: &Task) -> Result<Option<Value>, String> {
        (self.handler)(task.clone()).await
    }

    fn can_handle(&self, task_type: &str) -> bool {
        self.task_types.is_empty() || self.task_types.iter().any(|t| t == task_type)
    }

    fn task_types(&self) -> Vec<&str> {
        self.task_types.iter().map(|s| s.as_str()).collect()
    }
}

pub struct CompositeExecutor {
    executors: Vec<Box<dyn TaskExecutor>>,
}

impl CompositeExecutor {
    pub fn new() -> Self {
        Self { executors: vec![] }
    }

    pub fn add<E: TaskExecutor + 'static>(mut self, executor: E) -> Self {
        self.executors.push(Box::new(executor));
        self
    }

    pub fn register<E: TaskExecutor + 'static>(&mut self, executor: E) {
        self.executors.push(Box::new(executor));
    }
}

impl Default for CompositeExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TaskExecutor for CompositeExecutor {
    async fn execute(&self, task: &Task) -> Result<Option<Value>, String> {
        for executor in &self.executors {
            if executor.can_handle(&task.task_type) {
                return executor.execute(task).await;
            }
        }
        Err(format!("No executor found for task type: {}", task.task_type))
    }

    fn can_handle(&self, task_type: &str) -> bool {
        self.executors.iter().any(|e| e.can_handle(task_type))
    }

    fn task_types(&self) -> Vec<&str> {
        self.executors.iter().flat_map(|e| e.task_types()).collect()
    }
}
