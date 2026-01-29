#[cfg(test)]
mod tests {
    use crate::*;

    // Helper to create a unique queue name for each test
    fn unique_queue_name(prefix: &str) -> String {
        format!("{}_{}", prefix, uuid::Uuid::new_v4().to_string().split('-').next().unwrap())
    }

    #[tokio::test]
    async fn test_create_queue() {
        let queue_name = unique_queue_name("test_create");
        let result = create_queue(&queue_name, None, None, None).await.unwrap();
        assert!(result.success);
        assert!(result.queue_url.contains(&queue_name));
        
        // Cleanup
        delete_queue(&queue_name).await.ok();
    }

    #[tokio::test]
    async fn test_create_queue_duplicate() {
        let queue_name = unique_queue_name("test_dup");
        create_queue(&queue_name, None, None, None).await.unwrap();
        
        let result = create_queue(&queue_name, None, None, None).await;
        assert!(result.is_err());
        
        // Cleanup
        delete_queue(&queue_name).await.ok();
    }

    #[tokio::test]
    async fn test_delete_queue() {
        let queue_name = unique_queue_name("test_delete");
        create_queue(&queue_name, None, None, None).await.unwrap();
        
        let result = delete_queue(&queue_name).await.unwrap();
        assert!(result.success);
        
        // Verify queue is gone
        let result = delete_queue(&queue_name).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_enqueue_dequeue() {
        let queue_name = unique_queue_name("test_enqueue_dequeue");
        create_queue(&queue_name, None, None, None).await.unwrap();
        
        // Enqueue a message
        let enqueue_result = enqueue(&queue_name, "test message", None, None, None).await.unwrap();
        assert!(enqueue_result.success);
        assert!(!enqueue_result.message_id.is_empty());
        
        // Dequeue the message
        let dequeue_result = dequeue(&queue_name, None, None).await.unwrap();
        assert!(dequeue_result.found);
        assert_eq!(dequeue_result.message, "test message");
        assert!(!dequeue_result.receipt_handle.is_empty());
        
        // Cleanup
        delete_queue(&queue_name).await.ok();
    }

    #[tokio::test]
    async fn test_dequeue_empty_queue() {
        let queue_name = unique_queue_name("test_dequeue_empty");
        create_queue(&queue_name, None, None, None).await.unwrap();
        
        let result = dequeue(&queue_name, None, None).await.unwrap();
        assert!(!result.found);
        assert!(result.message.is_empty());
        
        // Cleanup
        delete_queue(&queue_name).await.ok();
    }

    #[tokio::test]
    async fn test_peek() {
        let queue_name = unique_queue_name("test_peek");
        create_queue(&queue_name, None, None, None).await.unwrap();
        
        enqueue(&queue_name, "message 1", None, None, None).await.unwrap();
        enqueue(&queue_name, "message 2", None, None, None).await.unwrap();
        
        // Peek should not remove messages
        let peek_result = peek(&queue_name, Some(2)).await.unwrap();
        assert_eq!(peek_result.messages.len(), 2);
        
        // Messages should still be there
        let stats = get_queue_stats(&queue_name).await.unwrap();
        assert_eq!(stats.message_count, 2);
        
        // Cleanup
        delete_queue(&queue_name).await.ok();
    }

    #[tokio::test]
    async fn test_acknowledge() {
        let queue_name = unique_queue_name("test_ack");
        create_queue(&queue_name, None, None, None).await.unwrap();
        
        enqueue(&queue_name, "test message", None, None, None).await.unwrap();
        let dequeue_result = dequeue(&queue_name, None, None).await.unwrap();
        
        // Acknowledge the message
        let ack_result = acknowledge(&queue_name, &dequeue_result.receipt_handle).await.unwrap();
        assert!(ack_result.success);
        
        // Stats should show no in-flight messages
        let stats = get_queue_stats(&queue_name).await.unwrap();
        assert_eq!(stats.in_flight_count, 0);
        
        // Cleanup
        delete_queue(&queue_name).await.ok();
    }

    #[tokio::test]
    async fn test_nack() {
        let queue_name = unique_queue_name("test_nack");
        create_queue(&queue_name, None, None, None).await.unwrap();
        
        enqueue(&queue_name, "test message", None, None, None).await.unwrap();
        let dequeue_result = dequeue(&queue_name, None, None).await.unwrap();
        
        // Nack the message (return to queue)
        let nack_result = nack(&queue_name, &dequeue_result.receipt_handle, None).await.unwrap();
        assert!(nack_result.success);
        
        // Message should be back in queue
        let stats = get_queue_stats(&queue_name).await.unwrap();
        assert_eq!(stats.message_count, 1);
        assert_eq!(stats.in_flight_count, 0);
        
        // Cleanup
        delete_queue(&queue_name).await.ok();
    }

    #[tokio::test]
    async fn test_get_queue_stats() {
        let queue_name = unique_queue_name("test_stats");
        create_queue(&queue_name, None, None, None).await.unwrap();
        
        enqueue(&queue_name, "message 1", None, None, None).await.unwrap();
        enqueue(&queue_name, "message 2", None, None, None).await.unwrap();
        enqueue(&queue_name, "delayed message", Some(60), None, None).await.unwrap();
        
        // Dequeue one to make it in-flight
        dequeue(&queue_name, None, None).await.unwrap();
        
        let stats = get_queue_stats(&queue_name).await.unwrap();
        assert_eq!(stats.message_count, 1); // One visible
        assert_eq!(stats.delayed_count, 1); // One delayed
        assert_eq!(stats.in_flight_count, 1); // One in-flight
        
        // Cleanup
        delete_queue(&queue_name).await.ok();
    }

    #[tokio::test]
    async fn test_purge_queue() {
        let queue_name = unique_queue_name("test_purge");
        create_queue(&queue_name, None, None, None).await.unwrap();
        
        enqueue(&queue_name, "message 1", None, None, None).await.unwrap();
        enqueue(&queue_name, "message 2", None, None, None).await.unwrap();
        enqueue(&queue_name, "message 3", None, None, None).await.unwrap();
        
        let result = purge_queue(&queue_name).await.unwrap();
        assert!(result.success);
        assert_eq!(result.messages_deleted, 3);
        
        // Queue should be empty
        let stats = get_queue_stats(&queue_name).await.unwrap();
        assert_eq!(stats.message_count, 0);
        
        // Cleanup
        delete_queue(&queue_name).await.ok();
    }

    #[tokio::test]
    async fn test_list_queues() {
        let prefix = unique_queue_name("list_test");
        let queue1 = format!("{}_q1", prefix);
        let queue2 = format!("{}_q2", prefix);
        
        create_queue(&queue1, None, None, None).await.unwrap();
        create_queue(&queue2, None, None, None).await.unwrap();
        
        let result = list_queues(Some(&prefix)).await.unwrap();
        assert!(result.total >= 2);
        assert!(result.queues.iter().any(|q| q == &queue1));
        assert!(result.queues.iter().any(|q| q == &queue2));
        
        // Cleanup
        delete_queue(&queue1).await.ok();
        delete_queue(&queue2).await.ok();
    }

    #[tokio::test]
    async fn test_move_to_dlq() {
        let queue_name = unique_queue_name("test_dlq");
        create_queue(&queue_name, None, None, None).await.unwrap();
        
        enqueue(&queue_name, "problematic message", None, None, None).await.unwrap();
        let dequeue_result = dequeue(&queue_name, None, None).await.unwrap();
        
        // Move to DLQ
        let dlq_result = move_to_dlq(&queue_name, &dequeue_result.receipt_handle, Some("processing failed")).await.unwrap();
        assert!(dlq_result.success);
        
        // Message should no longer be in-flight
        let stats = get_queue_stats(&queue_name).await.unwrap();
        assert_eq!(stats.in_flight_count, 0);
        
        // Cleanup
        delete_queue(&queue_name).await.ok();
    }

    #[tokio::test]
    async fn test_priority_ordering() {
        let queue_name = unique_queue_name("test_priority");
        create_queue(&queue_name, None, None, None).await.unwrap();
        
        // Enqueue with different priorities
        enqueue(&queue_name, "low priority", None, None, Some(1)).await.unwrap();
        enqueue(&queue_name, "high priority", None, None, Some(10)).await.unwrap();
        enqueue(&queue_name, "medium priority", None, None, Some(5)).await.unwrap();
        
        // Should dequeue in priority order (highest first)
        let msg1 = dequeue(&queue_name, None, None).await.unwrap();
        assert_eq!(msg1.message, "high priority");
        
        let msg2 = dequeue(&queue_name, None, None).await.unwrap();
        assert_eq!(msg2.message, "medium priority");
        
        let msg3 = dequeue(&queue_name, None, None).await.unwrap();
        assert_eq!(msg3.message, "low priority");
        
        // Cleanup
        delete_queue(&queue_name).await.ok();
    }

    #[tokio::test]
    async fn test_deduplication() {
        let queue_name = unique_queue_name("test_dedup");
        create_queue(&queue_name, None, None, None).await.unwrap();
        
        // First message with dedup ID should succeed
        let result1 = enqueue(&queue_name, "message", None, Some("dedup-123"), None).await.unwrap();
        assert!(result1.success);
        
        // Second message with same dedup ID should be rejected
        let result2 = enqueue(&queue_name, "message", None, Some("dedup-123"), None).await.unwrap();
        assert!(!result2.success);
        
        // Only one message should be in queue
        let stats = get_queue_stats(&queue_name).await.unwrap();
        assert_eq!(stats.message_count, 1);
        
        // Cleanup
        delete_queue(&queue_name).await.ok();
    }

    #[tokio::test]
    async fn test_message_size_limit() {
        let queue_name = unique_queue_name("test_size_limit");
        create_queue(&queue_name, Some(100), None, None).await.unwrap();
        
        // Message within limit should succeed
        let small_msg = "x".repeat(50);
        let result = enqueue(&queue_name, &small_msg, None, None, None).await.unwrap();
        assert!(result.success);
        
        // Message exceeding limit should fail
        let large_msg = "x".repeat(200);
        let result = enqueue(&queue_name, &large_msg, None, None, None).await;
        assert!(result.is_err());
        
        // Cleanup
        delete_queue(&queue_name).await.ok();
    }
}
