// Tests for the workflow module

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_start_workflow() {
        let result = start("test-workflow", None, None).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert!(!output.execution_id.is_empty());
        assert_eq!(output.status, "Running");

        // Clean up
        let _ = cancel(&output.execution_id, None).await;
    }

    #[tokio::test]
    async fn test_pause_and_resume() {
        // Start a workflow
        let start_result = start("test-workflow-pause", None, None).await;
        assert!(start_result.is_ok());
        let execution_id = start_result.unwrap().execution_id;

        // Pause it
        let pause_result = pause(&execution_id, Some("Testing pause")).await;
        assert!(pause_result.is_ok());
        assert!(pause_result.unwrap().success);

        // Check status
        let status_result = get_status(&execution_id).await;
        assert!(status_result.is_ok());
        assert_eq!(status_result.unwrap().status, "Paused");

        // Resume it
        let resume_result = resume(&execution_id, None).await;
        assert!(resume_result.is_ok());
        assert!(resume_result.unwrap().success);

        // Check status again
        let status_result = get_status(&execution_id).await;
        assert!(status_result.is_ok());
        assert_eq!(status_result.unwrap().status, "Running");

        // Clean up
        let _ = cancel(&execution_id, None).await;
    }

    #[tokio::test]
    async fn test_cancel_workflow() {
        // Start a workflow
        let start_result = start("test-workflow-cancel", None, None).await;
        assert!(start_result.is_ok());
        let execution_id = start_result.unwrap().execution_id;

        // Cancel it
        let cancel_result = cancel(&execution_id, Some("Testing cancel")).await;
        assert!(cancel_result.is_ok());
        assert!(cancel_result.unwrap().success);

        // Check status
        let status_result = get_status(&execution_id).await;
        assert!(status_result.is_ok());
        assert_eq!(status_result.unwrap().status, "Cancelled");
    }

    #[tokio::test]
    async fn test_signal_workflow() {
        // Start a workflow
        let start_result = start("test-workflow-signal", None, None).await;
        assert!(start_result.is_ok());
        let execution_id = start_result.unwrap().execution_id;

        // Send a signal
        let signal_result = signal(
            &execution_id,
            "approval",
            Some(serde_json::json!({ "approved": true })),
        ).await;
        assert!(signal_result.is_ok());
        assert!(signal_result.unwrap().success);

        // Wait for the event (should find it)
        let wait_result = wait_for_event(&execution_id, "approval", Some(30)).await;
        assert!(wait_result.is_ok());
        let wait_output = wait_result.unwrap();
        assert!(wait_output.received);
        assert!(!wait_output.timed_out);

        // Clean up
        let _ = cancel(&execution_id, None).await;
    }

    #[tokio::test]
    async fn test_workflow_history() {
        // Start a workflow
        let start_result = start("test-workflow-history", None, None).await;
        assert!(start_result.is_ok());
        let execution_id = start_result.unwrap().execution_id;

        // Pause it
        let _ = pause(&execution_id, None).await;

        // Resume it  
        let _ = resume(&execution_id, None).await;

        // Get history
        let history_result = history(&execution_id).await;
        assert!(history_result.is_ok());
        let history_output = history_result.unwrap();
        assert!(history_output.total >= 3); // Start, Pause, Resume events

        // Clean up
        let _ = cancel(&execution_id, None).await;
    }

    #[tokio::test]
    async fn test_list_executions() {
        let workflow_id = "test-workflow-list";
        
        // Start multiple workflows
        let _ = start(workflow_id, None, None).await;
        let _ = start(workflow_id, None, None).await;

        // List executions
        let list_result = list_executions(Some(workflow_id), None, None, None, None, None).await;
        assert!(list_result.is_ok());
        let list_output = list_result.unwrap();
        assert!(list_output.total >= 2);
    }

    #[tokio::test]
    async fn test_terminate_all() {
        let workflow_id = "test-workflow-terminate-all";
        
        // Start multiple workflows
        let _ = start(workflow_id, None, None).await;
        let _ = start(workflow_id, None, None).await;

        // Terminate all
        let terminate_result = terminate_all(workflow_id, Some("Batch termination")).await;
        assert!(terminate_result.is_ok());
        let terminate_output = terminate_result.unwrap();
        assert!(terminate_output.success);
        assert!(terminate_output.terminated_count >= 2);
    }
}
