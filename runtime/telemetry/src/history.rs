// runtime/telemetry/src/history.rs

use crate::snapshot::TelemetrySnapshot;

use std::collections::{HashMap, VecDeque};

use tracing::{info, warn};

/// Maximum telemetry history retained per node.
///
/// Prevents unbounded memory growth.
const DEFAULT_HISTORY_LIMIT: usize = 128;

/// Maintains rolling telemetry history
/// for distributed orchestration analysis.
///
/// Responsibilities:
/// - telemetry timeline retention
/// - historical observability
/// - trend analysis support
/// - orchestration memory
///
/// This becomes the temporal memory layer
/// of the AetherFabric runtime.
#[derive(Debug)]
pub struct TelemetryHistory {
    /// Per-node telemetry history buffers
    history: HashMap<String, VecDeque<TelemetrySnapshot>>,

    /// Maximum retained snapshots per node
    history_limit: usize,
}

impl TelemetryHistory {
    /// Creates telemetry history store.
    pub fn new() -> Self {
        Self {
            history: HashMap::new(),
            history_limit: DEFAULT_HISTORY_LIMIT,
        }
    }

    /// Creates telemetry history with
    /// custom retention limit.
    pub fn with_limit(limit: usize) -> Self {
        Self {
            history: HashMap::new(),
            history_limit: limit,
        }
    }

    /// Appends telemetry snapshot into history.
    pub fn push_snapshot(&mut self, snapshot: TelemetrySnapshot) {
        let node_id = snapshot.node_id.clone();

        let node_history = self
            .history
            .entry(node_id.clone())
            .or_insert_with(VecDeque::new);

        node_history.push_back(snapshot);

        // Enforce rolling retention window
        while node_history.len() > self.history_limit {
            node_history.pop_front();
        }

        info!("Telemetry snapshot appended [{}]", node_id);
    }

    /// Returns immutable history
    /// for specific node.
    pub fn node_history(&self, node_id: &str) -> Option<&VecDeque<TelemetrySnapshot>> {
        self.history.get(node_id)
    }

    /// Returns latest snapshot for node.
    pub fn latest_snapshot(&self, node_id: &str) -> Option<&TelemetrySnapshot> {
        self.history.get(node_id).and_then(|history| history.back())
    }

    /// Returns oldest snapshot for node.
    pub fn oldest_snapshot(&self, node_id: &str) -> Option<&TelemetrySnapshot> {
        self.history
            .get(node_id)
            .and_then(|history| history.front())
    }

    /// Returns total tracked nodes.
    pub fn tracked_nodes(&self) -> usize {
        self.history.len()
    }

    /// Returns snapshot count for node.
    pub fn snapshot_count(&self, node_id: &str) -> usize {
        self.history
            .get(node_id)
            .map(|history| history.len())
            .unwrap_or(0)
    }

    /// Clears telemetry history for node.
    pub fn clear_node(&mut self, node_id: &str) {
        if self.history.remove(node_id).is_some() {
            warn!("Cleared telemetry history [{}]", node_id);
        }
    }

    /// Clears entire telemetry history.
    pub fn clear_all(&mut self) {
        self.history.clear();

        warn!("Cleared all telemetry history");
    }

    /// Calculates average metric value
    /// across node history window.
    pub fn average_metric(&self, node_id: &str, metric_name: &str) -> Option<f64> {
        let history = self.history.get(node_id)?;

        let mut total = 0.0;
        let mut count = 0;

        for snapshot in history {
            if let Some(metric) = snapshot.metric(metric_name) {
                total += metric.numeric_value().unwrap_or(0.0);
                count += 1;
            }
        }

        if count == 0 {
            return None;
        }

        Some(total / count as f64)
    }

    /// Detects whether metric trend
    /// is increasing over time.
    ///
    /// Useful later for:
    /// - thermal buildup prediction
    /// - workload pressure detection
    /// - orchestration instability analysis
    pub fn metric_rising(&self, node_id: &str, metric_name: &str) -> Option<bool> {
        let history = self.history.get(node_id)?;

        if history.len() < 2 {
            return None;
        }

        let first = history.front()?;

        let last = history.back()?;

        let first_metric = first.metric(metric_name)?;

        let last_metric = last.metric(metric_name)?;

        Some(last_metric.numeric_value().unwrap_or(0.0) > first_metric.numeric_value().unwrap_or(0.0))
    }

    /// Emits telemetry history summary.
    pub fn summary(&self) -> String {
        let mut output = String::new();

        output.push_str("========================================\n");

        output.push_str("TELEMETRY HISTORY SUMMARY\n");

        output.push_str("========================================\n\n");

        output.push_str(&format!("Tracked Nodes : {}\n", self.tracked_nodes()));

        output.push_str(&format!("History Limit : {}\n\n", self.history_limit));

        for (node_id, history) in &self.history {
            output.push_str(&format!("Node [{}]\n", node_id));

            output.push_str(&format!("Snapshots Retained : {}\n", history.len()));

            if let Some(avg_cpu) = self.average_metric(node_id, "cpu_usage") {
                output.push_str(&format!("Average CPU Usage : {:.2}%\n", avg_cpu,));
            }

            if let Some(avg_temp) = self.average_metric(node_id, "cpu_temperature") {
                output.push_str(&format!("Average Temperature : {:.2} C\n", avg_temp,));
            }

            output.push('\n');
        }

        output
    }
}
