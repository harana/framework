#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test_start_simple_command() {
        let result = start("echo", Some(vec!["hello".to_string()]), None, None, None).await.unwrap();
        
        assert!(result.started);
        assert!(result.process_id > 0);
        assert!(result.error.is_none());
    }

    #[tokio::test]
    async fn test_start_with_args() {
        let result = start(
            "echo",
            Some(vec!["arg1".to_string(), "arg2".to_string()]),
            None,
            None,
            None,
        ).await.unwrap();
        
        assert!(result.started);
    }

    #[tokio::test]
    async fn test_start_with_environment() {
        let mut env = HashMap::new();
        env.insert("TEST_VAR".to_string(), "test_value".to_string());
        
        let result = start(
            "printenv",
            Some(vec!["TEST_VAR".to_string()]),
            None,
            Some(env),
            None,
        ).await.unwrap();
        
        assert!(result.started);
    }

    #[tokio::test]
    async fn test_start_nonexistent_command() {
        let result = start("nonexistent_command_xyz", None, None, None, None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_status_running() {
        // Start a long-running process
        let start_result = start("sleep", Some(vec!["10".to_string()]), None, None, None).await.unwrap();
        
        // Check status
        let status_result = status(start_result.process_id).await.unwrap();
        
        assert!(status_result.status.running);
        assert_eq!(status_result.status.process_id, start_result.process_id);
        
        // Cleanup
        let _ = kill(start_result.process_id, None).await;
    }

    #[tokio::test]
    async fn test_status_not_found() {
        let result = status(999999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_stop_process() {
        // Start a long-running process
        let start_result = start("sleep", Some(vec!["60".to_string()]), None, None, None).await.unwrap();
        
        // Stop it
        let stop_result = stop(start_result.process_id, Some(true), Some(5)).await.unwrap();
        
        assert!(stop_result.stopped);
    }

    #[tokio::test]
    async fn test_kill_process() {
        // Start a long-running process
        let start_result = start("sleep", Some(vec!["60".to_string()]), None, None, None).await.unwrap();
        
        // Kill it
        let kill_result = kill(start_result.process_id, Some("SIGTERM")).await.unwrap();
        
        assert!(kill_result.killed);
    }

    #[tokio::test]
    async fn test_kill_not_found() {
        let result = kill(999999, None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_wait_for_process() {
        // Start a quick process
        let start_result = start("echo", Some(vec!["done".to_string()]), None, None, None).await.unwrap();
        
        // Wait for it
        let wait_result = wait(start_result.process_id, Some(5)).await.unwrap();
        
        assert!(wait_result.completed);
        assert_eq!(wait_result.exit_code, Some(0));
    }

    #[tokio::test]
    async fn test_wait_timeout() {
        // Start a long-running process
        let start_result = start("sleep", Some(vec!["60".to_string()]), None, None, None).await.unwrap();
        
        // Wait with short timeout
        let wait_result = wait(start_result.process_id, Some(1)).await.unwrap();
        
        assert!(!wait_result.completed);
        assert!(wait_result.error.is_some());
        
        // Cleanup
        let _ = kill(start_result.process_id, None).await;
    }

    #[tokio::test]
    async fn test_list_processes() {
        // Start a process
        let start_result = start("sleep", Some(vec!["30".to_string()]), None, None, None).await.unwrap();
        
        // List processes
        let list_result = list(None, None).await.unwrap();
        
        assert!(!list_result.processes.is_empty());
        assert!(list_result.processes.iter().any(|p| p.process_id == start_result.process_id));
        
        // Cleanup
        let _ = kill(start_result.process_id, None).await;
    }

    #[tokio::test]
    async fn test_list_with_filter() {
        // Start two different processes
        let sleep_result = start("sleep", Some(vec!["30".to_string()]), None, None, None).await.unwrap();
        
        // List with filter
        let list_result = list(Some("sleep"), None).await.unwrap();
        
        assert!(list_result.processes.iter().all(|p| p.command.contains("sleep")));
        
        // Cleanup
        let _ = kill(sleep_result.process_id, None).await;
    }

    #[tokio::test]
    async fn test_get_output() {
        // Start a process that outputs something
        let start_result = start("echo", Some(vec!["test_output".to_string()]), None, None, None).await.unwrap();
        
        // Give it a moment to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Get output
        let output_result = get_output(start_result.process_id, Some("stdout")).await.unwrap();
        
        // Note: Output capture may vary based on timing
        assert!(output_result.error.is_none());
    }

    #[tokio::test]
    async fn test_process_uptime() {
        // Start a process
        let start_result = start("sleep", Some(vec!["10".to_string()]), None, None, None).await.unwrap();
        
        // Wait a bit
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
        // Check status
        let status_result = status(start_result.process_id).await.unwrap();
        
        assert!(status_result.status.uptime_secs.unwrap_or(0) >= 1);
        
        // Cleanup
        let _ = kill(start_result.process_id, None).await;
    }

    #[tokio::test]
    async fn test_process_exit_code() {
        // Start a process that exits with code 0
        let start_result = start("true", None, None, None, None).await.unwrap();
        
        // Wait for it
        let wait_result = wait(start_result.process_id, Some(5)).await.unwrap();
        
        assert!(wait_result.completed);
        assert_eq!(wait_result.exit_code, Some(0));
    }

    #[tokio::test]
    async fn test_process_exit_code_nonzero() {
        // Start a process that exits with non-zero code
        let start_result = start("false", None, None, None, None).await.unwrap();
        
        // Wait for it
        let wait_result = wait(start_result.process_id, Some(5)).await.unwrap();
        
        assert!(wait_result.completed);
        assert_eq!(wait_result.exit_code, Some(1));
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn test_kill_with_sigkill() {
        // Start a long-running process
        let start_result = start("sleep", Some(vec!["60".to_string()]), None, None, None).await.unwrap();
        
        // Kill with SIGKILL
        let kill_result = kill(start_result.process_id, Some("SIGKILL")).await.unwrap();
        
        assert!(kill_result.killed);
    }
}
