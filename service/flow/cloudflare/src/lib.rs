// Harana Actions - Cloudflare Module
//
// This module provides Cloudflare Workers platform actions organized by service:
//
// - `ai` - Workers AI model inference (text generation, embeddings, classification, etc.)
// - `analytics_engine` - Analytics Engine data point writing and querying
// - `browser_rendering` - Browser rendering, screenshots, content extraction, and PDF generation
// - `d1` - D1 serverless SQL database operations
// - `durable_objects` - Durable Objects stateful storage, alarms, and WebSockets
// - `env` - Environment variables, secrets, and version metadata
// - `hyperdrive` - Hyperdrive database connection pooling
// - `kv` - Workers KV key-value storage
// - `mtls` - Mutual TLS client certificate authentication
// - `queue` - Queue message sending, batching, and processing
// - `r2` - R2 object storage (get, put, delete, multipart uploads)
// - `rate_limiting` - Rate limiting checks
// - `vectorize` - Vectorize vector database operations
// - `worker` - Worker invocation, service bindings, and cron triggers
// - `workflows` - Workflow instance management and step execution

pub mod ai;
pub mod analytics_engine;
pub mod browser_rendering;
pub mod d1;
pub mod durable_objects;
pub mod env;
pub mod hyperdrive;
pub mod kv;
pub mod mtls;
pub mod queue;
pub mod r2;
pub mod rate_limiting;
pub mod vectorize;
pub mod worker;
pub mod workflows;
