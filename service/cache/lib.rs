mod error;
pub mod memory;
mod service;

#[cfg(feature = "kv")]
pub mod kv;

#[cfg(feature = "mongodb")]
pub mod mongodb;

pub use error::{CacheError, CacheResult};
pub use memory::MemoryCacheService;
pub use service::CacheService;

#[cfg(feature = "kv")]
pub use kv::KvCacheService;

#[cfg(feature = "mongodb")]
pub use mongodb::MongoCacheService;
