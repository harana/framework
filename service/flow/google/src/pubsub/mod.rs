// Harana Actions - Google Cloud Pub/Sub Module
// This module provides Google Cloud Pub/Sub messaging actions.

pub mod output;

#[cfg(test)]
mod tests;

use chrono::{Duration, Utc};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use uuid::Uuid;

use output::*;

// In-memory storage
static TOPICS: Lazy<DashMap<String, StoredTopic>> = Lazy::new(DashMap::new);
static SUBSCRIPTIONS: Lazy<DashMap<String, StoredSubscription>> = Lazy::new(DashMap::new);
static MESSAGES: Lazy<DashMap<String, Vec<StoredMessage>>> = Lazy::new(DashMap::new);
static PENDING_MESSAGES: Lazy<DashMap<String, Vec<StoredPendingMessage>>> = Lazy::new(DashMap::new);
static SNAPSHOTS: Lazy<DashMap<String, StoredSnapshot>> = Lazy::new(DashMap::new);

fn topic_name(project_id: &str, topic_id: &str) -> String {
    format!("projects/{}/topics/{}", project_id, topic_id)
}

fn subscription_name(project_id: &str, subscription_id: &str) -> String {
    format!("projects/{}/subscriptions/{}", project_id, subscription_id)
}

fn snapshot_name(project_id: &str, snapshot_id: &str) -> String {
    format!("projects/{}/snapshots/{}", project_id, snapshot_id)
}

fn generate_message_id() -> String {
    Uuid::new_v4().to_string().replace("-", "")[..16].to_string()
}

fn generate_ack_id() -> String {
    format!("ack_{}", Uuid::new_v4().to_string().replace("-", "")[..24].to_string())
}

/// Create a Pub/Sub topic
pub async fn create_topic(
    project_id: String,
    topic_id: String,
    labels: Option<PubSubLabels>,
    message_retention_duration: Option<String>,
    schema_settings: Option<PubSubSchemaSettings>,
) -> CreateTopicOutput {
    let name = topic_name(&project_id, &topic_id);
    let now = Utc::now();

    let stored = StoredTopic {
        name: name.clone(),
        project_id,
        topic_id,
        labels: labels.unwrap_or_default(),
        message_retention_duration,
        schema_settings,
        created_at: now,
    };

    TOPICS.insert(name.clone(), stored);

    CreateTopicOutput {
        topic_name: name,
        success: true,
    }
}

/// Delete a Pub/Sub topic
pub async fn delete_topic(project_id: String, topic_id: String) -> DeleteTopicOutput {
    let name = topic_name(&project_id, &topic_id);
    let success = TOPICS.remove(&name).is_some();
    DeleteTopicOutput { success }
}

/// Get a Pub/Sub topic
pub async fn get_topic(project_id: String, topic_id: String) -> GetTopicOutput {
    let name = topic_name(&project_id, &topic_id);

    if let Some(entry) = TOPICS.get(&name) {
        GetTopicOutput {
            topic_name: entry.name.clone(),
            labels: entry.labels.clone(),
            message_retention_duration: entry.message_retention_duration.clone(),
            schema_settings: entry.schema_settings.clone(),
        }
    } else {
        GetTopicOutput {
            topic_name: name,
            labels: PubSubLabels::default(),
            message_retention_duration: None,
            schema_settings: None,
        }
    }
}

/// List Pub/Sub topics
pub async fn list_topics(project_id: String, page_size: Option<i32>, _page_token: Option<String>) -> ListTopicsOutput {
    let page_size = page_size.unwrap_or(100) as usize;
    let prefix = format!("projects/{}/topics/", project_id);

    let topics: Vec<PubSubTopic> = TOPICS
        .iter()
        .filter(|entry| entry.name.starts_with(&prefix))
        .take(page_size)
        .map(|entry| {
            let t = entry.value();
            PubSubTopic {
                name: t.name.clone(),
                labels: t.labels.clone(),
                message_retention_duration: t.message_retention_duration.clone(),
            }
        })
        .collect();

    ListTopicsOutput {
        topics,
        next_page_token: None,
    }
}

