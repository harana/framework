use std::collections::HashMap;

use crate::error::Result;

#[derive(Debug, Clone)]
pub enum MetricValue {
    Counter(u64),
    Gauge(f64),
    Histogram(Vec<f64>),
    Summary { sum: f64, count: u64 },
}

pub type Labels = HashMap<String, String>;

pub trait Metrics: Send + Sync {
    fn provider(&self) -> &str;
    fn increment(&self, name: &str, value: u64, labels: Option<&Labels>);
    fn decrement(&self, name: &str, value: u64, labels: Option<&Labels>);
    fn gauge(&self, name: &str, value: f64, labels: Option<&Labels>);
    fn histogram(&self, name: &str, value: f64, labels: Option<&Labels>);
    fn timing(&self, name: &str, duration_ms: u64, labels: Option<&Labels>);
    fn get(&self, name: &str) -> Option<MetricValue>;
    fn export(&self) -> Result<String>;
    fn reset(&mut self);
    fn register(&mut self, name: &str, description: &str, metric_type: &str) -> Result<()>;
}
