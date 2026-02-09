use std::fmt;

#[derive(Debug, Clone)]
pub enum BlobError {
    NotFound(String),
    AlreadyExists(String),
    IoError(String),
    SerializationError(String),
    BackendError(String),
    InvalidInput(String),
    NotSupported(String),
}

impl fmt::Display for BlobError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(key) => write!(f, "Blob '{key}' not found"),
            Self::AlreadyExists(key) => write!(f, "Blob '{key}' already exists"),
            Self::IoError(msg) => write!(f, "Blob I/O error: {msg}"),
            Self::SerializationError(msg) => write!(f, "Blob serialization error: {msg}"),
            Self::BackendError(msg) => write!(f, "Blob backend error: {msg}"),
            Self::InvalidInput(msg) => write!(f, "Blob invalid input: {msg}"),
            Self::NotSupported(msg) => write!(f, "Blob operation not supported: {msg}"),
        }
    }
}

impl std::error::Error for BlobError {}

pub type BlobResult<T> = Result<T, BlobError>;
