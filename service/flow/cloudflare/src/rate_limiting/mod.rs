// Harana Actions - Cloudflare Rate Limiting Module
// This module provides Cloudflare Rate Limiting actions for checking rate limits.

pub mod output;

use output::*;
use worker::Env;

fn to_err(e: impl std::fmt::Display) -> String {
    format!("Rate Limiting error: {e}")
}

/// Check Rate Limit
pub async fn limit(env: &Env, rate_limiter: &str, key: &str) -> Result<LimitOutput, String> {
    let limiter: worker::RateLimiter = env.get_binding(rate_limiter).map_err(to_err)?;
    let outcome = limiter.limit(key.to_string()).await.map_err(to_err)?;

    Ok(LimitOutput {
        success: outcome.success,
    })
}
