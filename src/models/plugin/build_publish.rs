use std::future::Future;
use std::pin::Pin;

use crate::error::Result;
use crate::traits::build_package::PackageOutput;

pub type AsyncResult<'a, T> = Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>;

#[derive(Debug, Clone)]
pub struct PublishTarget {
    pub registry: String,
    pub repository: Option<String>,
    pub tag: Option<String>,
    pub region: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PublishResult {
    pub success: bool,
    pub url: Option<String>,
    pub version: String,
    pub message: String,
    pub digest: Option<String>,
}

pub trait BuildPublish: Send + Sync {
    type Credentials;

    fn publisher(&self) -> &str;
    fn authenticate(&mut self, credentials: Self::Credentials) -> AsyncResult<'_, ()>;
    fn is_authenticated(&self) -> bool;

    fn publish(&self, package: &PackageOutput, target: &PublishTarget) -> AsyncResult<'_, PublishResult>;

    fn unpublish(&self, name: &str, version: &str, target: &PublishTarget) -> AsyncResult<'_, ()>;
    fn exists(&self, name: &str, version: &str, target: &PublishTarget) -> AsyncResult<'_, bool>;
    fn list_versions(&self, name: &str, target: &PublishTarget) -> AsyncResult<'_, Vec<String>>;
    fn latest_version(&self, name: &str, target: &PublishTarget) -> AsyncResult<'_, Option<String>>;

    fn download_url(&self, name: &str, version: &str, target: &PublishTarget) -> AsyncResult<'_, Option<String>>;
}
