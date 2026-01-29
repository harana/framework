#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::json;
    use std::collections::HashMap;

    fn create_test_variations() -> Vec<HashMap<String, Value>> {
        vec![
            {
                let mut v = HashMap::new();
                v.insert("name".to_string(), json!("off"));
                v.insert("value".to_string(), json!(false));
                v
            },
            {
                let mut v = HashMap::new();
                v.insert("name".to_string(), json!("on"));
                v.insert("value".to_string(), json!(true));
                v
            },
        ]
    }

    #[tokio::test]
    async fn test_create_flag() {
        let variations = create_test_variations();
        let result = create_flag(
            "test-flag-create",
            variations,
            "Test Flag",
            Some(0),
            Some(vec!["test".to_string()]),
            Some(false),
            Some("A test flag"),
        )
        .await
        .unwrap();

        assert!(result.success);
        assert!(!result.flag_id.is_empty());
        assert_eq!(result.key, "test-flag-create");
    }

    #[tokio::test]
    async fn test_get_flag() {
        let variations = create_test_variations();
        let create_result = create_flag(
            "test-flag-get",
            variations,
            "Get Test Flag",
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        let get_result = get_flag(&create_result.flag_id).await.unwrap();
        assert_eq!(get_result.flag.key, "test-flag-get");
        assert_eq!(get_result.flag.name, "Get Test Flag");
    }

    #[tokio::test]
    async fn test_update_flag() {
        let variations = create_test_variations();
        let create_result = create_flag(
            "test-flag-update",
            variations,
            "Update Test Flag",
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        let update_result = update_flag(
            &create_result.flag_id,
            Some("Updated Name"),
            Some(true),
            Some("Updated description"),
            None,
            Some(vec!["updated".to_string()]),
        )
        .await
        .unwrap();

        assert!(update_result.success);

        let get_result = get_flag(&create_result.flag_id).await.unwrap();
        assert_eq!(get_result.flag.name, "Updated Name");
        assert!(get_result.flag.enabled);
        assert_eq!(get_result.flag.description, "Updated description");
    }

    #[tokio::test]
    async fn test_toggle_flag() {
        let variations = create_test_variations();
        let create_result = create_flag(
            "test-flag-toggle",
            variations,
            "Toggle Test Flag",
            None,
            None,
            Some(false),
            None,
        )
        .await
        .unwrap();

        let toggle_result = toggle_flag(&create_result.flag_id, true).await.unwrap();
        assert!(toggle_result.success);
        assert!(toggle_result.enabled);

        let toggle_result = toggle_flag(&create_result.flag_id, false).await.unwrap();
        assert!(!toggle_result.enabled);
    }

    #[tokio::test]
    async fn test_delete_flag() {
        let variations = create_test_variations();
        let create_result = create_flag(
            "test-flag-delete",
            variations,
            "Delete Test Flag",
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        let delete_result = delete_flag(&create_result.flag_id).await.unwrap();
        assert!(delete_result.success);

        let get_result = get_flag(&create_result.flag_id).await;
        assert!(get_result.is_err());
    }

    #[tokio::test]
    async fn test_list_flags() {
        // Create a few flags
        for i in 0..3 {
            let variations = create_test_variations();
            create_flag(
                &format!("list-flag-{}", i),
                variations,
                &format!("List Flag {}", i),
                None,
                Some(vec!["list-test".to_string()]),
                Some(i % 2 == 0),
                None,
            )
            .await
            .unwrap();
        }

        let list_result = list_flags(None, Some(10), Some(vec!["list-test".to_string()]), None)
            .await
            .unwrap();

        assert!(list_result.total >= 2);
    }

    #[tokio::test]
    async fn test_list_flags_filter_enabled() {
        let variations = create_test_variations();
        create_flag(
            "enabled-flag",
            variations.clone(),
            "Enabled Flag",
            None,
            Some(vec!["enabled-test".to_string()]),
            Some(true),
            None,
        )
        .await
        .unwrap();

        create_flag(
            "disabled-flag",
            variations,
            "Disabled Flag",
            None,
            Some(vec!["enabled-test".to_string()]),
            Some(false),
            None,
        )
        .await
        .unwrap();

        let enabled_list = list_flags(Some(true), None, Some(vec!["enabled-test".to_string()]), None)
            .await
            .unwrap();

        assert!(enabled_list.flags.iter().all(|f| f.enabled));
    }

    #[tokio::test]
    async fn test_evaluate_flag_disabled() {
        let variations = create_test_variations();
        let create_result = create_flag(
            "eval-disabled-flag",
            variations,
            "Eval Disabled Flag",
            Some(0),
            None,
            Some(false),
            None,
        )
        .await
        .unwrap();

        let eval_result = evaluate_flag(&create_result.flag_id, Some("user123"), None)
            .await
            .unwrap();

        assert!(!eval_result.enabled);
        assert_eq!(eval_result.reason, "Flag is disabled");
    }

    #[tokio::test]
    async fn test_evaluate_flag_enabled() {
        let variations = create_test_variations();
        let create_result = create_flag(
            "eval-enabled-flag",
            variations,
            "Eval Enabled Flag",
            Some(1), // Default to "on" variation
            None,
            Some(true),
            None,
        )
        .await
        .unwrap();

        let eval_result = evaluate_flag(&create_result.flag_id, Some("user123"), None)
            .await
            .unwrap();

        assert!(eval_result.enabled);
        assert_eq!(eval_result.variation.name, "on");
    }

    #[tokio::test]
    async fn test_targeting_rule() {
        let variations = create_test_variations();
        let create_result = create_flag(
            "targeting-flag",
            variations,
            "Targeting Flag",
            Some(0),
            None,
            Some(true),
            None,
        )
        .await
        .unwrap();

        // Create a targeting rule
        let conditions = vec![{
            let mut c = HashMap::new();
            c.insert("attribute".to_string(), json!("plan"));
            c.insert("operator".to_string(), json!("eq"));
            c.insert("value".to_string(), json!("premium"));
            c
        }];

        let rule_result = create_targeting_rule(
            1, // "on" variation
            "Premium users",
            conditions,
            &create_result.flag_id,
            Some(0),
        )
        .await
        .unwrap();

        assert!(rule_result.success);

        // Evaluate with matching context
        let mut context = HashMap::new();
        context.insert("plan".to_string(), json!("premium"));

        let eval_result = evaluate_flag(&create_result.flag_id, Some("user123"), Some(context))
            .await
            .unwrap();

        assert!(eval_result.enabled);
        assert_eq!(eval_result.variation.name, "on");
        assert!(eval_result.reason.contains("Premium users"));
    }

    #[tokio::test]
    async fn test_targeting_rule_no_match() {
        let variations = create_test_variations();
        let create_result = create_flag(
            "targeting-no-match-flag",
            variations,
            "Targeting No Match Flag",
            Some(0),
            None,
            Some(true),
            None,
        )
        .await
        .unwrap();

        let conditions = vec![{
            let mut c = HashMap::new();
            c.insert("attribute".to_string(), json!("plan"));
            c.insert("operator".to_string(), json!("eq"));
            c.insert("value".to_string(), json!("enterprise"));
            c
        }];

        create_targeting_rule(1, "Enterprise users", conditions, &create_result.flag_id, None)
            .await
            .unwrap();

        // Evaluate with non-matching context
        let mut context = HashMap::new();
        context.insert("plan".to_string(), json!("free"));

        let eval_result = evaluate_flag(&create_result.flag_id, Some("user123"), Some(context))
            .await
            .unwrap();

        // Should fall back to default variation
        assert_eq!(eval_result.variation.name, "off");
    }

    #[tokio::test]
    async fn test_update_targeting_rule() {
        let variations = create_test_variations();
        let create_result = create_flag(
            "update-rule-flag",
            variations,
            "Update Rule Flag",
            None,
            None,
            Some(true),
            None,
        )
        .await
        .unwrap();

        let conditions = vec![{
            let mut c = HashMap::new();
            c.insert("attribute".to_string(), json!("country"));
            c.insert("operator".to_string(), json!("eq"));
            c.insert("value".to_string(), json!("US"));
            c
        }];

        let rule_result = create_targeting_rule(1, "US users", conditions, &create_result.flag_id, None)
            .await
            .unwrap();

        let update_result = update_targeting_rule(
            &rule_result.rule_id,
            Some("Updated rule name"),
            None,
            None,
            Some(10),
        )
        .await
        .unwrap();

        assert!(update_result.success);
    }

    #[tokio::test]
    async fn test_delete_targeting_rule() {
        let variations = create_test_variations();
        let create_result = create_flag(
            "delete-rule-flag",
            variations,
            "Delete Rule Flag",
            None,
            None,
            Some(true),
            None,
        )
        .await
        .unwrap();

        let conditions = vec![{
            let mut c = HashMap::new();
            c.insert("attribute".to_string(), json!("region"));
            c.insert("operator".to_string(), json!("eq"));
            c.insert("value".to_string(), json!("EU"));
            c
        }];

        let rule_result = create_targeting_rule(1, "EU users", conditions, &create_result.flag_id, None)
            .await
            .unwrap();

        let delete_result = delete_targeting_rule(&rule_result.rule_id).await.unwrap();
        assert!(delete_result.success);
    }

    #[tokio::test]
    async fn test_percentage_rollout() {
        let variations = create_test_variations();
        let create_result = create_flag(
            "rollout-flag",
            variations,
            "Rollout Flag",
            Some(0),
            None,
            Some(true),
            None,
        )
        .await
        .unwrap();

        let mut percentages = HashMap::new();
        percentages.insert("0".to_string(), json!(50)); // 50% get variation 0
        percentages.insert("1".to_string(), json!(50)); // 50% get variation 1

        let rollout_result = create_rollout(&create_result.flag_id, percentages, Some("test-seed"))
            .await
            .unwrap();

        assert!(rollout_result.success);
        assert!(!rollout_result.rollout_id.is_empty());
    }

    #[tokio::test]
    async fn test_update_rollout() {
        let variations = create_test_variations();
        let create_result = create_flag(
            "update-rollout-flag",
            variations,
            "Update Rollout Flag",
            None,
            None,
            Some(true),
            None,
        )
        .await
        .unwrap();

        let mut percentages = HashMap::new();
        percentages.insert("0".to_string(), json!(90));
        percentages.insert("1".to_string(), json!(10));

        let rollout_result = create_rollout(&create_result.flag_id, percentages, None)
            .await
            .unwrap();

        let mut new_percentages = HashMap::new();
        new_percentages.insert("0".to_string(), json!(50));
        new_percentages.insert("1".to_string(), json!(50));

        let update_result = update_rollout(&rollout_result.rollout_id, new_percentages)
            .await
            .unwrap();

        assert!(update_result.success);
    }

    #[tokio::test]
    async fn test_evaluation_count() {
        let variations = create_test_variations();
        let create_result = create_flag(
            "count-flag",
            variations,
            "Count Flag",
            Some(1),
            None,
            Some(true),
            None,
        )
        .await
        .unwrap();

        // Evaluate multiple times
        for i in 0..5 {
            evaluate_flag(&create_result.flag_id, Some(&format!("user{}", i)), None)
                .await
                .unwrap();
        }

        let count_result = get_evaluation_count(&create_result.flag_id, None, None)
            .await
            .unwrap();

        assert!(count_result.total_evaluations >= 5);
    }

    #[tokio::test]
    async fn test_create_environment() {
        let result = create_environment("Production", "production", Some("Production environment"))
            .await
            .unwrap();

        assert!(result.success);
        assert!(!result.environment_id.is_empty());
        assert_eq!(result.key, "production");
    }

    #[tokio::test]
    async fn test_clone_flag() {
        // Create environments
        create_environment("Staging", "staging", None).await.unwrap();
        create_environment("Prod", "prod", None).await.unwrap();

        let variations = create_test_variations();
        let create_result = create_flag(
            "clone-source-flag",
            variations,
            "Clone Source Flag",
            None,
            None,
            Some(true),
            None,
        )
        .await
        .unwrap();

        let clone_result = clone_flag("staging", &create_result.flag_id, "prod")
            .await
            .unwrap();

        assert!(clone_result.success);
        assert_ne!(clone_result.flag_id, create_result.flag_id);
    }

    #[tokio::test]
    async fn test_archive_and_restore_flag() {
        let variations = create_test_variations();
        let create_result = create_flag(
            "archive-flag",
            variations,
            "Archive Flag",
            None,
            None,
            Some(true),
            None,
        )
        .await
        .unwrap();

        // Archive
        let archive_result = archive_flag(&create_result.flag_id).await.unwrap();
        assert!(archive_result.success);
        assert!(archive_result.archived);

        // Verify archived flag returns disabled on evaluation
        let eval_result = evaluate_flag(&create_result.flag_id, Some("user123"), None)
            .await
            .unwrap();
        assert!(!eval_result.enabled);

        // Restore
        let restore_result = restore_flag(&create_result.flag_id).await.unwrap();
        assert!(restore_result.success);
        assert!(!restore_result.archived);
    }

    #[tokio::test]
    async fn test_rule_conditions_operators() {
        let variations = create_test_variations();
        let create_result = create_flag(
            "operators-flag",
            variations,
            "Operators Flag",
            Some(0),
            None,
            Some(true),
            None,
        )
        .await
        .unwrap();

        // Test "in" operator
        let conditions = vec![{
            let mut c = HashMap::new();
            c.insert("attribute".to_string(), json!("tier"));
            c.insert("operator".to_string(), json!("in"));
            c.insert("value".to_string(), json!(["gold", "platinum"]));
            c
        }];

        create_targeting_rule(1, "Premium tiers", conditions, &create_result.flag_id, None)
            .await
            .unwrap();

        let mut context = HashMap::new();
        context.insert("tier".to_string(), json!("gold"));

        let eval_result = evaluate_flag(&create_result.flag_id, Some("user123"), Some(context))
            .await
            .unwrap();

        assert_eq!(eval_result.variation.name, "on");
    }

    #[tokio::test]
    async fn test_get_flag_not_found() {
        let result = get_flag("non-existent-flag").await;
        assert!(result.is_err());
    }
}
