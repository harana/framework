//! Output types for AWS CloudFront actions
//!
//! This module contains all the output structs and helper types used by the AWS CloudFront actions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDistributionOutput {
        pub distribution_id: String,
        pub domain_name: String,
        pub arn: String,
        pub status: String,
        pub etag: String,
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDistributionOutput {
        pub distribution_id: String,
        pub domain_name: String,
        pub arn: String,
        pub status: String,
        pub enabled: bool,
        pub aliases: Vec<String>,
        pub origins: Vec<CloudFrontOrigin>,
        pub default_cache_behavior: Option<CacheBehavior>,
        pub cache_behaviors: Vec<CacheBehavior>,
        pub price_class: String,
        pub last_modified_time: DateTime<Utc>,
        pub etag: String,
        pub comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDistributionOutput {
        pub distribution_id: String,
        pub status: String,
        pub etag: String,
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDistributionOutput {
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDistributionsOutput {
        pub distributions: Vec<CloudFrontDistribution>,
        pub next_marker: Option<String>,
        pub is_truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvalidationOutput {
        pub invalidation_id: String,
        pub status: String,
        pub create_time: DateTime<Utc>,
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInvalidationOutput {
        pub invalidation_id: String,
        pub status: String,
        pub create_time: DateTime<Utc>,
        pub paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListInvalidationsOutput {
        pub invalidations: Vec<InvalidationSummary>,
        pub next_marker: Option<String>,
        pub is_truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudFrontDistribution {
        pub id: String,
        pub arn: String,
        pub domain_name: String,
        pub status: String,
        pub enabled: bool,
        pub aliases: Vec<String>,
        pub comment: Option<String>,
        pub price_class: String,
        pub last_modified_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudFrontOrigin {
        pub id: String,
        pub domain_name: String,
        pub origin_path: Option<String>,
        pub custom_headers: Option<HashMap<String, String>>,
        pub s3_origin_config: Option<S3OriginConfig>,
        pub custom_origin_config: Option<CustomOriginConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3OriginConfig {
        pub origin_access_identity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomOriginConfig {
        pub http_port: i32,
        pub https_port: i32,
        pub origin_protocol_policy: String,
        pub origin_ssl_protocols: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheBehavior {
        pub path_pattern: Option<String>,
        pub target_origin_id: String,
        pub viewer_protocol_policy: String,
        pub allowed_methods: Vec<String>,
        pub cached_methods: Vec<String>,
        pub compress: bool,
        pub default_ttl: Option<i64>,
        pub max_ttl: Option<i64>,
        pub min_ttl: Option<i64>,
        pub cache_policy_id: Option<String>,
        pub origin_request_policy_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomErrorResponse {
        pub error_code: i32,
        pub response_code: Option<i32>,
        pub response_page_path: Option<String>,
        pub error_caching_min_ttl: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvalidationSummary {
        pub id: String,
        pub create_time: DateTime<Utc>,
        pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudFrontTags {
        pub tags: HashMap<String, String>,
}