/// Publish a message to a topic
pub async fn publish(
    project_id: String,
    topic_id: String,
    data: String,
    attributes: Option<PubSubMessageAttributes>,
    ordering_key: Option<String>,
) -> PublishOutput {
    let name = topic_name(&project_id, &topic_id);
    let message_id = generate_message_id();
    let now = Utc::now();

    let message = StoredMessage {
        message_id: message_id.clone(),
        topic_name: name.clone(),
        data,
        attributes: attributes.unwrap_or_default(),
        ordering_key,
        publish_time: now,
    };

    MESSAGES.entry(name).or_insert_with(Vec::new).push(message);

    PublishOutput {
        message_id,
        success: true,
    }
}

/// Publish multiple messages to a topic
pub async fn publish_batch(
    project_id: String,
    topic_id: String,
    messages: Vec<PubSubPublishMessage>,
) -> PublishBatchOutput {
    let name = topic_name(&project_id, &topic_id);
    let now = Utc::now();
    let mut message_ids = Vec::new();
    let mut success_count = 0;

    for msg in messages {
        let message_id = generate_message_id();
        let stored = StoredMessage {
            message_id: message_id.clone(),
            topic_name: name.clone(),
            data: msg.data,
            attributes: msg.attributes.unwrap_or_default(),
            ordering_key: msg.ordering_key,
            publish_time: now,
        };

        MESSAGES.entry(name.clone()).or_insert_with(Vec::new).push(stored);
        message_ids.push(message_id);
        success_count += 1;
    }

    PublishBatchOutput {
        message_ids,
        success_count,
        failure_count: 0,
    }
}

/// Create a subscription
#[allow(clippy::too_many_arguments)]
pub async fn create_subscription(
    project_id: String,
    subscription_id: String,
    topic_id: String,
    ack_deadline_seconds: Option<i32>,
    message_retention_duration: Option<String>,
    push_config: Option<PushConfig>,
    dead_letter_policy: Option<DeadLetterPolicy>,
    retry_policy: Option<RetryPolicy>,
    enable_message_ordering: Option<bool>,
    retain_acked_messages: Option<bool>,
    filter: Option<String>,
) -> CreateSubscriptionOutput {
    let name = subscription_name(&project_id, &subscription_id);
    let now = Utc::now();

    let stored = StoredSubscription {
        name: name.clone(),
        project_id,
        subscription_id,
        topic_id,
        ack_deadline_seconds: ack_deadline_seconds.unwrap_or(10),
        message_retention_duration,
        labels: PubSubLabels::default(),
        push_config,
        dead_letter_policy,
        retry_policy,
        enable_message_ordering: enable_message_ordering.unwrap_or(false),
        retain_acked_messages: retain_acked_messages.unwrap_or(false),
        filter,
        created_at: now,
    };

    SUBSCRIPTIONS.insert(name.clone(), stored);

    CreateSubscriptionOutput {
        subscription_name: name,
        success: true,
    }
}

/// Delete a subscription
pub async fn delete_subscription(project_id: String, subscription_id: String) -> DeleteSubscriptionOutput {
    let name = subscription_name(&project_id, &subscription_id);
    let success = SUBSCRIPTIONS.remove(&name).is_some();
    DeleteSubscriptionOutput { success }
}

/// Get a subscription
pub async fn get_subscription(project_id: String, subscription_id: String) -> GetSubscriptionOutput {
    let name = subscription_name(&project_id, &subscription_id);

    if let Some(entry) = SUBSCRIPTIONS.get(&name) {
        GetSubscriptionOutput {
            subscription_name: entry.name.clone(),
            topic: topic_name(&entry.project_id, &entry.topic_id),
            ack_deadline_seconds: entry.ack_deadline_seconds,
            message_retention_duration: entry.message_retention_duration.clone(),
            labels: entry.labels.clone(),
            push_config: entry.push_config.clone(),
        }
    } else {
        GetSubscriptionOutput {
            subscription_name: name,
            topic: String::new(),
            ack_deadline_seconds: 10,
            message_retention_duration: None,
            labels: PubSubLabels::default(),
            push_config: None,
        }
    }
}

