// Harana Actions - Date Module
// This module provides date actions and functionality.

pub mod output;

use chrono::{DateTime, Datelike, Duration, NaiveDateTime, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use output::*;

/// Parse a date string to DateTime<Utc>
fn parse_date(date: &str, format: Option<&str>) -> Result<DateTime<Utc>, String> {
    if let Some(fmt) = format {
        let naive = NaiveDateTime::parse_from_str(date, fmt)
            .map_err(|e| format!("Failed to parse date: {}", e))?;
        Ok(DateTime::from_naive_utc_and_offset(naive, Utc))
    } else {
        // Try common formats
        let formats = [
            "%Y-%m-%dT%H:%M:%S%.fZ",
            "%Y-%m-%dT%H:%M:%SZ",
            "%Y-%m-%dT%H:%M:%S",
            "%Y-%m-%d %H:%M:%S",
            "%Y-%m-%d",
        ];
        
        for fmt in formats {
            if let Ok(naive) = NaiveDateTime::parse_from_str(date, fmt) {
                return Ok(DateTime::from_naive_utc_and_offset(naive, Utc));
            }
            // Try date only
            if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(date, fmt) {
                let naive = naive_date.and_hms_opt(0, 0, 0).unwrap();
                return Ok(DateTime::from_naive_utc_and_offset(naive, Utc));
            }
        }
        
        // Try RFC3339
        DateTime::parse_from_rfc3339(date)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| format!("Failed to parse date: {}", e))
    }
}

/// Parse timezone string to Tz
fn parse_timezone(tz: &str) -> Result<Tz, String> {
    tz.parse::<Tz>()
        .map_err(|_| format!("Invalid timezone: {}", tz))
}

/// Add Duration To Date
/// 
/// Adds a specified duration to a date.
pub async fn add(
    date: &str,
    unit: &str,
    amount: i32,
) -> Result<AddOutput, String> {
    let dt = parse_date(date, None)?;
    
    let duration = match unit.to_lowercase().as_str() {
        "seconds" | "second" | "s" => Duration::seconds(amount as i64),
        "minutes" | "minute" | "m" => Duration::minutes(amount as i64),
        "hours" | "hour" | "h" => Duration::hours(amount as i64),
        "days" | "day" | "d" => Duration::days(amount as i64),
        "weeks" | "week" | "w" => Duration::weeks(amount as i64),
        _ => return Err(format!("Unsupported unit: {}", unit)),
    };
    
    let result = dt + duration;
    
    Ok(AddOutput {
        result: result.to_rfc3339(),
    })
}

/// Convert Date Timezone
/// 
/// Converts a date from one timezone to another.
pub async fn convert_timezone(
    to_timezone: &str,
    date: &str,
    from_timezone: &str,
) -> Result<ConvertTimezoneOutput, String> {
    let from_tz = parse_timezone(from_timezone)?;
    let to_tz = parse_timezone(to_timezone)?;
    
    // Parse the date in the source timezone
    let naive = NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S"))
        .map_err(|e| format!("Failed to parse date: {}", e))?;
    
    let from_dt = from_tz.from_local_datetime(&naive)
        .single()
        .ok_or("Ambiguous or invalid local time")?;
    
    let to_dt = from_dt.with_timezone(&to_tz);
    
    Ok(ConvertTimezoneOutput {
        result: to_dt.format("%Y-%m-%dT%H:%M:%S").to_string(),
    })
}

/// Calculate Date Difference
/// 
/// Calculates the difference between two dates.
pub async fn diff(
    end: &str,
    start: &str,
    unit: Option<&str>,
) -> Result<DiffOutput, String> {
    let start_dt = parse_date(start, None)?;
    let end_dt = parse_date(end, None)?;
    
    let duration = end_dt.signed_duration_since(start_dt);
    let unit = unit.unwrap_or("seconds");
    
    let difference = match unit.to_lowercase().as_str() {
        "seconds" | "second" | "s" => duration.num_seconds() as f64,
        "minutes" | "minute" | "m" => duration.num_minutes() as f64,
        "hours" | "hour" | "h" => duration.num_hours() as f64,
        "days" | "day" | "d" => duration.num_days() as f64,
        "weeks" | "week" | "w" => duration.num_weeks() as f64,
        "milliseconds" | "ms" => duration.num_milliseconds() as f64,
        _ => return Err(format!("Unsupported unit: {}", unit)),
    };
    
    Ok(DiffOutput { difference })
}

