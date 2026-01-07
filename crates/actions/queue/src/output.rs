// Harana Actions - Queue Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// acknowledge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcknowledgeOutput {
    pub success: bool
}

// create_queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQueueOutput {
    pub success: bool,
    pub queue_url: String
}

// delete_queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteQueueOutput {
    pub success: bool
}

// dequeue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DequeueOutput {
    pub message: String,
    pub message_id: String,
    pub found: bool,
    pub receipt_handle: String
}

// enqueue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnqueueOutput {
    pub success: bool,
    pub message_id: String
}

// get_queue_stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQueueStatsOutput {
    pub delayed_count: i32,
    pub oldest_message_age: i32,
    pub in_flight_count: i32,
    pub message_count: i32
}

// list_queues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListQueuesOutput {
    pub queues: Vec<String>,
    pub total: i32
}

// move_to_dlq
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveToDlqOutput {
    pub success: bool
}

// nack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NackOutput {
    pub success: bool
}

// peek
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeekOutput {
    pub messages: Vec<HashMap<String, Value>>
}

// purge_queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurgeQueueOutput {
    pub messages_deleted: i32,
    pub success: bool
}
