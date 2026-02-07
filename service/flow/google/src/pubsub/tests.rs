use super::*;

#[tokio::test]
async fn test_create_topic() {
    clear_all_data();

    let result = create_topic(
        "my-project".to_string(),
        "my-topic".to_string(),
        None,
        Some("604800s".to_string()),
        None,
    )
    .await;

    assert!(result.success);
    assert_eq!(result.topic_name, "projects/my-project/topics/my-topic");
}

#[tokio::test]
async fn test_get_topic() {
    clear_all_data();

    create_topic("my-project".to_string(), "get-topic".to_string(), None, None, None).await;

    let result = get_topic("my-project".to_string(), "get-topic".to_string()).await;

    assert_eq!(result.topic_name, "projects/my-project/topics/get-topic");
}

#[tokio::test]
async fn test_delete_topic() {
    clear_all_data();

    create_topic("my-project".to_string(), "delete-topic".to_string(), None, None, None).await;

    let result = delete_topic("my-project".to_string(), "delete-topic".to_string()).await;
    assert!(result.success);
}

#[tokio::test]
async fn test_list_topics() {
    clear_all_data();

    for i in 0..5 {
        create_topic("my-project".to_string(), format!("topic-{}", i), None, None, None).await;
    }

    let result = list_topics("my-project".to_string(), Some(3), None).await;
    assert_eq!(result.topics.len(), 3);
}

#[tokio::test]
async fn test_publish_message() {
    clear_all_data();

    create_topic("my-project".to_string(), "publish-topic".to_string(), None, None, None).await;

    let result = publish(
        "my-project".to_string(),
        "publish-topic".to_string(),
        "Hello, World!".to_string(),
        None,
        None,
    )
    .await;

    assert!(result.success);
    assert!(!result.message_id.is_empty());
}

#[tokio::test]
async fn test_publish_batch() {
    clear_all_data();

    create_topic("my-project".to_string(), "batch-topic".to_string(), None, None, None).await;

    let messages = vec![
        PubSubPublishMessage {
            data: "Message 1".to_string(),
            attributes: None,
            ordering_key: None,
        },
        PubSubPublishMessage {
            data: "Message 2".to_string(),
            attributes: None,
            ordering_key: None,
        },
        PubSubPublishMessage {
            data: "Message 3".to_string(),
            attributes: None,
            ordering_key: None,
        },
    ];

    let result = publish_batch("my-project".to_string(), "batch-topic".to_string(), messages).await;

    assert_eq!(result.message_ids.len(), 3);
    assert_eq!(result.success_count, 3);
    assert_eq!(result.failure_count, 0);
}

#[tokio::test]
async fn test_subscription_lifecycle() {
    clear_all_data();

    // Create topic first
    create_topic("my-project".to_string(), "sub-topic".to_string(), None, None, None).await;

    // Create subscription
    let create_result = create_subscription(
        "my-project".to_string(),
        "my-subscription".to_string(),
        "sub-topic".to_string(),
        Some(30),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await;

    assert!(create_result.success);
    assert_eq!(
        create_result.subscription_name,
        "projects/my-project/subscriptions/my-subscription"
    );

    // Get subscription
    let get_result = get_subscription("my-project".to_string(), "my-subscription".to_string()).await;

    assert_eq!(get_result.ack_deadline_seconds, 30);

    // Delete subscription
    let delete_result = delete_subscription("my-project".to_string(), "my-subscription".to_string()).await;

    assert!(delete_result.success);
}

#[tokio::test]
async fn test_pull_and_acknowledge() {
    clear_all_data();

    // Setup
    create_topic("my-project".to_string(), "pull-topic".to_string(), None, None, None).await;

    create_subscription(
        "my-project".to_string(),
        "pull-subscription".to_string(),
        "pull-topic".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await;

    // Publish messages
    publish(
        "my-project".to_string(),
        "pull-topic".to_string(),
        "Test message 1".to_string(),
        None,
        None,
    )
    .await;

    publish(
        "my-project".to_string(),
        "pull-topic".to_string(),
        "Test message 2".to_string(),
        None,
        None,
    )
    .await;

    // Pull messages
    let pull_result = pull(
        "my-project".to_string(),
        "pull-subscription".to_string(),
        Some(10),
        None,
    )
    .await;

    assert_eq!(pull_result.count, 2);
    assert_eq!(pull_result.messages.len(), 2);

    // Acknowledge messages
    let ack_ids: Vec<String> = pull_result.messages.iter().map(|m| m.ack_id.clone()).collect();

    let ack_result = acknowledge("my-project".to_string(), "pull-subscription".to_string(), ack_ids).await;

    assert!(ack_result.success);
}

#[tokio::test]
async fn test_modify_ack_deadline() {
    clear_all_data();

    create_topic("my-project".to_string(), "ack-topic".to_string(), None, None, None).await;

    create_subscription(
        "my-project".to_string(),
        "ack-subscription".to_string(),
        "ack-topic".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await;

    publish(
        "my-project".to_string(),
        "ack-topic".to_string(),
        "Test".to_string(),
        None,
        None,
    )
    .await;

    let pull_result = pull("my-project".to_string(), "ack-subscription".to_string(), Some(1), None).await;

    let ack_id = pull_result.messages[0].ack_id.clone();

    let result = modify_ack_deadline(
        "my-project".to_string(),
        "ack-subscription".to_string(),
        vec![ack_id],
        60,
    )
    .await;

    assert!(result.success);
}

#[tokio::test]
async fn test_snapshot_lifecycle() {
    clear_all_data();

    // Setup
    create_topic("my-project".to_string(), "snap-topic".to_string(), None, None, None).await;

    create_subscription(
        "my-project".to_string(),
        "snap-subscription".to_string(),
        "snap-topic".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await;

    // Create snapshot
    let create_result = create_snapshot(
        "my-project".to_string(),
        "my-snapshot".to_string(),
        "snap-subscription".to_string(),
        None,
    )
    .await;

    assert!(create_result.success);
    assert_eq!(create_result.snapshot_name, "projects/my-project/snapshots/my-snapshot");

    // Delete snapshot
    let delete_result = delete_snapshot("my-project".to_string(), "my-snapshot".to_string()).await;

    assert!(delete_result.success);
}

#[tokio::test]
async fn test_seek() {
    clear_all_data();

    create_topic("my-project".to_string(), "seek-topic".to_string(), None, None, None).await;

    create_subscription(
        "my-project".to_string(),
        "seek-subscription".to_string(),
        "seek-topic".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await;

    let result = seek(
        "my-project".to_string(),
        "seek-subscription".to_string(),
        Some(Utc::now()),
        None,
    )
    .await;

    assert!(result.success);
}

#[tokio::test]
async fn test_update_topic() {
    clear_all_data();

    create_topic("my-project".to_string(), "update-topic".to_string(), None, None, None).await;

    let result = update_topic(
        "my-project".to_string(),
        "update-topic".to_string(),
        Some(PubSubLabels {
            labels: vec![PubSubLabel {
                key: "env".to_string(),
                value: "production".to_string(),
            }],
        }),
        Some("172800s".to_string()),
    )
    .await;

    assert!(result.success);
}

#[tokio::test]
async fn test_list_subscriptions() {
    clear_all_data();

    create_topic("my-project".to_string(), "list-topic".to_string(), None, None, None).await;

    for i in 0..3 {
        create_subscription(
            "my-project".to_string(),
            format!("subscription-{}", i),
            "list-topic".to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await;
    }

    let result = list_subscriptions("my-project".to_string(), None, None).await;
    assert_eq!(result.subscriptions.len(), 3);
}
