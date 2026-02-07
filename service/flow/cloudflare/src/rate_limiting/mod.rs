// Harana Actions - Cloudflare Rate Limiting Module
// This module provides Cloudflare Rate Limiting actions for checking rate limits.

pub mod output;

use output::*;

/// Check Rate Limit
pub async fn limit(
    rate_limiter: &str,
    key: &str,
) -> Result<LimitOutput, String> {
    unimplemented!("limit")
}
