#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test_add_days() {
        let result = add("2024-01-01T00:00:00Z", "days", 5).await.unwrap();
        assert!(result.result.contains("2024-01-06"));
    }

    #[tokio::test]
    async fn test_add_hours() {
        let result = add("2024-01-01T00:00:00Z", "hours", 3).await.unwrap();
        assert!(result.result.contains("03:00:00"));
    }

    #[tokio::test]
    async fn test_diff_days() {
        let result = diff("2024-01-10", "2024-01-01", Some("days")).await.unwrap();
        assert_eq!(result.difference, 9.0);
    }

    #[tokio::test]
    async fn test_diff_hours() {
        let result = diff("2024-01-01T03:00:00Z", "2024-01-01T00:00:00Z", Some("hours")).await.unwrap();
        assert_eq!(result.difference, 3.0);
    }

    #[tokio::test]
    async fn test_format() {
        let result = format("%Y-%m-%d", "2024-01-15T12:30:00Z", None).await.unwrap();
        assert_eq!(result.formatted, "2024-01-15");
    }

    #[tokio::test]
    async fn test_now() {
        let result = now(None).await.unwrap();
        assert!(!result.timestamp.is_empty());
        assert!(result.unix > 0);
    }

    #[tokio::test]
    async fn test_parse() {
        let result = parse("2024-01-15", None, None).await.unwrap();
        assert_eq!(result.year, 2024);
        assert_eq!(result.month, 1);
        assert_eq!(result.day, 15);
    }

    #[tokio::test]
    async fn test_start_of_day() {
        let result = start_of("2024-01-15T14:30:00Z", "day", None).await.unwrap();
        assert!(result.result.contains("00:00:00"));
    }

    #[tokio::test]
    async fn test_start_of_month() {
        let result = start_of("2024-01-15T14:30:00Z", "month", None).await.unwrap();
        assert!(result.result.contains("2024-01-01"));
    }

    #[tokio::test]
    async fn test_end_of_day() {
        let result = end_of("day", "2024-01-15T14:30:00Z", None).await.unwrap();
        assert!(result.result.contains("23:59:59"));
    }

    #[tokio::test]
    async fn test_subtract_days() {
        let result = subtract(5, "2024-01-10T00:00:00Z", "days").await.unwrap();
        assert!(result.result.contains("2024-01-05"));
    }

    #[tokio::test]
    async fn test_convert_timezone() {
        let result = convert_timezone("America/New_York", "2024-01-15T12:00:00", "UTC").await.unwrap();
        assert!(result.result.contains("07:00:00")); // UTC-5
    }
}
