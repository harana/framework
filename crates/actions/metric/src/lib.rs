pub mod output;

use output::*;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use prometheus::{
    Counter, CounterVec, Gauge, GaugeVec, Histogram, HistogramOpts, HistogramVec,
    Opts, Registry, TextEncoder, Encoder,
};
use std::collections::HashMap;
use std::time::Instant;
use uuid::Uuid;

/// Global Prometheus registry
static REGISTRY: Lazy<Registry> = Lazy::new(|| Registry::new());

/// Storage for counter metrics
static COUNTERS: Lazy<DashMap<String, Counter>> = Lazy::new(DashMap::new);
static COUNTER_VECS: Lazy<DashMap<String, CounterVec>> = Lazy::new(DashMap::new);

/// Storage for gauge metrics
static GAUGES: Lazy<DashMap<String, Gauge>> = Lazy::new(DashMap::new);
static GAUGE_VECS: Lazy<DashMap<String, GaugeVec>> = Lazy::new(DashMap::new);

/// Storage for histogram metrics
static HISTOGRAMS: Lazy<DashMap<String, Histogram>> = Lazy::new(DashMap::new);
static HISTOGRAM_VECS: Lazy<DashMap<String, HistogramVec>> = Lazy::new(DashMap::new);

/// Storage for active timers
#[derive(Debug)]
struct TimerValue {
    name: String,
    start: Instant,
    tags: HashMap<String, String>,
}

static TIMERS: Lazy<DashMap<String, TimerValue>> = Lazy::new(DashMap::new);

/// Metadata about registered metrics
static METRIC_METADATA: Lazy<RwLock<HashMap<String, MetricMetadata>>> = 
    Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Debug, Clone)]
struct MetricMetadata {
    metric_type: String,
    label_names: Vec<String>,
}

/// Storage for raw metric values (for query_metrics and get_metric_summary)
#[derive(Debug, Clone)]
struct MetricDataValue {
    timestamp: i64,
    value: f64,
}

static METRIC_VALUES: Lazy<DashMap<String, Vec<MetricDataValue>>> = Lazy::new(DashMap::new);

/// Converts optional tags to HashMap<String, String>
fn normalize_tags(tags: Option<HashMap<String, String>>) -> HashMap<String, String> {
    tags.unwrap_or_default()
}

/// Sanitize metric name for Prometheus (replace dots with underscores, etc.)
fn sanitize_name(name: &str) -> String {
    name.replace('.', "_").replace('-', "_")
}

/// Generate a metric key with tags for internal tracking
fn metric_key(name: &str, tags: &HashMap<String, String>) -> String {
    if tags.is_empty() {
        name.to_string()
    } else {
        let mut tag_parts: Vec<String> = tags
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        tag_parts.sort();
        format!("{}:{}", name, tag_parts.join(","))
    }
}

/// Parse duration string to milliseconds
fn parse_duration_ms(duration: &str) -> i64 {
    let duration = duration.trim().to_lowercase();
    if duration.ends_with("ms") {
        duration.trim_end_matches("ms").parse().unwrap_or(60000)
    } else if duration.ends_with('s') {
        duration.trim_end_matches('s').parse::<i64>().unwrap_or(60) * 1000
    } else if duration.ends_with('m') {
        duration.trim_end_matches('m').parse::<i64>().unwrap_or(1) * 60 * 1000
    } else if duration.ends_with('h') {
        duration.trim_end_matches('h').parse::<i64>().unwrap_or(1) * 3600 * 1000
    } else if duration.ends_with('d') {
        duration.trim_end_matches('d').parse::<i64>().unwrap_or(1) * 86400 * 1000
    } else {
        duration.parse::<i64>().unwrap_or(60) * 1000
    }
}