/// Get End Of Period
/// 
/// Returns the end of the specified period (day, week, month, year).
pub async fn end_of(
    period: &str,
    date: &str,
    timezone: Option<&str>,
) -> Result<EndOfOutput, String> {
    let dt = parse_date(date, None)?;
    
    let result = match period.to_lowercase().as_str() {
        "day" => dt
            .with_hour(23).unwrap()
            .with_minute(59).unwrap()
            .with_second(59).unwrap()
            .with_nanosecond(999_999_999).unwrap(),
        "week" => {
            let days_until_sunday = 6 - dt.weekday().num_days_from_monday();
            (dt + Duration::days(days_until_sunday as i64))
                .with_hour(23).unwrap()
                .with_minute(59).unwrap()
                .with_second(59).unwrap()
                .with_nanosecond(999_999_999).unwrap()
        }
        "month" => {
            let next_month = if dt.month() == 12 {
                dt.with_year(dt.year() + 1).unwrap().with_month(1).unwrap()
            } else {
                dt.with_month(dt.month() + 1).unwrap()
            };
            let last_day = next_month.with_day(1).unwrap() - Duration::days(1);
            last_day
                .with_hour(23).unwrap()
                .with_minute(59).unwrap()
                .with_second(59).unwrap()
                .with_nanosecond(999_999_999).unwrap()
        }
        "year" => dt
            .with_month(12).unwrap()
            .with_day(31).unwrap()
            .with_hour(23).unwrap()
            .with_minute(59).unwrap()
            .with_second(59).unwrap()
            .with_nanosecond(999_999_999).unwrap(),
        _ => return Err(format!("Unsupported period: {}", period)),
    };
    
    let result_str = if let Some(tz) = timezone {
        let tz = parse_timezone(tz)?;
        result.with_timezone(&tz).format("%Y-%m-%dT%H:%M:%S").to_string()
    } else {
        result.to_rfc3339()
    };
    
    Ok(EndOfOutput { result: result_str })
}

/// Format Date To String
/// 
/// Formats a date according to the specified format string.
pub async fn format(
    format: &str,
    date: &str,
    timezone: Option<&str>,
) -> Result<FormatOutput, String> {
    let dt = parse_date(date, None)?;
    
    let formatted = if let Some(tz) = timezone {
        let tz = parse_timezone(tz)?;
        dt.with_timezone(&tz).format(format).to_string()
    } else {
        dt.format(format).to_string()
    };
    
    Ok(FormatOutput { formatted })
}

/// Get Current Timestamp
/// 
/// Returns the current timestamp.
pub async fn now(
    timezone: Option<&str>,
) -> Result<NowOutput, String> {
    let now = Utc::now();
    
    let timestamp = if let Some(tz) = timezone {
        let tz = parse_timezone(tz)?;
        now.with_timezone(&tz).to_rfc3339()
    } else {
        now.to_rfc3339()
    };
    
    Ok(NowOutput {
        timestamp,
        unix: now.timestamp(),
        unix_millis: now.timestamp_millis(),
    })
}

/// Parse Date From String
/// 
/// Parses a date string into a standardized format.
pub async fn parse(
    date: &str,
    format: Option<&str>,
    _timezone: Option<&str>,
) -> Result<ParseOutput, String> {
    let dt = parse_date(date, format)?;
    
    Ok(ParseOutput {
        timestamp: dt.to_rfc3339(),
        unix: dt.timestamp(),
        year: dt.year(),
        month: dt.month() as i32,
        day: dt.day() as i32,
        hour: dt.hour() as i32,
        minute: dt.minute() as i32,
        second: dt.second() as i32,
    })
}

/// Get Start Of Period
/// 
/// Returns the start of the specified period (day, week, month, year).
pub async fn start_of(
    date: &str,
    period: &str,
    timezone: Option<&str>,
) -> Result<StartOfOutput, String> {
    let dt = parse_date(date, None)?;
    
    let result = match period.to_lowercase().as_str() {
        "day" => dt
            .with_hour(0).unwrap()
            .with_minute(0).unwrap()
            .with_second(0).unwrap()
            .with_nanosecond(0).unwrap(),
        "week" => {
            let days_since_monday = dt.weekday().num_days_from_monday();
            (dt - Duration::days(days_since_monday as i64))
                .with_hour(0).unwrap()
                .with_minute(0).unwrap()
                .with_second(0).unwrap()
                .with_nanosecond(0).unwrap()
        }
        "month" => dt
            .with_day(1).unwrap()
            .with_hour(0).unwrap()
            .with_minute(0).unwrap()
            .with_second(0).unwrap()
            .with_nanosecond(0).unwrap(),
        "year" => dt
            .with_month(1).unwrap()
            .with_day(1).unwrap()
            .with_hour(0).unwrap()
            .with_minute(0).unwrap()
            .with_second(0).unwrap()
            .with_nanosecond(0).unwrap(),
        _ => return Err(format!("Unsupported period: {}", period)),
    };
    
    let result_str = if let Some(tz) = timezone {
        let tz = parse_timezone(tz)?;
        result.with_timezone(&tz).format("%Y-%m-%dT%H:%M:%S").to_string()
    } else {
        result.to_rfc3339()
    };
    
    Ok(StartOfOutput { result: result_str })
}

/// Subtract Duration From Date
/// 
/// Subtracts a specified duration from a date.
pub async fn subtract(
    amount: i32,
    date: &str,
    unit: &str,
) -> Result<SubtractOutput, String> {
    let dt = parse_date(date, None)?;
    
    let duration = match unit.to_lowercase().as_str() {
        "seconds" | "second" | "s" => Duration::seconds(amount as i64),
        "minutes" | "minute" | "m" => Duration::minutes(amount as i64),
        "hours" | "hour" | "h" => Duration::hours(amount as i64),
        "days" | "day" | "d" => Duration::days(amount as i64),
        "weeks" | "week" | "w" => Duration::weeks(amount as i64),
        _ => return Err(format!("Unsupported unit: {}", unit)),
    };
    
    let result = dt - duration;
    
    Ok(SubtractOutput {
        result: result.to_rfc3339(),
    })
}

#[cfg(test)]
mod tests;
