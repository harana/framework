// Harana Actions - Cloudflare Rate Limiting Module Output Types

use serde::{Deserialize, Serialize};

// limit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitOutput {
    pub success: bool,
}
