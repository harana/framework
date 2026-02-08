// Harana Actions - Event Module
// This module provides event actions and functionality.

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;
use once_cell::sync::Lazy;
use dashmap::DashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
struct EventRecord {
    event_id: String,
    event_type: String,
    channel: String,
    payload: String,
    metadata: HashMap<String, Value>,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
struct Subscription {
    subscription_id: String,
    channel: String,
    handler: String,
    event_types: Option<Vec<String>>,
    acknowledged_events: Vec<String>,
}

/// Global event storage: event_id -> EventRecord
static EVENTS: Lazy<DashMap<String, EventRecord>> = Lazy::new(DashMap::new);

/// Channel storage: channel_name -> Vec<event_id>
static CHANNELS: Lazy<DashMap<String, Vec<String>>> = Lazy::new(DashMap::new);

/// Subscription storage: subscription_id -> Subscription
static SUBSCRIPTIONS: Lazy<DashMap<String, Subscription>> = Lazy::new(DashMap::new);

/// Broadcast channels for real-time event distribution
static BROADCASTERS: Lazy<DashMap<String, broadcast::Sender<String>>> = Lazy::new(DashMap::new);

/// Emit Event To Channel
pub async fn emit_event(
    channel: &str,
    event_type: &str,
    metadata: Option<HashMap<String, Value>>,
    payload: Option<Value>,
) -> Result<EmitEventOutput, String> {
    let event_id = Uuid::new_v4().to_string();
    let payload_str = payload.map(|p| p.to_string()).unwrap_or_default();
    let metadata = metadata.unwrap_or_default();
    
    let record = EventRecord {
        event_id: event_id.clone(),
        event_type: event_type.to_string(),
        channel: channel.to_string(),
        payload: payload_str,
        metadata,
        created_at: Utc::now(),
    };
    
    EVENTS.insert(event_id.clone(), record);
    
    // Add to channel
    CHANNELS
        .entry(channel.to_string())
        .or_insert_with(Vec::new)
        .push(event_id.clone());
    
    // Broadcast to subscribers if channel exists
    if let Some(sender) = BROADCASTERS.get(channel) {
        let _ = sender.send(event_id.clone());
    }
    
    Ok(EmitEventOutput {
        event_id,
        success: true,
    })
}

/// Subscribe To Channel
pub async fn subscribe_channel(
    handler: &str,
    channel: &str,
    event_types: Option<Vec<String>>,
) -> Result<SubscribeChannelOutput, String> {
    let subscription_id = Uuid::new_v4().to_string();
    
    let subscription = Subscription {
        subscription_id: subscription_id.clone(),
        channel: channel.to_string(),
        handler: handler.to_string(),
        event_types,
        acknowledged_events: Vec::new(),
    };
    
    SUBSCRIPTIONS.insert(subscription_id.clone(), subscription);
    
    // Create broadcast channel if it doesn't exist
    BROADCASTERS
        .entry(channel.to_string())
        .or_insert_with(|| broadcast::channel(1000).0);
    
    Ok(SubscribeChannelOutput {
        subscription_id,
        success: true,
    })
}

/// Unsubscribe From Channel
pub async fn unsubscribe_channel(
    subscription_id: &str,
) -> Result<UnsubscribeChannelOutput, String> {
    let removed = SUBSCRIPTIONS.remove(subscription_id).is_some();
    
    Ok(UnsubscribeChannelOutput {
        success: removed,
    })
}

/// Broadcast Event To All
pub async fn broadcast_event(
    event_type: &str,
    metadata: Option<HashMap<String, Value>>,
    exclude_channels: Option<Vec<String>>,
    payload: Option<Value>,
) -> Result<BroadcastEventOutput, String> {
    let event_id = Uuid::new_v4().to_string();
    let exclude = exclude_channels.unwrap_or_default();
    
    // Collect channel names first to avoid holding the iterator while modifying
    let channels: Vec<String> = CHANNELS
        .iter()
        .map(|entry| entry.key().clone())
        .filter(|channel| !exclude.contains(channel))
        .collect();
    
    let recipients = channels.len() as i32;
    
    // Broadcast to all channels except excluded ones
    for channel in channels {
        emit_event(
            &channel,
            event_type,
            metadata.clone(),
            payload.clone(),
        ).await?;
    }
    
    Ok(BroadcastEventOutput {
        event_id,
        recipients,
    })
}

/// Get Event By ID
pub async fn get_event(
    event_id: &str,
) -> Result<GetEventOutput, String> {
    let event = EVENTS
        .get(event_id)
        .ok_or_else(|| format!("Event not found: {}", event_id))?;
    
    Ok(GetEventOutput {
        event_type: event.event_type.clone(),
        metadata: event.metadata.clone(),
        payload: event.payload.clone(),
        created_at: event.created_at.to_rfc3339(),
        channel: event.channel.clone(),
    })
}

/// List Channel Events
pub async fn list_events(
    channel: &str,
    event_types: Option<Vec<String>>,
    end_time: Option<&str>,
    limit: Option<i32>,
    start_time: Option<&str>,
) -> Result<ListEventsOutput, String> {
    let limit = limit.unwrap_or(100) as usize;
    
    // Parse time filters if provided
    let start_dt = start_time
        .map(|s| DateTime::parse_from_rfc3339(s).ok())
        .flatten()
        .map(|dt| dt.with_timezone(&Utc));
    
    let end_dt = end_time
        .map(|s| DateTime::parse_from_rfc3339(s).ok())
        .flatten()
        .map(|dt| dt.with_timezone(&Utc));
    
    let channel_events = CHANNELS.get(channel);
    if channel_events.is_none() {
        return Ok(ListEventsOutput {
            events: Vec::new(),
            total: 0,
        });
    }
    
    let event_ids = channel_events.unwrap();
    let mut events = Vec::new();
    
    for event_id in event_ids.iter() {
        if let Some(event) = EVENTS.get(event_id) {
            // Filter by event type
            if let Some(ref types) = event_types {
                if !types.contains(&event.event_type) {
                    continue;
                }
            }
            
            // Filter by time range
            if let Some(start) = start_dt {
                if event.created_at < start {
                    continue;
                }
            }
            if let Some(end) = end_dt {
                if event.created_at > end {
                    continue;
                }
            }
            
            let mut event_map = HashMap::new();
            event_map.insert("event_id".to_string(), Value::String(event.event_id.clone()));
            event_map.insert("event_type".to_string(), Value::String(event.event_type.clone()));
            event_map.insert("channel".to_string(), Value::String(event.channel.clone()));
            event_map.insert("payload".to_string(), Value::String(event.payload.clone()));
            event_map.insert("created_at".to_string(), Value::String(event.created_at.to_rfc3339()));
            
            events.push(event_map);
            
            if events.len() >= limit {
                break;
            }
        }
    }
    
    let total = events.len() as i32;
    
    Ok(ListEventsOutput {
        events,
        total,
    })
}

/// Acknowledge Event Receipt
pub async fn acknowledge_event(
    event_id: &str,
    subscription_id: &str,
) -> Result<AcknowledgeEventOutput, String> {
    // Verify event exists
    if !EVENTS.contains_key(event_id) {
        return Err(format!("Event not found: {}", event_id));
    }
    
    // Verify subscription exists and update
    let mut subscription = SUBSCRIPTIONS
        .get_mut(subscription_id)
        .ok_or_else(|| format!("Subscription not found: {}", subscription_id))?;
    
    if !subscription.acknowledged_events.contains(&event_id.to_string()) {
        subscription.acknowledged_events.push(event_id.to_string());
    }
    
    Ok(AcknowledgeEventOutput {
        success: true,
    })
}

/// Replay Events From Time
pub async fn replay_events(
    start_time: &str,
    channel: &str,
    end_time: Option<&str>,
    event_types: Option<Vec<String>>,
) -> Result<ReplayEventsOutput, String> {
    // Parse time range
    let start_dt = DateTime::parse_from_rfc3339(start_time)
        .map_err(|e| format!("Invalid start_time: {}", e))?
        .with_timezone(&Utc);
    
    let end_dt = end_time
        .map(|s| DateTime::parse_from_rfc3339(s).ok())
        .flatten()
        .map(|dt| dt.with_timezone(&Utc));
    
    let channel_events = CHANNELS.get(channel);
    if channel_events.is_none() {
        return Ok(ReplayEventsOutput {
            events_replayed: 0,
            success: true,
        });
    }
    
    let event_ids = channel_events.unwrap();
    let mut replayed_count = 0;
    
    for event_id in event_ids.iter() {
        if let Some(event) = EVENTS.get(event_id) {
            // Filter by time range
            if event.created_at < start_dt {
                continue;
            }
            if let Some(end) = end_dt {
                if event.created_at > end {
                    continue;
                }
            }
            
            // Filter by event type
            if let Some(ref types) = event_types {
                if !types.contains(&event.event_type) {
                    continue;
                }
            }
            
            // Broadcast to subscribers
            if let Some(sender) = BROADCASTERS.get(channel) {
                let _ = sender.send(event.event_id.clone());
            }
            
            replayed_count += 1;
        }
    }
    
    Ok(ReplayEventsOutput {
        events_replayed: replayed_count,
        success: true,
    })
}

#[cfg(test)]
mod tests;
