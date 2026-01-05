// Harana Actions - Event Module Output Types
// Auto-generated output structs for Event action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// emit_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmitEventOutput {
    pub event_id: String,
    pub success: bool,
}

// subscribe_channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeChannelOutput {
    pub subscription_id: String,
    pub success: bool,
}

// unsubscribe_channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeChannelOutput {
    pub success: bool,
}

// broadcast_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastEventOutput {
    pub event_id: String,
    pub recipients: i32,
}

// get_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEventOutput {
    pub channel: String,
    pub created_at: String, // datetime
    pub event_type: String,
    pub metadata: HashMap<String, Value>,
    pub payload: Value,
}

// list_events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEventsOutput {
    pub events: Vec<HashMap<String, Value>>,
    pub total: i32,
}

// acknowledge_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcknowledgeEventOutput {
    pub success: bool,
}

// replay_events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayEventsOutput {
    pub events_replayed: i32,
    pub success: bool,
}
