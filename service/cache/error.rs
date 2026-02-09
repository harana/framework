use std::fmt;

#[derive(Debug, Clone)]
pub enum CacheError {
    NotFound(String),
    ConnectionError(String),
    SerializationError(String),
    BackendError(String),
    NotSupported(String),
}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(key) => write!(f, "Cache key '{key}' not found"),
            Self::ConnectionError(msg) => write!(f, "Cache connection error: {msg}"),
            Self::SerializationError(msg) => write!(f, "Cache serialization error: {msg}"),
            Self::BackendError(msg) => write!(f, "Cache backend error: {msg}"),
            Self::NotSupported(msg) => write!(f, "Cache operation not supported: {msg}"),
        }
    }
}

impl std::error::Error for CacheError {}

pub type CacheResult<T> = Result<T, CacheError>;
