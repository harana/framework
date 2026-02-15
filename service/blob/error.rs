use std::fmt;

#[derive(Debug, Clone)]
pub enum BlobError {
    AlreadyExists(String),
    BackendError(String),
    InvalidInput(String),
    IoError(String),
    NotFound(String),
    NotSupported(String),
    SerializationError(String),
}

impl fmt::Display for BlobError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyExists(key) => write!(f, "Blob '{key}' already exists"),
            Self::BackendError(msg) => write!(f, "Blob backend error: {msg}"),
            Self::InvalidInput(msg) => write!(f, "Blob invalid input: {msg}"),
            Self::IoError(msg) => write!(f, "Blob I/O error: {msg}"),
            Self::NotFound(key) => write!(f, "Blob '{key}' not found"),
            Self::NotSupported(msg) => write!(f, "Blob operation not supported: {msg}"),
            Self::SerializationError(msg) => write!(f, "Blob serialization error: {msg}"),
        }
    }
}

impl std::error::Error for BlobError {}

pub type BlobResult<T> = Result<T, BlobError>;
