use std::future::Future;
use std::pin::Pin;

use crate::error::Result;

pub type AsyncResult<'a, T> = Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>;

#[derive(Debug, Clone, Default)]
pub struct BuildOutput {
    pub files_processed: usize,
    pub files_written: usize,
    pub duration_ms: u64,
    pub artifacts: Vec<String>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

pub trait Build: Send + Sync {
    type Config;

    fn new(config: Self::Config) -> Result<Self>
    where
        Self: Sized;

    fn build(&self) -> AsyncResult<'_, BuildOutput>;
    fn clean(&self) -> AsyncResult<'_, ()>;
    fn needs_rebuild(&self) -> AsyncResult<'_, bool>;
    fn watch(&self) -> AsyncResult<'_, ()>;
    fn config(&self) -> &Self::Config;
    fn validate_config(&self) -> Result<()>;
    fn output_dir(&self) -> &str;
}