/// List subscriptions
pub async fn list_subscriptions(
    project_id: String,
    page_size: Option<i32>,
    _page_token: Option<String>,
) -> ListSubscriptionsOutput {
    let page_size = page_size.unwrap_or(100) as usize;
    let prefix = format!("projects/{}/subscriptions/", project_id);

    let subscriptions: Vec<PubSubSubscription> = SUBSCRIPTIONS
        .iter()
        .filter(|entry| entry.name.starts_with(&prefix))
        .take(page_size)
        .map(|entry| {
            let s = entry.value();
            PubSubSubscription {
                name: s.name.clone(),
                topic: topic_name(&s.project_id, &s.topic_id),
                ack_deadline_seconds: s.ack_deadline_seconds,
                message_retention_duration: s.message_retention_duration.clone(),
                labels: s.labels.clone(),
            }
        })
        .collect();

    ListSubscriptionsOutput {
        subscriptions,
        next_page_token: None,
    }
}

/// Pull messages from a subscription
pub async fn pull(
    project_id: String,
    subscription_id: String,
    max_messages: Option<i32>,
    _return_immediately: Option<bool>,
) -> PullOutput {
    let sub_name = subscription_name(&project_id, &subscription_id);
    let max_messages = max_messages.unwrap_or(10) as usize;

    // Get topic for this subscription
    let topic_name_str = if let Some(entry) = SUBSCRIPTIONS.get(&sub_name) {
        topic_name(&entry.project_id, &entry.topic_id)
    } else {
        return PullOutput {
            messages: Vec::new(),
            count: 0,
        };
    };

    // Get messages from topic
    let messages: Vec<PubSubReceivedMessage> = MESSAGES
        .get(&topic_name_str)
        .map(|entry| {
            entry
                .iter()
                .take(max_messages)
                .map(|m| {
                    let ack_id = generate_ack_id();

                    // Add to pending messages
                    let pending = StoredPendingMessage {
                        ack_id: ack_id.clone(),
                        message: m.clone(),
                        subscription_name: sub_name.clone(),
                        delivery_attempt: 1,
                        ack_deadline: Utc::now() + Duration::seconds(10),
                    };

                    PENDING_MESSAGES
                        .entry(sub_name.clone())
                        .or_insert_with(Vec::new)
                        .push(pending);

                    PubSubReceivedMessage {
                        ack_id,
                        message: PubSubMessage {
                            message_id: m.message_id.clone(),
                            data: m.data.clone(),
                            attributes: m.attributes.clone(),
                            publish_time: m.publish_time,
                            ordering_key: m.ordering_key.clone(),
                        },
                        delivery_attempt: 1,
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    let count = messages.len() as i32;

    PullOutput { messages, count }
}

/// Acknowledge messages
pub async fn acknowledge(project_id: String, subscription_id: String, ack_ids: Vec<String>) -> AcknowledgeOutput {
    let sub_name = subscription_name(&project_id, &subscription_id);

    if let Some(mut entry) = PENDING_MESSAGES.get_mut(&sub_name) {
        entry.retain(|m| !ack_ids.contains(&m.ack_id));
    }

    AcknowledgeOutput { success: true }
}

/// Modify acknowledgement deadline
pub async fn modify_ack_deadline(
    project_id: String,
    subscription_id: String,
    ack_ids: Vec<String>,
    ack_deadline_seconds: i32,
) -> ModifyAckDeadlineOutput {
    let sub_name = subscription_name(&project_id, &subscription_id);

    if let Some(mut entry) = PENDING_MESSAGES.get_mut(&sub_name) {
        for pending in entry.iter_mut() {
            if ack_ids.contains(&pending.ack_id) {
                pending.ack_deadline = Utc::now() + Duration::seconds(ack_deadline_seconds as i64);
            }
        }
    }

    ModifyAckDeadlineOutput { success: true }
}

/// Seek subscription to a point in time or snapshot
pub async fn seek(
    project_id: String,
    subscription_id: String,
    _time: Option<chrono::DateTime<Utc>>,
    _snapshot: Option<String>,
) -> SeekOutput {
    let sub_name = subscription_name(&project_id, &subscription_id);

    // Clear pending messages for the subscription
    if let Some(mut entry) = PENDING_MESSAGES.get_mut(&sub_name) {
        entry.clear();
    }

    SeekOutput { success: true }
}

/// Create a snapshot
pub async fn create_snapshot(
    project_id: String,
    snapshot_id: String,
    subscription_id: String,
    labels: Option<PubSubLabels>,
) -> CreateSnapshotOutput {
    let name = snapshot_name(&project_id, &snapshot_id);
    let sub_name = subscription_name(&project_id, &subscription_id);
    let now = Utc::now();
    let expire_time = now + Duration::days(7);

    // Get topic from subscription
    let topic = if let Some(entry) = SUBSCRIPTIONS.get(&sub_name) {
        topic_name(&entry.project_id, &entry.topic_id)
    } else {
        String::new()
    };

    let stored = StoredSnapshot {
        name: name.clone(),
        project_id,
        snapshot_id,
        subscription_name: sub_name,
        topic: topic.clone(),
        labels: labels.unwrap_or_default(),
        expire_time,
        created_at: now,
    };

    SNAPSHOTS.insert(name.clone(), stored);

    CreateSnapshotOutput {
        snapshot_name: name,
        topic,
        expire_time,
        success: true,
    }
}

/// Delete a snapshot
pub async fn delete_snapshot(project_id: String, snapshot_id: String) -> DeleteSnapshotOutput {
    let name = snapshot_name(&project_id, &snapshot_id);
    let success = SNAPSHOTS.remove(&name).is_some();
    DeleteSnapshotOutput { success }
}

/// List subscriptions for a topic
pub async fn list_topic_subscriptions(
    project_id: String,
    topic_id: String,
    page_size: Option<i32>,
    _page_token: Option<String>,
) -> ListTopicSubscriptionsOutput {
    let page_size = page_size.unwrap_or(100) as usize;

    let subscriptions: Vec<String> = SUBSCRIPTIONS
        .iter()
        .filter(|entry| entry.project_id == project_id && entry.topic_id == topic_id)
        .take(page_size)
        .map(|entry| entry.name.clone())
        .collect();

    ListTopicSubscriptionsOutput {
        subscriptions,
        next_page_token: None,
    }
}

/// Update a topic
pub async fn update_topic(
    project_id: String,
    topic_id: String,
    labels: Option<PubSubLabels>,
    message_retention_duration: Option<String>,
) -> UpdateTopicOutput {
    let name = topic_name(&project_id, &topic_id);
    let mut success = false;

    if let Some(mut entry) = TOPICS.get_mut(&name) {
        if let Some(l) = labels {
            entry.labels = l;
        }
        if let Some(d) = message_retention_duration {
            entry.message_retention_duration = Some(d);
        }
        success = true;
    }

    UpdateTopicOutput { success }
}

/// Update a subscription
pub async fn update_subscription(
    project_id: String,
    subscription_id: String,
    ack_deadline_seconds: Option<i32>,
    labels: Option<PubSubLabels>,
    message_retention_duration: Option<String>,
    push_config: Option<PushConfig>,
) -> UpdateSubscriptionOutput {
    let name = subscription_name(&project_id, &subscription_id);
    let mut success = false;

    if let Some(mut entry) = SUBSCRIPTIONS.get_mut(&name) {
        if let Some(a) = ack_deadline_seconds {
            entry.ack_deadline_seconds = a;
        }
        if let Some(l) = labels {
            entry.labels = l;
        }
        if let Some(d) = message_retention_duration {
            entry.message_retention_duration = Some(d);
        }
        if let Some(p) = push_config {
            entry.push_config = Some(p);
        }
        success = true;
    }

    UpdateSubscriptionOutput { success }
}

// Utility functions for testing
#[cfg(test)]
pub fn clear_all_data() {
    TOPICS.clear();
    SUBSCRIPTIONS.clear();
    MESSAGES.clear();
    PENDING_MESSAGES.clear();
    SNAPSHOTS.clear();
}
