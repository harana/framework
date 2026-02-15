// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait S3Action {
    async fn deploy_s3(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn deploy_s3_cloudfront(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn deploy_s3_sync(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn deploy_aws_credentials(&self) -> Result<(), Box<dyn std::error::Error>>;
}
