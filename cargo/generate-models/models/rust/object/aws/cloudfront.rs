// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// aws_cloudfront_distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsCloudfrontDistribution {
    pub account_id: String,
    pub aliases: Option<String>,
    pub arn: Option<String>,
    pub certificate_arn: Option<String>,
    pub comment: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub default_root_object: Option<String>,
    pub distribution_id: String,
    pub domain_name: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    pub http_version: String,
    pub last_modified_time: Option<chrono::DateTime<chrono::Utc>>,
    pub origin_domain_name: String,
    pub origin_id: String,
    pub price_class: String,
    pub region: Option<String>,
    pub ssl_support_method: String,
    pub status: String,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsCloudfrontDistribution {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_cloudfront_invalidation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsCloudfrontInvalidation {
    #[serde(default = "chrono::Utc::now")]
    pub create_time: chrono::DateTime<chrono::Utc>,
    /// Reference: aws_cloudfront_distribution.id
    pub distribution_id: String,
    pub invalidation_id: String,
    pub paths: String,
    pub status: String,
}

impl AwsCloudfrontInvalidation {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_cloudfront_cache_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsCloudfrontCachePolicy {
    pub account_id: String,
    pub cache_policy_id: String,
    pub comment: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub default_ttl: i64,
    pub etag: Option<String>,
    pub max_ttl: i64,
    pub min_ttl: i64,
    pub name: String,
    pub parameters_in_cache_key: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsCloudfrontCachePolicy {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_cloudfront_origin_request_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsCloudfrontOriginRequestPolicy {
    pub account_id: String,
    pub comment: Option<String>,
    pub cookies_config: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub etag: Option<String>,
    pub headers_config: Option<String>,
    pub name: String,
    pub origin_request_policy_id: String,
    pub query_strings_config: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsCloudfrontOriginRequestPolicy {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_cloudfront_function
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsCloudfrontFunction {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub etag: Option<String>,
    pub function_arn: Option<String>,
    pub function_name: String,
    pub runtime: String,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsCloudfrontFunction {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

