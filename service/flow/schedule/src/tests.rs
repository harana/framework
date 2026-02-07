#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_create_schedule() {
        let result = create_schedule(
            "0 0 0 * * *",  // 6-field cron: sec min hour day month dow
            "Daily Task",
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        assert!(result.success);
        assert_eq!(result.name, "Daily Task");
        assert!(!result.schedule_id.is_empty());
        assert!(result.next_run > 0);
    }

    #[tokio::test]
    async fn test_create_schedule_with_timezone() {
        let result = create_schedule(
            "0 0 9 * * *",  // 9 AM daily
            "Morning Task",
            None,
            Some("America/New_York"),
            None,
            None,
        )
        .await
        .unwrap();

        assert!(result.success);
        assert!(!result.schedule_id.is_empty());
    }

    #[tokio::test]
    async fn test_create_schedule_invalid_cron() {
        let result = create_schedule(
            "invalid cron",
            "Bad Task",
            None,
            None,
            None,
            None,
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_schedule() {
        let created = create_schedule(
            "0 0 12 * * *",
            "Noon Task",
            None,
            Some("UTC"),
            Some(true),
            Some("A test schedule"),
        )
        .await
        .unwrap();

        let result = get_schedule(&created.schedule_id).await.unwrap();
        
        assert_eq!(result.name, "Noon Task");
        assert_eq!(result.cron_expression, "0 0 12 * * *");
        assert_eq!(result.timezone, "UTC");
        assert!(result.enabled);
        assert!(result.next_run > 0);
    }

    #[tokio::test]
    async fn test_update_schedule() {
        let created = create_schedule(
            "0 0 0 * * *",
            "Original Name",
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        let result = update_schedule(
            &created.schedule_id,
            Some("0 0 6 * * *"),
            Some("Updated Name"),
            None,
            None,
            None,
            Some("Updated description"),
        )
        .await
        .unwrap();

        assert!(result.success);

        let updated = get_schedule(&created.schedule_id).await.unwrap();
        assert_eq!(updated.name, "Updated Name");
        assert_eq!(updated.cron_expression, "0 0 6 * * *");
    }

    #[tokio::test]
    async fn test_enable_disable_schedule() {
        let created = create_schedule(
            "0 0 0 * * *",
            "Toggle Task",
            None,
            None,
            Some(true),
            None,
        )
        .await
        .unwrap();

        // Disable
        let disabled = disable_schedule(&created.schedule_id).await.unwrap();
        assert!(!disabled.enabled);
        assert!(disabled.success);

        let schedule = get_schedule(&created.schedule_id).await.unwrap();
        assert!(!schedule.enabled);

        // Enable
        let enabled = enable_schedule(&created.schedule_id).await.unwrap();
        assert!(enabled.enabled);
        assert!(enabled.success);

        let schedule = get_schedule(&created.schedule_id).await.unwrap();
        assert!(schedule.enabled);
    }

    #[tokio::test]
    async fn test_delete_schedule() {
        let created = create_schedule(
            "0 0 0 * * *",
            "Temp Task",
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        let result = delete_schedule(&created.schedule_id).await.unwrap();
        assert!(result.success);

        // Should not be found after deletion
        let get_result = get_schedule(&created.schedule_id).await;
        assert!(get_result.is_err());
    }

    #[tokio::test]
    async fn test_list_schedules() {
        // Create multiple schedules
        create_schedule("0 0 0 * * *", "Task A", None, None, Some(true), None)
            .await
            .unwrap();
        create_schedule("0 0 6 * * *", "Task B", None, None, Some(false), None)
            .await
            .unwrap();

        let result = list_schedules(None, None, None, None).await.unwrap();
        assert!(result.total >= 2);
        assert!(!result.schedules.is_empty());
    }

    #[tokio::test]
    async fn test_list_schedules_filtered_enabled() {
        create_schedule("0 0 0 * * *", "Enabled Task", None, None, Some(true), None)
            .await
            .unwrap();
        create_schedule("0 0 6 * * *", "Disabled Task", None, None, Some(false), None)
            .await
            .unwrap();

        let result = list_schedules(None, None, None, Some(true)).await.unwrap();
        assert!(result.total >= 1);
    }

    #[tokio::test]
    async fn test_list_schedules_with_search() {
        create_schedule("0 0 0 * * *", "Search Test Alpha", None, None, None, None)
            .await
            .unwrap();
        create_schedule("0 0 6 * * *", "Search Test Beta", None, None, None, None)
            .await
            .unwrap();

        let result = list_schedules(None, Some("Alpha"), None, None)
            .await
            .unwrap();
        assert!(result.total >= 1);
    }

    #[tokio::test]
    async fn test_validate_cron() {
        // Valid cron - 6 field format
        let result = validate_cron("0 0 0 * * *", None).await.unwrap();
        assert!(result.valid);
        assert!(result.error.is_empty());
        assert!(result.next_run > 0);

        // Invalid cron
        let result = validate_cron("not a cron", None).await.unwrap();
        assert!(!result.valid);
        assert!(!result.error.is_empty());
    }

    #[tokio::test]
    async fn test_get_next_run() {
        let created = create_schedule(
            "0 0 */2 * * *",  // Every 2 hours
            "Hourly Task",
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        let result = get_next_run(&created.schedule_id, Some(5)).await.unwrap();
        assert_eq!(result.next_runs.len(), 5);
        assert_eq!(result.schedule_id, created.schedule_id);

        // Verify times are increasing
        for i in 1..result.next_runs.len() {
            assert!(result.next_runs[i] > result.next_runs[i - 1]);
        }
    }

    #[tokio::test]
    async fn test_pause_resume_schedule() {
        let created = create_schedule(
            "0 0 0 * * *",
            "Pausable Task",
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        // Pause
        let paused = pause_schedule(&created.schedule_id, Some(2147483647))
            .await
            .unwrap();
        assert!(paused.paused);
        assert!(paused.success);

        // Resume
        let resumed = resume_schedule(&created.schedule_id).await.unwrap();
        assert!(!resumed.paused);
        assert!(resumed.success);
        assert!(resumed.next_run > 0);
    }

    #[tokio::test]
    async fn test_trigger_schedule() {
        let created = create_schedule(
            "0 0 0 * * *",
            "Triggered Task",
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        let result = trigger_schedule(&created.schedule_id).await.unwrap();
        assert!(result.success);
        assert!(!result.execution_id.is_empty());
        assert!(result.triggered_at > 0);
    }

    #[tokio::test]
    async fn test_create_one_time() {
        let future_time = (Utc::now().timestamp() + 3600) as i32;
        
        let result = create_one_time(
            "One Time Task",
            future_time,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        assert!(result.success);
        assert_eq!(result.run_at, future_time);
        assert!(!result.schedule_id.is_empty());
    }

    #[tokio::test]
    async fn test_create_interval() {
        let result = create_interval(
            "Interval Task",
            3600,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        assert!(result.success);
        assert!(result.next_run > 0);
        assert!(!result.schedule_id.is_empty());
    }

    #[tokio::test]
    async fn test_bulk_enable() {
        let s1 = create_schedule("0 0 0 * * *", "Task 1", None, None, Some(false), None)
            .await
            .unwrap();
        let s2 = create_schedule("0 0 6 * * *", "Task 2", None, None, Some(false), None)
            .await
            .unwrap();

        let result = bulk_enable(vec![s1.schedule_id.clone(), s2.schedule_id.clone()])
            .await
            .unwrap();

        assert_eq!(result.successful, 2);
        assert_eq!(result.failed, 0);
    }

    #[tokio::test]
    async fn test_bulk_disable() {
        let s1 = create_schedule("0 0 0 * * *", "Task 1", None, None, Some(true), None)
            .await
            .unwrap();
        let s2 = create_schedule("0 0 6 * * *", "Task 2", None, None, Some(true), None)
            .await
            .unwrap();

        let result = bulk_disable(vec![s1.schedule_id.clone(), s2.schedule_id.clone()])
            .await
            .unwrap();

        assert_eq!(result.successful, 2);
        assert_eq!(result.failed, 0);
    }

    #[tokio::test]
    async fn test_get_schedule_history() {
        let created = create_schedule(
            "0 0 0 * * *",
            "History Task",
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        // Trigger to create history
        trigger_schedule(&created.schedule_id).await.unwrap();
        trigger_schedule(&created.schedule_id).await.unwrap();

        let result = get_schedule_history(&created.schedule_id, None, None, None)
            .await
            .unwrap();

        assert_eq!(result.total, 2);
        assert_eq!(result.executions.len(), 2);
    }

    #[tokio::test]
    async fn test_get_schedule_stats() {
        let created = create_schedule(
            "0 0 0 * * *",
            "Stats Task",
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        // Trigger to create stats
        trigger_schedule(&created.schedule_id).await.unwrap();

        let result = get_schedule_stats(&created.schedule_id, None, None)
            .await
            .unwrap();

        assert_eq!(result.total_executions, 1);
    }

    #[tokio::test]
    async fn test_update_action() {
        let created = create_schedule(
            "0 0 0 * * *",
            "Action Task",
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        let mut action_config = HashMap::new();
        action_config.insert("url".to_string(), serde_json::Value::String("https://example.com".to_string()));

        let result = update_action(
            &created.schedule_id,
            action_config,
            "Webhook",
        )
        .await
        .unwrap();

        assert!(result.success);
    }
}
