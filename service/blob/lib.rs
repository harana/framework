mod error;
pub mod file;
mod service;

#[cfg(feature = "r2")]
pub mod r2;

pub use error::{BlobError, BlobResult};
pub use file::FileBlobService;
pub use service::BlobService;

#[cfg(feature = "r2")]
pub use r2::R2BlobService;
