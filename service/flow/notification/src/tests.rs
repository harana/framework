#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test_send_push() {
        let result = send_push(
            "user123",
            "Test Title",
            "Test Body",
            None,
            Some(5),
            Some("default"),
            Some("high"),
        )
        .await
        .unwrap();

        assert!(result.success);
        assert!(!result.notification_id.is_empty());

        // Verify we can get the notification
        let status = status(&result.notification_id).await.unwrap();
        assert_eq!(status.status, "sent");
        assert!(status.sent_at.is_some());
        assert!(status.delivered_at.is_some());
    }

    #[tokio::test]
    async fn test_send_sms() {
        let result = send_sms("+1234567890", "Test SMS", Some("TestSender"))
            .await
            .unwrap();

        assert!(result.success);
        assert!(!result.message_id.is_empty());
    }

    #[tokio::test]
    async fn test_send_in_app() {
        let mut data = HashMap::new();
        data.insert("key".to_string(), json!("value"));

        let result = send_in_app(
            "user456",
            "In-App Title",
            "In-App Message",
            Some("alert"),
            Some("/action/path"),
            Some(data),
        )
        .await
        .unwrap();

        assert!(result.success);
        assert!(!result.notification_id.is_empty());
    }

    #[tokio::test]
    async fn test_send_bulk() {
        let users = vec!["user1", "user2", "user3"];
        let result = send_bulk(users, "Bulk Title", "Bulk Body", Some("push"), None)
            .await
            .unwrap();

        assert_eq!(result.total, 3);
        assert_eq!(result.successful, 3);
        assert_eq!(result.failed, 0);
    }

    #[tokio::test]
    async fn test_mark_as_read() {
        let notification = send_push("user789", "Test", "Body", None, None, None, None)
            .await
            .unwrap();

        let result = mark_as_read(&notification.notification_id)
            .await
            .unwrap();
        assert!(result.success);

        let status = status(&notification.notification_id).await.unwrap();
        assert_eq!(status.status, "read");
        assert!(status.read_at.is_some());
    }

    #[tokio::test]
    async fn test_list_notifications() {
        let user_id = "list_test_user";

        // Send multiple notifications
        send_push(user_id, "Title 1", "Body 1", None, None, None, None)
            .await
            .unwrap();
        send_push(user_id, "Title 2", "Body 2", None, None, None, None)
            .await
            .unwrap();
        send_push(user_id, "Title 3", "Body 3", None, None, None, None)
            .await
            .unwrap();

        let result = list(user_id, None, Some(10), None).await.unwrap();
        assert_eq!(result.total, 3);
        assert_eq!(result.unread_count, 3);
        assert_eq!(result.notifications.len(), 3);
    }

    #[tokio::test]
    async fn test_list_unread_only() {
        let user_id = "unread_test_user";

        let notif1 = send_push(user_id, "Title 1", "Body 1", None, None, None, None)
            .await
            .unwrap();
        let _notif2 = send_push(user_id, "Title 2", "Body 2", None, None, None, None)
            .await
            .unwrap();

        // Mark one as read
        mark_as_read(&notif1.notification_id).await.unwrap();

        let result = list(user_id, Some(true), Some(10), None).await.unwrap();
        assert_eq!(result.total, 1);
        assert_eq!(result.unread_count, 1);
    }

    #[tokio::test]
    async fn test_list_with_pagination() {
        let user_id = "pagination_test_user";

        // Send 5 notifications
        for i in 0..5 {
            send_push(user_id, &format!("Title {}", i), "Body", None, None, None, None)
                .await
                .unwrap();
        }

        let result = list(user_id, None, Some(2), Some(1)).await.unwrap();
        assert_eq!(result.total, 2);
        assert_eq!(result.notifications.len(), 2);
    }

    #[tokio::test]
    async fn test_delete_notification() {
        let notification = send_push("delete_user", "Test", "Body", None, None, None, None)
            .await
            .unwrap();

        let delete_result = delete(&notification.notification_id).await.unwrap();
        assert!(delete_result.success);

        // Verify it's deleted
        let status_result = status(&notification.notification_id).await;
        assert!(status_result.is_err());
    }

    #[tokio::test]
    async fn test_register_device() {
        let result = register_device("device_user", "token123", "ios")
            .await
            .unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_unregister_device() {
        let token = "token456";
        register_device("device_user2", token, "android")
            .await
            .unwrap();

        let result = unregister_device(token).await.unwrap();
        assert!(result.success);

        // Verify it's unregistered
        let unregister_again = unregister_device(token).await;
        assert!(unregister_again.is_err());
    }

    #[tokio::test]
    async fn test_notification_status_not_found() {
        let result = status("non_existent_id").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Notification not found");
    }

    #[tokio::test]
    async fn test_mark_as_read_not_found() {
        let result = mark_as_read("non_existent_id").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Notification not found");
    }

    #[tokio::test]
    async fn test_notification_with_data() {
        let mut data = HashMap::new();
        data.insert("custom_field".to_string(), json!("custom_value"));
        data.insert("number".to_string(), json!(42));

        let _result = send_push(
            "data_user",
            "Title",
            "Body",
            Some(data.clone()),
            None,
            None,
            None,
        )
        .await
        .unwrap();

        let notifications = list("data_user", None, None, None).await.unwrap();
        assert_eq!(notifications.notifications.len(), 1);

        let notif = &notifications.notifications[0];
        assert!(notif.contains_key("data"));
    }

    #[tokio::test]
    async fn test_send_bulk_with_data() {
        let mut data = HashMap::new();
        data.insert("campaign_id".to_string(), json!("campaign123"));

        let users = vec!["bulk_user1", "bulk_user2"];
        let result = send_bulk(
            users,
            "Campaign Title",
            "Campaign Body",
            Some("in_app"),
            Some(data),
        )
        .await
        .unwrap();

        assert_eq!(result.total, 2);
        assert_eq!(result.successful, 2);
    }

    #[tokio::test]
    async fn test_multiple_devices_per_user() {
        let user = "multi_device_user";

        register_device(user, "ios_token", "ios").await.unwrap();
        register_device(user, "android_token", "android")
            .await
            .unwrap();

        // Both devices should be registered
        // We can verify by unregistering them
        assert!(unregister_device("ios_token").await.is_ok());
        assert!(unregister_device("android_token").await.is_ok());
    }
}