/// Get or create a Gauge for the given name and tags
fn get_or_create_gauge(name: &str, tags: &HashMap<String, String>) -> Result<Gauge, String> {
    let sanitized = sanitize_name(name);
    
    if tags.is_empty() {
        if let Some(gauge) = GAUGES.get(&sanitized) {
            return Ok(gauge.clone());
        }
        
        let opts = Opts::new(&sanitized, format!("Gauge metric: {}", name));
        let gauge = Gauge::with_opts(opts)
            .map_err(|e| format!("Failed to create gauge: {}", e))?;
        
        REGISTRY.register(Box::new(gauge.clone()))
            .map_err(|e| format!("Failed to register gauge: {}", e))?;
        
        GAUGES.insert(sanitized.clone(), gauge.clone());
        
        METRIC_METADATA.write().insert(name.to_string(), MetricMetadata {
            metric_type: "gauge".to_string(),
            label_names: vec![],
        });
        
        Ok(gauge)
    } else {
        let label_names: Vec<&str> = tags.keys().map(|s| s.as_str()).collect();
        let label_values: Vec<&str> = tags.values().map(|s| s.as_str()).collect();
        
        if let Some(gauge_vec) = GAUGE_VECS.get(&sanitized) {
            return gauge_vec.get_metric_with_label_values(&label_values)
                .map_err(|e| format!("Failed to get gauge with labels: {}", e));
        }
        
        let opts = Opts::new(&sanitized, format!("Gauge metric: {}", name));
        let gauge_vec = GaugeVec::new(opts, &label_names)
            .map_err(|e| format!("Failed to create gauge vec: {}", e))?;
        
        REGISTRY.register(Box::new(gauge_vec.clone()))
            .map_err(|e| format!("Failed to register gauge vec: {}", e))?;
        
        GAUGE_VECS.insert(sanitized.clone(), gauge_vec.clone());
        
        METRIC_METADATA.write().insert(name.to_string(), MetricMetadata {
            metric_type: "gauge".to_string(),
            label_names: tags.keys().cloned().collect(),
        });
        
        gauge_vec.get_metric_with_label_values(&label_values)
            .map_err(|e| format!("Failed to get gauge with labels: {}", e))
    }
}

/// Get or create a Counter for the given name and tags
fn get_or_create_counter(name: &str, tags: &HashMap<String, String>) -> Result<Counter, String> {
    let sanitized = sanitize_name(name);
    
    if tags.is_empty() {
        if let Some(counter) = COUNTERS.get(&sanitized) {
            return Ok(counter.clone());
        }
        
        let opts = Opts::new(&sanitized, format!("Counter metric: {}", name));
        let counter = Counter::with_opts(opts)
            .map_err(|e| format!("Failed to create counter: {}", e))?;
        
        REGISTRY.register(Box::new(counter.clone()))
            .map_err(|e| format!("Failed to register counter: {}", e))?;
        
        COUNTERS.insert(sanitized.clone(), counter.clone());
        
        METRIC_METADATA.write().insert(name.to_string(), MetricMetadata {
            metric_type: "counter".to_string(),
            label_names: vec![],
        });
        
        Ok(counter)
    } else {
        let label_names: Vec<&str> = tags.keys().map(|s| s.as_str()).collect();
        let label_values: Vec<&str> = tags.values().map(|s| s.as_str()).collect();
        
        if let Some(counter_vec) = COUNTER_VECS.get(&sanitized) {
            return counter_vec.get_metric_with_label_values(&label_values)
                .map_err(|e| format!("Failed to get counter with labels: {}", e));
        }
        
        let opts = Opts::new(&sanitized, format!("Counter metric: {}", name));
        let counter_vec = CounterVec::new(opts, &label_names)
            .map_err(|e| format!("Failed to create counter vec: {}", e))?;
        
        REGISTRY.register(Box::new(counter_vec.clone()))
            .map_err(|e| format!("Failed to register counter vec: {}", e))?;
        
        COUNTER_VECS.insert(sanitized.clone(), counter_vec.clone());
        
        METRIC_METADATA.write().insert(name.to_string(), MetricMetadata {
            metric_type: "counter".to_string(),
            label_names: tags.keys().cloned().collect(),
        });
        
        counter_vec.get_metric_with_label_values(&label_values)
            .map_err(|e| format!("Failed to get counter with labels: {}", e))
    }
}

