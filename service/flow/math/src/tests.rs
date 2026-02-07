#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test_abs_positive() {
        let result = abs(5.0).await.unwrap();
        assert_eq!(result.result, 5.0);
    }

    #[tokio::test]
    async fn test_abs_negative() {
        let result = abs(-5.0).await.unwrap();
        assert_eq!(result.result, 5.0);
    }

    #[tokio::test]
    async fn test_average() {
        let result = average(vec![1.0, 2.0, 3.0, 4.0, 5.0]).await.unwrap();
        assert_eq!(result.result, 3.0);
    }

    #[tokio::test]
    async fn test_average_empty() {
        let result = average(vec![]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ceil() {
        let result = ceil(4.3).await.unwrap();
        assert_eq!(result.result, 5.0);
    }

    #[tokio::test]
    async fn test_floor() {
        let result = floor(4.7).await.unwrap();
        assert_eq!(result.result, 4.0);
    }

    #[tokio::test]
    async fn test_max() {
        let result = max(vec![1.0, 5.0, 3.0, 2.0]).await.unwrap();
        assert_eq!(result.result, 5.0);
    }

    #[tokio::test]
    async fn test_min() {
        let result = min(vec![1.0, 5.0, 3.0, 2.0]).await.unwrap();
        assert_eq!(result.result, 1.0);
    }

    #[tokio::test]
    async fn test_percentage() {
        let result = percentage(25.0, 100.0, None).await.unwrap();
        assert_eq!(result.result, 25.0);
    }

    #[tokio::test]
    async fn test_percentage_precision() {
        let result = percentage(1.0, 3.0, Some(4)).await.unwrap();
        assert_eq!(result.result, 33.3333);
    }

    #[tokio::test]
    async fn test_round() {
        let result = round(3.14159, Some(2)).await.unwrap();
        assert_eq!(result.result, 3.14);
    }

    #[tokio::test]
    async fn test_round_no_precision() {
        let result = round(3.7, None).await.unwrap();
        assert_eq!(result.result, 4.0);
    }

    #[tokio::test]
    async fn test_sum() {
        let result = sum(vec![1.0, 2.0, 3.0, 4.0, 5.0]).await.unwrap();
        assert_eq!(result.result, 15.0);
    }

    #[tokio::test]
    async fn test_sum_empty() {
        let result = sum(vec![]).await.unwrap();
        assert_eq!(result.result, 0.0);
    }
}
