mod error;
pub mod memory;
mod store;

#[cfg(feature = "kv")]
pub mod kv;

pub use error::{CacheError, CacheResult};
pub use memory::MemoryCacheStore;
pub use store::{CacheStore, GetOptions, KeyEntry, ListOptions, ListResponse, PutOptions};

#[cfg(feature = "kv")]
pub use kv::KvCacheStore;
