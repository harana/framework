// Harana Actions - Parallel Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;

// ParallelTask - input task structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelTask {
    pub id: String,
    pub handler: String,
    pub input: Option<Value>,
    pub timeout_ms: Option<i64>,
}

// ParallelResult - result of a single task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelResult {
    pub task_id: String,
    pub success: bool,
    pub result: Option<Value>,
    pub error: Option<String>,
    pub duration_ms: i64,
}

// ParallelSettledResult - result for all_settled
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelSettledResult {
    pub task_id: String,
    pub status: String, // "fulfilled" or "rejected"
    pub value: Option<Value>,
    pub reason: Option<String>,
    pub duration_ms: i64,
}

// ParallelError - error info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelError {
    pub task_id: String,
    pub index: i32,
    pub message: String,
    pub code: String,
}

// all
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllOutput {
    pub completed: i32,
    pub failed: i32,
    pub results: Vec<ParallelResult>,
    pub success: bool,
}

// race
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceOutput {
    pub result: Option<Value>,
    pub success: bool,
    pub winner_index: i32,
}

// any
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnyOutput {
    pub result: Option<Value>,
    pub success: bool,
    pub success_index: i32,
}

// all_settled
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllSettledOutput {
    pub fulfilled: i32,
    pub rejected: i32,
    pub results: Vec<ParallelSettledResult>,
}

// map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapOutput {
    pub errors: Vec<ParallelError>,
    pub results: Vec<Value>,
    pub success: bool,
}

// filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterOutput {
    pub count: i32,
    pub results: Vec<Value>,
}

// reduce
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReduceOutput {
    pub result: Value,
}

// retry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryOutput {
    pub attempts: i32,
    pub result: Option<Value>,
    pub success: bool,
}

// timeout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutOutput {
    pub result: Option<Value>,
    pub success: bool,
    pub timed_out: bool,
}

// batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOutput {
    pub batch_count: i32,
    pub results: Vec<Value>,
    pub success: bool,
}

// semaphore
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemaphoreOutput {
    pub completed: i32,
    pub failed: i32,
    pub results: Vec<ParallelResult>,
}