/// Get or create a Histogram for the given name, tags, and buckets
fn get_or_create_histogram(
    name: &str,
    tags: &HashMap<String, String>,
    buckets: &[f64],
) -> Result<Histogram, String> {
    let sanitized = sanitize_name(name);
    
    if tags.is_empty() {
        if let Some(histogram) = HISTOGRAMS.get(&sanitized) {
            return Ok(histogram.clone());
        }
        
        let opts = HistogramOpts::new(&sanitized, format!("Histogram metric: {}", name))
            .buckets(buckets.to_vec());
        let histogram = Histogram::with_opts(opts)
            .map_err(|e| format!("Failed to create histogram: {}", e))?;
        
        REGISTRY.register(Box::new(histogram.clone()))
            .map_err(|e| format!("Failed to register histogram: {}", e))?;
        
        HISTOGRAMS.insert(sanitized.clone(), histogram.clone());
        
        METRIC_METADATA.write().insert(name.to_string(), MetricMetadata {
            metric_type: "histogram".to_string(),
            label_names: vec![],
        });
        
        Ok(histogram)
    } else {
        let label_names: Vec<&str> = tags.keys().map(|s| s.as_str()).collect();
        let label_values: Vec<&str> = tags.values().map(|s| s.as_str()).collect();
        
        if let Some(histogram_vec) = HISTOGRAM_VECS.get(&sanitized) {
            return histogram_vec.get_metric_with_label_values(&label_values)
                .map_err(|e| format!("Failed to get histogram with labels: {}", e));
        }
        
        let opts = HistogramOpts::new(&sanitized, format!("Histogram metric: {}", name))
            .buckets(buckets.to_vec());
        let histogram_vec = HistogramVec::new(opts, &label_names)
            .map_err(|e| format!("Failed to create histogram vec: {}", e))?;
        
        REGISTRY.register(Box::new(histogram_vec.clone()))
            .map_err(|e| format!("Failed to register histogram vec: {}", e))?;
        
        HISTOGRAM_VECS.insert(sanitized.clone(), histogram_vec.clone());
        
        METRIC_METADATA.write().insert(name.to_string(), MetricMetadata {
            metric_type: "histogram".to_string(),
            label_names: tags.keys().cloned().collect(),
        });
        
        histogram_vec.get_metric_with_label_values(&label_values)
            .map_err(|e| format!("Failed to get histogram with labels: {}", e))
    }
}

/// Record Metric Value.
/// Records a metric value as a gauge and stores for historical queries.
pub async fn record_metric(
    name: &str,
    value: f64,
    timestamp: Option<&str>,
    tags: Option<HashMap<String, String>>,
) -> Result<RecordMetricOutput, String> {
    let tags = normalize_tags(tags);
    let key = metric_key(name, &tags);
    
    // Set gauge value in Prometheus
    let gauge = get_or_create_gauge(name, &tags)?;
    gauge.set(value);
    
    // Store historical value for query_metrics
    let ts = match timestamp {
        Some(ts) => DateTime::parse_from_rfc3339(ts)
            .map(|dt| dt.with_timezone(&Utc).timestamp_millis())
            .unwrap_or_else(|_| Utc::now().timestamp_millis()),
        None => Utc::now().timestamp_millis(),
    };
    
    METRIC_VALUES
        .entry(key)
        .or_default()
        .push(MetricDataValue {
            timestamp: ts,
            value,
        });
    
    Ok(RecordMetricOutput { success: true })
}

/// Increment Counter Metric.
pub async fn increment_counter(
    name: &str,
    amount: Option<f64>,
    tags: Option<HashMap<String, String>>,
) -> Result<IncrementCounterOutput, String> {
    let amount = amount.unwrap_or(1.0);
    let tags = normalize_tags(tags);
    
    let counter = get_or_create_counter(name, &tags)?;
    counter.inc_by(amount);
    
    Ok(IncrementCounterOutput { value: counter.get() })
}

/// Set Gauge Value.
pub async fn set_gauge(
    name: &str,
    value: f64,
    tags: Option<HashMap<String, String>>,
) -> Result<SetGaugeOutput, String> {
    let tags = normalize_tags(tags);
    
    let gauge = get_or_create_gauge(name, &tags)?;
    gauge.set(value);
    
    Ok(SetGaugeOutput { success: true })
}

