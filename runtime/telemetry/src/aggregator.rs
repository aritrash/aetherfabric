// runtime/telemetry/src/aggregator.rs

use crate::metric::TelemetryMetric;
use crate::snapshot::TelemetrySnapshot;

use std::collections::HashMap;

use tracing::{info, warn};

/// Responsible for aggregating telemetry across:
/// - nodes
/// - orchestration runtime
/// - thermal infrastructure
/// - scheduler state
///
/// The aggregator becomes the centralized
/// observability engine of AetherFabric.
#[derive(Debug)]
pub struct TelemetryAggregator {
    /// Latest snapshot for each node
    snapshots: HashMap<String, TelemetrySnapshot>,
}

impl TelemetryAggregator {
    /// Creates empty telemetry aggregator.
    pub fn new() -> Self {
        Self {
            snapshots: HashMap::new(),
        }
    }

    /// Inserts or updates node snapshot.
    pub fn update_snapshot(&mut self, snapshot: TelemetrySnapshot) {
        info!("Updating telemetry snapshot [{}]", snapshot.node_id);

        self.snapshots.insert(snapshot.node_id.clone(), snapshot);
    }

    /// Returns snapshot for node.
    pub fn get_snapshot(&self, node_id: &str) -> Option<&TelemetrySnapshot> {
        self.snapshots.get(node_id)
    }

    /// Returns all snapshots.
    pub fn all_snapshots(&self) -> Vec<&TelemetrySnapshot> {
        self.snapshots.values().collect()
    }

    /// Returns total tracked nodes.
    pub fn total_nodes(&self) -> usize {
        self.snapshots.len()
    }

    /// Calculates average CPU usage
    /// across entire fabric.
    pub fn average_cpu_usage(&self) -> f64 {
        let mut total = 0.0;
        let mut count = 0;

        for snapshot in self.snapshots.values() {
            if let Some(metric) = snapshot.metric("cpu_usage") {
                total += metric.value;
                count += 1;
            }
        }

        if count == 0 {
            return 0.0;
        }

        total / count as f64
    }

    /// Calculates average fabric temperature.
    pub fn average_temperature(&self) -> f64 {
        let mut total = 0.0;
        let mut count = 0;

        for snapshot in self.snapshots.values() {
            if let Some(metric) = snapshot.metric("cpu_temperature") {
                total += metric.value;
                count += 1;
            }
        }

        if count == 0 {
            return 0.0;
        }

        total / count as f64
    }

    /// Returns hottest node in fabric.
    pub fn hottest_node(&self) -> Option<(&String, f64)> {
        self.snapshots
            .iter()
            .filter_map(|(node_id, snapshot)| {
                snapshot
                    .metric("cpu_temperature")
                    .map(|metric| (node_id, metric.value))
            })
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    /// Returns most heavily loaded node.
    pub fn busiest_node(&self) -> Option<(&String, f64)> {
        self.snapshots
            .iter()
            .filter_map(|(node_id, snapshot)| {
                snapshot
                    .metric("cpu_usage")
                    .map(|metric| (node_id, metric.value))
            })
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    /// Emits fabric-wide telemetry summary.
    pub fn summary(&self) -> String {
        let mut output = String::new();

        output.push_str("========================================\n");

        output.push_str("FABRIC TELEMETRY SUMMARY\n");

        output.push_str("========================================\n\n");

        output.push_str(&format!("Tracked Nodes      : {}\n", self.total_nodes()));

        output.push_str(&format!(
            "Average CPU Usage  : {:.2}%\n",
            self.average_cpu_usage()
        ));

        output.push_str(&format!(
            "Average Temperature: {:.2} C\n",
            self.average_temperature()
        ));

        if let Some((node, temp)) = self.hottest_node() {
            output.push_str(&format!("Hottest Node       : {} ({:.2} C)\n", node, temp,));
        }

        if let Some((node, cpu)) = self.busiest_node() {
            output.push_str(&format!("Busiest Node       : {} ({:.2}%)\n", node, cpu,));
        }

        output.push('\n');

        output
    }
}
