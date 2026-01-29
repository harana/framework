pub mod output;

use output::*;
use chrono::{DateTime, Duration, Utc};
use dashmap::DashMap;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use uuid::Uuid;

/// Internal message representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QueueMessage {
    message_id: String,
    message: String,
    priority: i32,
    enqueued_at: DateTime<Utc>,
    visible_at: DateTime<Utc>,
    receipt_handle: Option<String>,
    deduplication_id: Option<String>,
}

impl PartialEq for QueueMessage {
    fn eq(&self, other: &Self) -> bool {
        self.message_id == other.message_id
    }
}

impl Eq for QueueMessage {}

impl PartialOrd for QueueMessage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueMessage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Higher priority first, then earlier enqueued_at
        other.priority.cmp(&self.priority)
            .then_with(|| self.enqueued_at.cmp(&other.enqueued_at))
    }
}

/// Queue configuration.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct QueueConfig {
    max_message_size: i32,
    message_retention: i32,
    visibility_timeout: i32,
    created_at: DateTime<Utc>,
}

/// Internal queue storage.
struct Queue {
    config: QueueConfig,
    messages: VecDeque<QueueMessage>,
    in_flight: HashMap<String, QueueMessage>,
    dlq: VecDeque<QueueMessage>,
    dedup_ids: HashMap<String, DateTime<Utc>>,
}

impl Queue {
    fn new(config: QueueConfig) -> Self {
        Self {
            config,
            messages: VecDeque::new(),
            in_flight: HashMap::new(),
            dlq: VecDeque::new(),
            dedup_ids: HashMap::new(),
        }
    }
}

// Use once_cell for lazy initialization
use std::sync::OnceLock;

fn get_queues() -> &'static DashMap<String, Arc<RwLock<Queue>>> {
    static QUEUES: OnceLock<DashMap<String, Arc<RwLock<Queue>>>> = OnceLock::new();
    QUEUES.get_or_init(|| DashMap::new())
}

/// Enqueue Message To Queue.
pub async fn enqueue(
    queue_name: &str,
    message: &str,
    delay_seconds: Option<i32>,
    deduplication_id: Option<&str>,
    priority: Option<i32>,
) -> Result<EnqueueOutput, String> {
    let queues = get_queues();
    let queue = queues.get(queue_name)
        .ok_or_else(|| format!("Queue '{}' not found", queue_name))?;
    
    let mut q = queue.write();
    
    // Check deduplication
    if let Some(dedup_id) = deduplication_id {
        if let Some(expiry) = q.dedup_ids.get(dedup_id) {
            if *expiry > Utc::now() {
                return Ok(EnqueueOutput {
                    success: false,
                    message_id: String::new(),
                });
            }
        }
        // Store dedup_id for 5 minutes
        q.dedup_ids.insert(dedup_id.to_string(), Utc::now() + Duration::minutes(5));
    }
    
    // Check message size
    if message.len() as i32 > q.config.max_message_size {
        return Err(format!(
            "Message size {} exceeds maximum {}",
            message.len(),
            q.config.max_message_size
        ));
    }
    
    let delay = delay_seconds.unwrap_or(0);
    let now = Utc::now();
    let visible_at = if delay > 0 {
        now + Duration::seconds(delay as i64)
    } else {
        now
    };
    
    let msg = QueueMessage {
        message_id: Uuid::new_v4().to_string(),
        message: message.to_string(),
        priority: priority.unwrap_or(0),
        enqueued_at: now,
        visible_at,
        receipt_handle: None,
        deduplication_id: deduplication_id.map(String::from),
    };
    
    let message_id = msg.message_id.clone();
    q.messages.push_back(msg);
    
    // Sort by priority (higher first) then by enqueued_at
    let mut msgs: Vec<_> = q.messages.drain(..).collect();
    msgs.sort_by(|a, b| b.priority.cmp(&a.priority).then_with(|| a.enqueued_at.cmp(&b.enqueued_at)));
    q.messages = msgs.into_iter().collect();
    
    Ok(EnqueueOutput {
        success: true,
        message_id,
    })
}

