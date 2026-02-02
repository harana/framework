#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::json;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_start_workflow() {
        let result = start("test-workflow", None, Some(r#"{"data": "test"}"#)).await;
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.success);
        assert!(!output.execution_id.is_empty());
        assert_eq!(output.status, "running");
    }

    #[tokio::test]
    async fn test_start_and_get_status() {
        let start_result = start("test-workflow-2", None, None).await.unwrap();
        let execution_id = &start_result.execution_id;
        
        let status_result = get_status(execution_id).await;
        assert!(status_result.is_ok());
        
        let status = status_result.unwrap();
        assert_eq!(status.status, "running");
        assert_eq!(status.current_step, "start");
        assert_eq!(status.progress, 0.1);
    }

    #[tokio::test]
    async fn test_pause_and_resume() {
        let start_result = start("test-workflow-3", None, None).await.unwrap();
        let execution_id = &start_result.execution_id;
        
        // Pause the workflow
        let pause_result = pause(execution_id, Some("Testing pause")).await;
        assert!(pause_result.is_ok());
        assert!(pause_result.unwrap().success);
        
        // Verify paused status
        let status = get_status(execution_id).await.unwrap();
        assert_eq!(status.status, "paused");
        
        // Resume the workflow
        let resume_result = resume(execution_id, None).await;
        assert!(resume_result.is_ok());
        assert!(resume_result.unwrap().success);
        
        // Verify running status
        let status = get_status(execution_id).await.unwrap();
        assert_eq!(status.status, "running");
    }

    #[tokio::test]
    async fn test_cancel_workflow() {
        let start_result = start("test-workflow-4", None, None).await.unwrap();
        let execution_id = &start_result.execution_id;
        
        let cancel_result = cancel(execution_id, Some("User requested cancellation")).await;
        assert!(cancel_result.is_ok());
        assert!(cancel_result.unwrap().success);
        
        let status = get_status(execution_id).await.unwrap();
        assert_eq!(status.status, "cancelled");
        assert!(!status.error.is_empty());
    }

    #[tokio::test]
    async fn test_get_result() {
        let start_result = start("test-workflow-5", None, Some(r#"{"input": 123}"#)).await.unwrap();
        let execution_id = &start_result.execution_id;
        
        let result = get_result(execution_id).await;
        assert!(result.is_ok());
        
        let output = result.unwrap();
        // Workflow is still running, not completed
        assert!(!output.completed);
    }

    #[tokio::test]
    async fn test_list_executions() {
        // Start multiple workflows
        start("list-test-1", None, None).await.unwrap();
        start("list-test-2", None, None).await.unwrap();
        start("list-test-1", None, None).await.unwrap();
        
        // List all executions
        let result = list_executions(None, None, None, Some(10), None, None).await;
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.total > 0);
        assert!(!output.executions.is_empty());
        
        // Filter by workflow_id
        let filtered = list_executions(None, Some("list-test-1"), None, Some(10), None, None).await.unwrap();
        assert!(filtered.total >= 2);
    }

    #[tokio::test]
    async fn test_history() {
        let start_result = start("test-workflow-6", None, None).await.unwrap();
        let execution_id = &start_result.execution_id;
        
        // Pause and resume to create history
        pause(execution_id, Some("Test")).await.unwrap();
        resume(execution_id, None).await.unwrap();
        
        let history_result = history(execution_id).await;
        assert!(history_result.is_ok());
        
        let output = history_result.unwrap();
        assert!(output.total >= 3); // At least: started, paused, resumed
        assert_eq!(output.events.len(), output.total as usize);
    }

    #[tokio::test]
    async fn test_signal() {
        let start_result = start("test-workflow-7", None, None).await.unwrap();
        let execution_id = &start_result.execution_id;
        
        let signal_result = signal(
            "test-signal",
            execution_id,
            Some(r#"{"message": "hello"}"#),
        ).await;
        
        assert!(signal_result.is_ok());
        assert!(signal_result.unwrap().success);
        
        // Check history for signal
        let history_result = history(execution_id).await.unwrap();
        let has_signal = history_result.events.iter().any(|e| {
            e.get("event_type").and_then(|v| v.as_str()) == Some("signal_received")
        });
        assert!(has_signal);
    }

    #[tokio::test]
    async fn test_retry_step() {
        let start_result = start("test-workflow-8", None, None).await.unwrap();
        let execution_id = &start_result.execution_id;
        
        // Simulate failure by manually setting status
        {
            if let Some(mut exec) = EXECUTIONS.get_mut(execution_id) {
                exec.status = "failed".to_string();
                exec.error = "Test error".to_string();
            }
        }
        
        let retry_result = retry_step(execution_id, "step-1").await;
        assert!(retry_result.is_ok());
        assert!(retry_result.unwrap().success);
        
        let status = get_status(execution_id).await.unwrap();
        assert_eq!(status.status, "running");
        assert!(status.error.is_empty());
    }

    #[tokio::test]
    async fn test_skip_step() {
        let start_result = start("test-workflow-9", None, None).await.unwrap();
        let execution_id = &start_result.execution_id;
        
        let skip_result = skip_step(
            execution_id,
            "step-2",
            Some(r#"{"skipped": true}"#),
        ).await;
        
        assert!(skip_result.is_ok());
        assert!(skip_result.unwrap().success);
        
        // Check history
        let history_result = history(execution_id).await.unwrap();
        let has_skip = history_result.events.iter().any(|e| {
            e.get("event_type").and_then(|v| v.as_str()) == Some("step_skipped")
        });
        assert!(has_skip);
    }

    #[tokio::test]
    async fn test_terminate_all() {
        // Start multiple workflows with same workflow_id
        start("terminate-test", None, None).await.unwrap();
        start("terminate-test", None, None).await.unwrap();
        start("terminate-test", None, None).await.unwrap();
        
        let result = terminate_all("terminate-test", Some("Bulk termination")).await;
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.terminated_count >= 3);
        
        // Verify all are cancelled
        let list_result = list_executions(
            Some("cancelled"),
            Some("terminate-test"),
            None,
            Some(10),
            None,
            None,
        ).await.unwrap();
        assert!(list_result.total >= 3);
    }

    #[tokio::test]
    async fn test_wait_for_event_timeout() {
        let start_result = start("test-workflow-10", None, None).await.unwrap();
        let execution_id = &start_result.execution_id;
        
        // Wait with very short timeout (should timeout)
        let wait_result = wait_for_event("never-happens", execution_id, Some(1)).await;
        assert!(wait_result.is_ok());
        
        let output = wait_result.unwrap();
        assert!(output.timed_out);
        assert!(!output.received);
    }

    #[tokio::test]
    async fn test_invalid_execution_id() {
        let result = get_status("non-existent-id").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[tokio::test]
    async fn test_pause_non_running_workflow() {
        let start_result = start("test-workflow-11", None, None).await.unwrap();
        let execution_id = &start_result.execution_id;
        
        // Cancel it first
        cancel(execution_id, None).await.unwrap();
        
        // Try to pause - should fail
        let pause_result = pause(execution_id, None).await;
        assert!(pause_result.is_err());
    }

    #[tokio::test]
    async fn test_workflow_with_context() {
        let mut context = HashMap::new();
        context.insert("user_id".to_string(), json!("12345"));
        context.insert("tenant".to_string(), json!("acme-corp"));
        
        let result = start("context-test", Some(context), None).await;
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.success);
    }
}
