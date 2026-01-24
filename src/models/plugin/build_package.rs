use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use crate::error::Result;
use crate::traits::build::BuildOutput;

pub type AsyncResult<'a, T> = Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>;

#[derive(Debug, Clone, Default)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub license: Option<String>,
    pub dependencies: HashMap<String, String>,
    pub extra: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct PackageOutput {
    pub path: String,
    pub size_bytes: u64,
    pub checksum: String,
    pub metadata: PackageMetadata,
    pub format: String,
}

pub trait BuildPackage: Send + Sync {
    type Format;

    fn format(&self) -> &str;
    fn extensions(&self) -> &[&str];

    fn package(&self, build_output: &BuildOutput, metadata: PackageMetadata) -> AsyncResult<'_, PackageOutput>;

    fn extract(&self, package_path: &str, output_dir: &str) -> AsyncResult<'_, ()>;
    fn verify(&self, package_path: &str) -> AsyncResult<'_, bool>;
    fn read_metadata(&self, package_path: &str) -> AsyncResult<'_, PackageMetadata>;
    fn list_contents(&self, package_path: &str) -> AsyncResult<'_, Vec<String>>;
    fn checksum(&self, package_path: &str) -> AsyncResult<'_, String>;
}