/// Record Histogram Value.
pub async fn record_histogram(
    value: f64,
    name: &str,
    buckets: Option<Vec<f64>>,
    tags: Option<HashMap<String, String>>,
) -> Result<RecordHistogramOutput, String> {
    let tags = normalize_tags(tags);
    let buckets = buckets.unwrap_or_else(|| {
        vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]
    });
    
    let histogram = get_or_create_histogram(name, &tags, &buckets)?;
    histogram.observe(value);
    
    // Store for summary queries
    let key = metric_key(name, &tags);
    METRIC_VALUES
        .entry(key)
        .or_default()
        .push(MetricDataValue {
            timestamp: Utc::now().timestamp_millis(),
            value,
        });
    
    Ok(RecordHistogramOutput { success: true })
}

/// Start Timer Measurement.
pub async fn start_timer(
    name: &str,
    tags: Option<HashMap<String, String>>,
) -> Result<StartTimerOutput, String> {
    let timer_id = Uuid::new_v4().to_string();
    let tags = normalize_tags(tags);
    
    TIMERS.insert(
        timer_id.clone(),
        TimerValue {
            name: name.to_string(),
            start: Instant::now(),
            tags,
        },
    );
    
    Ok(StartTimerOutput { timer_id })
}

/// Stop Timer Measurement.
pub async fn stop_timer(
    timer_id: &str,
) -> Result<StopTimerOutput, String> {
    let timer = TIMERS
        .remove(timer_id)
        .ok_or_else(|| format!("Timer not found: {}", timer_id))?;
    
    let duration_ms = timer.1.start.elapsed().as_secs_f64() * 1000.0;
    
    // Record the timer value as a histogram
    let buckets = vec![1.0, 5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0, 2500.0, 5000.0, 10000.0];
    let histogram = get_or_create_histogram(&timer.1.name, &timer.1.tags, &buckets)?;
    histogram.observe(duration_ms);
    
    // Store for summary queries
    let key = metric_key(&timer.1.name, &timer.1.tags);
    METRIC_VALUES
        .entry(key)
        .or_default()
        .push(MetricDataValue {
            timestamp: Utc::now().timestamp_millis(),
            value: duration_ms,
        });
    
    Ok(StopTimerOutput {
        success: true,
        duration_ms,
    })
}

/// Query Metric Values.
pub async fn query_metrics(
    start_time: &str,
    name: &str,
    aggregation: Option<&str>,
    end_time: Option<&str>,
    tags: Option<HashMap<String, String>>,
    interval: Option<&str>,
) -> Result<QueryMetricsOutput, String> {
    let tags = normalize_tags(tags);
    let key = metric_key(name, &tags);
    let _aggregation = aggregation.unwrap_or("avg");
    let _interval_ms = parse_duration_ms(interval.unwrap_or("1m"));
    
    let start_ts = DateTime::parse_from_rfc3339(start_time)
        .map(|dt| dt.with_timezone(&Utc).timestamp_millis())
        .map_err(|e| format!("Invalid start_time: {}", e))?;
    
    let end_ts = match end_time {
        Some(et) => DateTime::parse_from_rfc3339(et)
            .map(|dt| dt.with_timezone(&Utc).timestamp_millis())
            .map_err(|e| format!("Invalid end_time: {}", e))?,
        None => Utc::now().timestamp_millis(),
    };
    
    let datapoints: Vec<MetricDatapoint> = METRIC_VALUES
        .get(&key)
        .map(|values| {
            values
                .iter()
                .filter(|v| v.timestamp >= start_ts && v.timestamp <= end_ts)
                .map(|v| MetricDatapoint {
                    timestamp: v.timestamp,
                    value: v.value,
                })
                .collect()
        })
        .unwrap_or_default();
    
    let total = datapoints.len() as i32;
    
    Ok(QueryMetricsOutput { datapoints, total })
}

