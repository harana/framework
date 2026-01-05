use std::future::Future;
use std::pin::Pin;

use crate::error::{BoxError, Result};

pub type AsyncResult<'a, T> = Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>;

pub trait Client: Send + Sync {
    type Config;
    type Response;

    fn new(config: Self::Config) -> Result<Self>
    where
        Self: Sized;

    fn connect(&mut self) -> AsyncResult<'_, ()>;
    fn disconnect(&mut self) -> AsyncResult<'_, ()>;
    fn is_connected(&self) -> bool;

    fn execute<R>(&self, request: R) -> AsyncResult<'_, Self::Response>
    where
        R: Send + 'static;
}
