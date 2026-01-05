use std::error::Error;
use std::fmt;

pub type BoxError = Box<dyn Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, BoxError>;

#[derive(Debug)]
pub struct PluginError {
    pub message: String,
    pub kind: PluginErrorKind,
    pub source: Option<BoxError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginErrorKind {
    AlreadyExists,
    Auth,
    Build,
    Config,
    Connection,
    Io,
    NotFound,
    Package,
    Parse,
    Publish,
    Timeout,
    Unknown,
    Unsupported,
    Validation,
}

impl PluginError {
    pub fn new(kind: PluginErrorKind, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            kind,
            source: None,
        }
    }

    pub fn with_source(
        kind: PluginErrorKind,
        message: impl Into<String>,
        source: impl Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            message: message.into(),
            kind,
            source: Some(Box::new(source)),
        }
    }
}

impl fmt::Display for PluginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl Error for PluginError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}
