//! Output types for AWS CloudFront actions
//!
//! This module contains all the output structs and helper types used by the AWS CloudFront actions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Output for create_distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDistributionOutput {
    /// The identifier for the distribution
    pub distribution_id: String,
    /// The domain name corresponding to the distribution
    pub domain_name: String,
    /// The ARN of the distribution
    pub arn: String,
    /// The current status of the distribution
    pub status: String,
    /// The ETag for the distribution
    pub etag: String,
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for get_distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDistributionOutput {
    /// The identifier for the distribution
    pub distribution_id: String,
    /// The domain name corresponding to the distribution
    pub domain_name: String,
    /// The ARN of the distribution
    pub arn: String,
    /// The current status of the distribution
    pub status: String,
    /// Whether the distribution is enabled
    pub enabled: bool,
    /// CNAMEs (alternate domain names) for the distribution
    pub aliases: Vec<String>,
    /// Information about origin servers
    pub origins: Vec<CloudFrontOrigin>,
    /// The default cache behavior
    pub default_cache_behavior: Option<CacheBehavior>,
    /// Additional cache behaviors
    pub cache_behaviors: Vec<CacheBehavior>,
    /// The price class for the distribution
    pub price_class: String,
    /// The date and time the distribution was last modified
    pub last_modified_time: DateTime<Utc>,
    /// The ETag for the distribution
    pub etag: String,
    /// Comment for the distribution
    pub comment: String,
}

/// Output for update_distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDistributionOutput {
    /// The identifier for the distribution
    pub distribution_id: String,
    /// The current status of the distribution
    pub status: String,
    /// The ETag for the updated distribution
    pub etag: String,
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for delete_distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDistributionOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for list_distributions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDistributionsOutput {
    /// List of distribution summaries
    pub distributions: Vec<CloudFrontDistribution>,
    /// The marker for the next set of results
    pub next_marker: Option<String>,
    /// Whether the list is truncated
    pub is_truncated: bool,
}

/// Output for create_invalidation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvalidationOutput {
    /// The identifier for the invalidation request
    pub invalidation_id: String,
    /// The current status of the invalidation
    pub status: String,
    /// The time the invalidation request was created
    pub create_time: DateTime<Utc>,
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for get_invalidation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInvalidationOutput {
    /// The identifier for the invalidation request
    pub invalidation_id: String,
    /// The current status of the invalidation
    pub status: String,
    /// The time the invalidation request was created
    pub create_time: DateTime<Utc>,
    /// The paths being invalidated
    pub paths: Vec<String>,
}

/// Output for list_invalidations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListInvalidationsOutput {
    /// List of invalidation summaries
    pub invalidations: Vec<InvalidationSummary>,
    /// The marker for the next set of results
    pub next_marker: Option<String>,
    /// Whether the list is truncated
    pub is_truncated: bool,
}

/// Summary information about a CloudFront distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudFrontDistribution {
    /// The identifier for the distribution
    pub id: String,
    /// The ARN of the distribution
    pub arn: String,
    /// The domain name corresponding to the distribution
    pub domain_name: String,
    /// The current status of the distribution
    pub status: String,
    /// Whether the distribution is enabled
    pub enabled: bool,
    /// CNAMEs (alternate domain names) for the distribution
    pub aliases: Vec<String>,
    /// Comment for the distribution
    pub comment: Option<String>,
    /// The price class for the distribution
    pub price_class: String,
    /// The date and time the distribution was last modified
    pub last_modified_time: DateTime<Utc>,
}

/// Information about a CloudFront origin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudFrontOrigin {
    /// The unique identifier for the origin
    pub id: String,
    /// The domain name for the origin
    pub domain_name: String,
    /// The path that CloudFront appends to the origin domain name
    pub origin_path: Option<String>,
    /// Custom headers to include in requests sent to the origin
    pub custom_headers: Option<HashMap<String, String>>,
    /// S3 origin configuration
    pub s3_origin_config: Option<S3OriginConfig>,
    /// Custom origin configuration
    pub custom_origin_config: Option<CustomOriginConfig>,
}

/// S3 origin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3OriginConfig {
    /// The CloudFront origin access identity
    pub origin_access_identity: String,
}

/// Custom origin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomOriginConfig {
    /// The HTTP port the custom origin listens on
    pub http_port: i32,
    /// The HTTPS port the custom origin listens on
    pub https_port: i32,
    /// The origin protocol policy
    pub origin_protocol_policy: String,
    /// The SSL/TLS protocols that CloudFront can use
    pub origin_ssl_protocols: Vec<String>,
}

/// Cache behavior configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheBehavior {
    /// The pattern that specifies which requests to apply the behavior to
    pub path_pattern: Option<String>,
    /// The ID of the origin to send requests to
    pub target_origin_id: String,
    /// The protocol viewers can use to access the files
    pub viewer_protocol_policy: String,
    /// HTTP methods that CloudFront processes and forwards
    pub allowed_methods: Vec<String>,
    /// HTTP methods for which CloudFront caches responses
    pub cached_methods: Vec<String>,
    /// Whether CloudFront automatically compresses content
    pub compress: bool,
    /// The default time for objects to stay in cache
    pub default_ttl: Option<i64>,
    /// The maximum time for objects to stay in cache
    pub max_ttl: Option<i64>,
    /// The minimum time for objects to stay in cache
    pub min_ttl: Option<i64>,
    /// The ID of the cache policy
    pub cache_policy_id: Option<String>,
    /// The ID of the origin request policy
    pub origin_request_policy_id: Option<String>,
}

/// Custom error response configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomErrorResponse {
    /// The HTTP status code for which you want to specify a custom error page
    pub error_code: i32,
    /// The HTTP status code that CloudFront returns to viewers
    pub response_code: Option<i32>,
    /// The path to the custom error page
    pub response_page_path: Option<String>,
    /// The minimum amount of time to cache the error response
    pub error_caching_min_ttl: Option<i64>,
}

/// Summary information about an invalidation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvalidationSummary {
    /// The unique identifier for the invalidation request
    pub id: String,
    /// The time the invalidation request was created
    pub create_time: DateTime<Utc>,
    /// The current status of the invalidation
    pub status: String,
}

/// Tags for CloudFront resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudFrontTags {
    /// Map of tag keys to values
    pub tags: HashMap<String, String>,
}