/// Dequeue Message From Queue.
pub async fn dequeue(
    queue_name: &str,
    visibility_timeout: Option<i32>,
    wait_time_seconds: Option<i32>,
) -> Result<DequeueOutput, String> {
    let wait_time = wait_time_seconds.unwrap_or(0);
    let deadline = Utc::now() + Duration::seconds(wait_time as i64);
    
    loop {
        let queues = get_queues();
        let queue = queues.get(queue_name)
            .ok_or_else(|| format!("Queue '{}' not found", queue_name))?;
        
        let mut q = queue.write();
        let now = Utc::now();
        let vis_timeout = visibility_timeout.unwrap_or(q.config.visibility_timeout);
        
        // Find first visible message
        if let Some(pos) = q.messages.iter().position(|m| m.visible_at <= now) {
            let mut msg = q.messages.remove(pos).unwrap();
            let receipt_handle = Uuid::new_v4().to_string();
            msg.receipt_handle = Some(receipt_handle.clone());
            msg.visible_at = now + Duration::seconds(vis_timeout as i64);
            
            let result = DequeueOutput {
                found: true,
                message: msg.message.clone(),
                message_id: msg.message_id.clone(),
                receipt_handle: receipt_handle.clone(),
            };
            
            q.in_flight.insert(receipt_handle, msg);
            return Ok(result);
        }
        
        drop(q);
        drop(queue);
        
        // No message found, check if we should wait
        if wait_time == 0 || Utc::now() >= deadline {
            return Ok(DequeueOutput {
                found: false,
                message: String::new(),
                message_id: String::new(),
                receipt_handle: String::new(),
            });
        }
        
        // Wait a bit before retrying
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

/// Peek Message Without Removing.
pub async fn peek(
    queue_name: &str,
    count: Option<i32>,
) -> Result<PeekOutput, String> {
    let queues = get_queues();
    let queue = queues.get(queue_name)
        .ok_or_else(|| format!("Queue '{}' not found", queue_name))?;
    
    let q = queue.read();
    let count = count.unwrap_or(1).max(1) as usize;
    let now = Utc::now();
    
    let messages: Vec<HashMap<String, Value>> = q.messages
        .iter()
        .filter(|m| m.visible_at <= now)
        .take(count)
        .map(|m| {
            let mut map = HashMap::new();
            map.insert("message_id".to_string(), json!(m.message_id));
            map.insert("message".to_string(), json!(m.message));
            map.insert("enqueued_at".to_string(), json!(m.enqueued_at.to_rfc3339()));
            map
        })
        .collect();
    
    Ok(PeekOutput { messages })
}

/// Acknowledge Message Processing.
pub async fn acknowledge(
    queue_name: &str,
    receipt_handle: &str,
) -> Result<AcknowledgeOutput, String> {
    let queues = get_queues();
    let queue = queues.get(queue_name)
        .ok_or_else(|| format!("Queue '{}' not found", queue_name))?;
    
    let mut q = queue.write();
    
    if q.in_flight.remove(receipt_handle).is_some() {
        Ok(AcknowledgeOutput { success: true })
    } else {
        Ok(AcknowledgeOutput { success: false })
    }
}

/// Return Message To Queue.
pub async fn nack(
    queue_name: &str,
    receipt_handle: &str,
    delay_seconds: Option<i32>,
) -> Result<NackOutput, String> {
    let queues = get_queues();
    let queue = queues.get(queue_name)
        .ok_or_else(|| format!("Queue '{}' not found", queue_name))?;
    
    let mut q = queue.write();
    
    if let Some(mut msg) = q.in_flight.remove(receipt_handle) {
        let delay = delay_seconds.unwrap_or(0);
        msg.visible_at = Utc::now() + Duration::seconds(delay as i64);
        msg.receipt_handle = None;
        q.messages.push_back(msg);
        Ok(NackOutput { success: true })
    } else {
        Ok(NackOutput { success: false })
    }
}

/// Get Queue Statistics.
pub async fn get_queue_stats(
    queue_name: &str,
) -> Result<GetQueueStatsOutput, String> {
    let queues = get_queues();
    let queue = queues.get(queue_name)
        .ok_or_else(|| format!("Queue '{}' not found", queue_name))?;
    
    let q = queue.read();
    let now = Utc::now();
    
    let message_count = q.messages.iter().filter(|m| m.visible_at <= now).count() as i32;
    let delayed_count = q.messages.iter().filter(|m| m.visible_at > now).count() as i32;
    let in_flight_count = q.in_flight.len() as i32;
    
    let oldest_message_age = q.messages
        .iter()
        .filter(|m| m.visible_at <= now)
        .map(|m| (now - m.enqueued_at).num_seconds() as i32)
        .max()
        .unwrap_or(0);
    
    Ok(GetQueueStatsOutput {
        message_count,
        delayed_count,
        in_flight_count,
        oldest_message_age,
    })
}

/// Purge All Queue Messages.
pub async fn purge_queue(
    queue_name: &str,
) -> Result<PurgeQueueOutput, String> {
    let queues = get_queues();
    let queue = queues.get(queue_name)
        .ok_or_else(|| format!("Queue '{}' not found", queue_name))?;
    
    let mut q = queue.write();
    let messages_deleted = (q.messages.len() + q.in_flight.len()) as i32;
    q.messages.clear();
    q.in_flight.clear();
    
    Ok(PurgeQueueOutput {
        success: true,
        messages_deleted,
    })
}

/// Create New Queue.
pub async fn create_queue(
    queue_name: &str,
    max_message_size: Option<i32>,
    message_retention: Option<i32>,
    visibility_timeout: Option<i32>,
) -> Result<CreateQueueOutput, String> {
    let queues = get_queues();
    
    if queues.contains_key(queue_name) {
        return Err(format!("Queue '{}' already exists", queue_name));
    }
    
    let config = QueueConfig {
        max_message_size: max_message_size.unwrap_or(262144),
        message_retention: message_retention.unwrap_or(345600),
        visibility_timeout: visibility_timeout.unwrap_or(30),
        created_at: Utc::now(),
    };
    
    queues.insert(queue_name.to_string(), Arc::new(RwLock::new(Queue::new(config))));
    
    Ok(CreateQueueOutput {
        success: true,
        queue_url: format!("queue://{}", queue_name),
    })
}

/// Delete Existing Queue.
pub async fn delete_queue(
    queue_name: &str,
) -> Result<DeleteQueueOutput, String> {
    let queues = get_queues();
    
    if queues.remove(queue_name).is_some() {
        Ok(DeleteQueueOutput { success: true })
    } else {
        Err(format!("Queue '{}' not found", queue_name))
    }
}

/// List Available Queues.
pub async fn list_queues(
    prefix: Option<&str>,
) -> Result<ListQueuesOutput, String> {
    let queues = get_queues();
    
    let queue_names: Vec<String> = queues
        .iter()
        .map(|entry| entry.key().clone())
        .filter(|name| {
            prefix.map_or(true, |p| name.starts_with(p))
        })
        .collect();
    
    let total = queue_names.len() as i32;
    
    Ok(ListQueuesOutput {
        queues: queue_names,
        total,
    })
}

/// Move Message To Dead Letter.
pub async fn move_to_dlq(
    queue_name: &str,
    receipt_handle: &str,
    _reason: Option<&str>,
) -> Result<MoveToDlqOutput, String> {
    let queues = get_queues();
    let queue = queues.get(queue_name)
        .ok_or_else(|| format!("Queue '{}' not found", queue_name))?;
    
    let mut q = queue.write();
    
    if let Some(msg) = q.in_flight.remove(receipt_handle) {
        q.dlq.push_back(msg);
        Ok(MoveToDlqOutput { success: true })
    } else {
        Ok(MoveToDlqOutput { success: false })
    }
}

#[cfg(test)]
mod tests;
