// runtime/telemetry/src/snapshot.rs

use crate::metric::TelemetryMetric;

use serde::{Deserialize, Serialize};

use std::time::{SystemTime, UNIX_EPOCH};

/// Represents point-in-time orchestration telemetry.
///
/// A snapshot captures:
/// - node runtime state
/// - orchestration observability
/// - thermal metrics
/// - scheduler/runtime health
///
/// This becomes the atomic observability frame
/// used throughout the telemetry subsystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySnapshot {
    /// Source node identifier
    pub node_id: String,

    /// Snapshot timestamp
    pub timestamp: u64,

    /// Collected telemetry metrics
    pub metrics: Vec<TelemetryMetric>,
}

impl TelemetrySnapshot {
    /// Creates empty telemetry snapshot.
    pub fn new(node_id: String) -> Self {
        Self {
            node_id,
            timestamp: current_timestamp(),
            metrics: Vec::new(),
        }
    }

    /// Adds telemetry metric into snapshot.
    pub fn add_metric(&mut self, metric: TelemetryMetric) {
        self.metrics.push(metric);
    }

    /// Retrieves metric by name.
    pub fn metric(&self, name: &str) -> Option<&TelemetryMetric> {
        self.metrics.iter().find(|metric| metric.name == name)
    }

    /// Returns total metric count.
    pub fn metric_count(&self) -> usize {
        self.metrics.len()
    }

    /// Returns true if snapshot
    /// contains no metrics.
    pub fn is_empty(&self) -> bool {
        self.metrics.is_empty()
    }

    /// Returns all metrics matching category.
    pub fn metrics_by_category(
        &self,
        category: crate::metric::MetricCategory,
    ) -> Vec<&TelemetryMetric> {
        self.metrics
            .iter()
            .filter(|metric| metric.category == category)
            .collect()
    }

    /// Removes metric by name.
    pub fn remove_metric(&mut self, name: &str) {
        self.metrics.retain(|metric| metric.name != name);
    }

    /// Clears all metrics.
    pub fn clear(&mut self) {
        self.metrics.clear();
    }

    /// Returns formatted snapshot summary.
    pub fn summary(&self) -> String {
        let mut output = String::new();

        output.push_str("========================================\n");

        output.push_str("TELEMETRY SNAPSHOT\n");

        output.push_str("========================================\n\n");

        output.push_str(&format!("Node ID      : {}\n", self.node_id));

        output.push_str(&format!("Timestamp    : {}\n", self.timestamp));

        output.push_str(&format!("Metric Count : {}\n\n", self.metric_count()));

        output.push_str("Metrics\n");

        output.push_str("----------------------------------------\n");

        for metric in &self.metrics {
            output.push_str(&format!(
                "{} = {} {}\n",
                metric.name, metric.value, metric.unit,
            ));
        }

        output.push('\n');

        output
    }
}

/// Returns current UNIX timestamp.
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
