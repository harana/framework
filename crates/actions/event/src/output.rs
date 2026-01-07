// Harana Actions - Event Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// acknowledge_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcknowledgeEventOutput {
    pub success: bool
}

// broadcast_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastEventOutput {
    pub recipients: i32,
    pub event_id: String
}

// emit_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmitEventOutput {
    pub event_id: String,
    pub success: bool
}

// get_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEventOutput {
    pub event_type: String,
    pub metadata: HashMap<String, Value>,
    pub payload: String,
    pub created_at: String,
    pub channel: String
}

// list_events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEventsOutput {
    pub total: i32,
    pub events: Vec<HashMap<String, Value>>
}

// replay_events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayEventsOutput {
    pub success: bool,
    pub events_replayed: i32
}

// subscribe_channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeChannelOutput {
    pub subscription_id: String,
    pub success: bool
}

// unsubscribe_channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeChannelOutput {
    pub success: bool
}