/// Get Metric Summary.
pub async fn get_metric_summary(
    name: &str,
    tags: Option<HashMap<String, String>>,
    period: Option<&str>,
) -> Result<GetMetricSummaryOutput, String> {
    let tags = normalize_tags(tags);
    let key = metric_key(name, &tags);
    let period_ms = parse_duration_ms(period.unwrap_or("1h"));
    let cutoff = Utc::now().timestamp_millis() - period_ms;
    
    let mut values: Vec<f64> = METRIC_VALUES
        .get(&key)
        .map(|m| {
            m.iter()
                .filter(|v| v.timestamp >= cutoff)
                .map(|v| v.value)
                .collect()
        })
        .unwrap_or_default();
    
    if values.is_empty() {
        return Ok(GetMetricSummaryOutput {
            min: 0.0,
            max: 0.0,
            avg: 0.0,
            sum: 0.0,
            count: 0,
            p50: 0.0,
            p95: 0.0,
            p99: 0.0,
        });
    }
    
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    
    let count = values.len();
    let sum: f64 = values.iter().sum();
    let min = *values.first().unwrap_or(&0.0);
    let max = *values.last().unwrap_or(&0.0);
    let avg = sum / count as f64;
    
    let p50 = percentile(&values, 50.0);
    let p95 = percentile(&values, 95.0);
    let p99 = percentile(&values, 99.0);
    
    Ok(GetMetricSummaryOutput {
        min,
        max,
        avg,
        sum,
        count: count as i32,
        p50,
        p95,
        p99,
    })
}

/// Calculate percentile from sorted values
fn percentile(sorted_values: &[f64], p: f64) -> f64 {
    if sorted_values.is_empty() {
        return 0.0;
    }
    let idx = (p / 100.0 * (sorted_values.len() - 1) as f64).round() as usize;
    sorted_values[idx.min(sorted_values.len() - 1)]
}

/// List Available Metrics.
pub async fn list_metrics(
    limit: Option<i32>,
    prefix: Option<&str>,
) -> Result<ListMetricsOutput, String> {
    let limit = limit.unwrap_or(100) as usize;
    let metadata = METRIC_METADATA.read();
    
    let metrics: Vec<MetricInfo> = metadata
        .iter()
        .filter(|(name, _)| {
            prefix.map_or(true, |p| name.starts_with(p))
        })
        .take(limit)
        .map(|(name, meta)| {
            MetricInfo {
                name: name.clone(),
                metric_type: meta.metric_type.clone(),
                tags: meta.label_names.clone(),
            }
        })
        .collect();
    
    let total = metrics.len() as i32;
    
    Ok(ListMetricsOutput { metrics, total })
}

/// Delete Metric Data.
pub async fn delete_metric(
    name: &str,
    before: Option<&str>,
    tags: Option<HashMap<String, String>>,
) -> Result<DeleteMetricOutput, String> {
    let tags = normalize_tags(tags);
    let key = metric_key(name, &tags);
    let mut deleted_count = 0;
    
    let cutoff = before
        .and_then(|b| {
            DateTime::parse_from_rfc3339(b)
                .map(|dt| dt.with_timezone(&Utc).timestamp_millis())
                .ok()
        });
    
    // Delete from metric values storage
    if let Some(mut entry) = METRIC_VALUES.get_mut(&key) {
        let original_len = entry.len();
        if let Some(cutoff_ts) = cutoff {
            entry.retain(|v| v.timestamp >= cutoff_ts);
        } else {
            entry.clear();
        }
        deleted_count += (original_len - entry.len()) as i32;
        
        if entry.is_empty() {
            drop(entry);
            METRIC_VALUES.remove(&key);
        }
    }
    
    // Note: Prometheus metrics cannot be unregistered once registered,
    // but we can reset them. For counters, we can't reset, but gauges
    // and histograms can be observed with new values.
    
    // Remove metadata if all data is deleted and no cutoff
    if cutoff.is_none() && tags.is_empty() {
        METRIC_METADATA.write().remove(name);
    }
    
    Ok(DeleteMetricOutput { deleted_count })
}

/// Export metrics in Prometheus text format.
/// This is useful for exposing metrics to a Prometheus scraper.
pub fn export_prometheus_metrics() -> Result<String, String> {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    
    encoder.encode(&metric_families, &mut buffer)
        .map_err(|e| format!("Failed to encode metrics: {}", e))?;
    
    String::from_utf8(buffer)
        .map_err(|e| format!("Failed to convert metrics to string: {}", e))
}

/// Get the global Prometheus registry.
/// Useful for integrating with HTTP servers to expose /metrics endpoint.
pub fn get_registry() -> &'static Registry {
    &REGISTRY
}

#[cfg(test)]
mod tests;
