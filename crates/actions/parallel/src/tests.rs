#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::json;

    fn make_task(id: &str, handler: &str, input: Value) -> ParallelTask {
        ParallelTask {
            id: id.to_string(),
            handler: handler.to_string(),
            input: Some(input),
            timeout_ms: None,
        }
    }

    #[tokio::test]
    async fn test_all_success() {
        let tasks = vec![
            make_task("1", "echo", json!("hello")),
            make_task("2", "echo", json!("world")),
            make_task("3", "double", json!(5)),
        ];

        let result = all(tasks, None, None, None).await.unwrap();

        assert!(result.success);
        assert_eq!(result.completed, 3);
        assert_eq!(result.failed, 0);
        assert_eq!(result.results.len(), 3);
    }

    #[tokio::test]
    async fn test_all_with_failure() {
        let tasks = vec![
            make_task("1", "echo", json!("hello")),
            make_task("2", "fail", json!(null)),
            make_task("3", "echo", json!("world")),
        ];

        let result = all(tasks, Some(false), None, None).await.unwrap();

        assert!(!result.success);
        assert_eq!(result.completed, 2);
        assert_eq!(result.failed, 1);
    }

    #[tokio::test]
    async fn test_all_fail_fast() {
        let tasks = vec![
            make_task("1", "fail", json!(null)),
            make_task("2", "echo", json!("hello")),
        ];

        let result = all(tasks, Some(true), None, None).await.unwrap();

        assert!(!result.success);
        assert_eq!(result.failed, 1);
    }

    #[tokio::test]
    async fn test_all_with_concurrency() {
        let tasks: Vec<_> = (0..10)
            .map(|i| make_task(&format!("{}", i), "echo", json!(i)))
            .collect();

        let result = all(tasks, None, Some(2), None).await.unwrap();

        assert!(result.success);
        assert_eq!(result.completed, 10);
    }

    #[tokio::test]
    async fn test_race() {
        let tasks = vec![
            make_task("1", "echo", json!("first")),
            make_task("2", "slow", json!("second")),
        ];

        let result = race(tasks, None).await.unwrap();

        assert!(result.success);
        assert!(result.winner_index >= 0);
    }

    #[tokio::test]
    async fn test_race_empty() {
        let tasks: Vec<ParallelTask> = vec![];
        let result = race(tasks, None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_any_success() {
        let tasks = vec![
            make_task("1", "fail", json!(null)),
            make_task("2", "echo", json!("success")),
            make_task("3", "fail", json!(null)),
        ];

        let result = any(tasks, None).await.unwrap();

        assert!(result.success);
        assert_eq!(result.success_index, 1);
    }

    #[tokio::test]
    async fn test_any_all_fail() {
        let tasks = vec![
            make_task("1", "fail", json!(null)),
            make_task("2", "fail", json!(null)),
        ];

        let result = any(tasks, None).await.unwrap();

        assert!(!result.success);
        assert_eq!(result.success_index, -1);
    }

    #[tokio::test]
    async fn test_all_settled() {
        let tasks = vec![
            make_task("1", "echo", json!("success")),
            make_task("2", "fail", json!(null)),
            make_task("3", "double", json!(10)),
        ];

        let result = all_settled(tasks, None, None).await.unwrap();

        assert_eq!(result.fulfilled, 2);
        assert_eq!(result.rejected, 1);
        assert_eq!(result.results.len(), 3);
        
        assert!(result.results.iter().any(|r| r.status == "fulfilled"));
        assert!(result.results.iter().any(|r| r.status == "rejected"));
    }

    #[tokio::test]
    async fn test_map() {
        let items = vec![json!(1), json!(2), json!(3), json!(4), json!(5)];

        let result = map("double", items, None).await.unwrap();

        assert!(result.success);
        assert_eq!(result.results.len(), 5);
        assert_eq!(result.results[0], json!(2));
        assert_eq!(result.results[4], json!(10));
    }

    #[tokio::test]
    async fn test_map_with_errors() {
        let items = vec![json!(1), json!("invalid"), json!(3)];

        let result = map("double", items, None).await.unwrap();

        assert!(!result.success);
        assert!(!result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_filter() {
        let items = vec![json!(1), json!(2), json!(3), json!(4), json!(5), json!(6)];

        let result = filter("is_even", items, None).await.unwrap();

        assert_eq!(result.count, 3);
        assert_eq!(result.results, vec![json!(2), json!(4), json!(6)]);
    }

    #[tokio::test]
    async fn test_reduce() {
        let items = vec![json!(1), json!(2), json!(3), json!(4), json!(5)];

        let result = reduce("sum", items, Some(json!(0)), None).await.unwrap();

        assert_eq!(result.result, json!(15));
    }

    #[tokio::test]
    async fn test_retry_success_first_attempt() {
        let task = make_task("1", "echo", json!("success"));

        let result = retry(task, None, Some(10), Some(3)).await.unwrap();

        assert!(result.success);
        assert_eq!(result.attempts, 1);
    }

    #[tokio::test]
    async fn test_retry_all_failures() {
        let task = make_task("1", "fail", json!(null));

        let result = retry(task, None, Some(10), Some(3)).await.unwrap();

        assert!(!result.success);
        assert_eq!(result.attempts, 3);
    }

    #[tokio::test]
    async fn test_timeout_success() {
        let task = make_task("1", "echo", json!("quick"));

        let result = timeout(task, 1000).await.unwrap();

        assert!(result.success);
        assert!(!result.timed_out);
        assert!(result.result.is_some());
    }

    #[tokio::test]
    async fn test_timeout_exceeded() {
        let task = make_task("1", "slow", json!("slow"));

        let result = timeout(task, 10).await.unwrap();

        assert!(!result.success);
        assert!(result.timed_out);
    }

    #[tokio::test]
    async fn test_batch() {
        let items: Vec<Value> = (1..=10).map(|i| json!(i)).collect();

        let result = batch("double", items, Some(3), Some(2)).await.unwrap();

        assert!(result.success);
        assert_eq!(result.batch_count, 4); // 10 items / 3 batch_size = 4 batches
        assert_eq!(result.results.len(), 10);
    }

    #[tokio::test]
    async fn test_semaphore() {
        let tasks: Vec<_> = (0..5)
            .map(|i| make_task(&format!("{}", i), "echo", json!(i)))
            .collect();

        let result = semaphore(2, tasks).await.unwrap();

        assert_eq!(result.completed, 5);
        assert_eq!(result.failed, 0);
        assert_eq!(result.results.len(), 5);
    }

    #[tokio::test]
    async fn test_semaphore_with_failures() {
        let tasks = vec![
            make_task("1", "echo", json!("ok")),
            make_task("2", "fail", json!(null)),
            make_task("3", "echo", json!("ok")),
        ];

        let result = semaphore(3, tasks).await.unwrap();

        assert_eq!(result.completed, 2);
        assert_eq!(result.failed, 1);
    }

    #[tokio::test]
    async fn test_all_with_timeout() {
        let tasks = vec![
            make_task("1", "echo", json!("quick")),
            make_task("2", "slow", json!("slow")),
        ];

        let result = all(tasks, None, None, Some(50)).await.unwrap();

        // One should complete, one should timeout
        assert!(result.results.iter().any(|r| r.success));
        assert!(result.results.iter().any(|r| !r.success));
    }

    #[tokio::test]
    async fn test_map_with_concurrency() {
        let items: Vec<Value> = (1..=20).map(|i| json!(i)).collect();

        let result = map("double", items, Some(5)).await.unwrap();

        assert!(result.success);
        assert_eq!(result.results.len(), 20);
    }

    #[tokio::test]
    async fn test_echo_handler() {
        let task = make_task("1", "echo", json!({"key": "value"}));
        let result = all(vec![task], None, None, None).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.results[0].result, Some(json!({"key": "value"})));
    }

    #[tokio::test]
    async fn test_increment_handler() {
        let task = make_task("1", "increment", json!(5));
        let result = all(vec![task], None, None, None).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.results[0].result, Some(json!(6)));
    }
}
