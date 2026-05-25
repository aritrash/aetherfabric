// runtime/protocol/src/telemetry.rs

use serde::{Deserialize, Serialize};

use std::time::{SystemTime, UNIX_EPOCH};

/// Represents telemetry category/type.
///
/// Useful for:
/// - orchestration analytics
/// - filtering
/// - telemetry routing
/// - future observability pipelines
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TelemetryCategory {
    Compute,
    Thermal,
    Memory,
    Network,
    Accelerator,
    Power,
}

/// Represents a protocol-level telemetry metric.
///
/// This abstraction exists independently from:
/// - runtime telemetry collectors
/// - hardware drivers
/// - orchestration logic
///
/// It is purely a distributed synchronization model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryMetric {
    /// Metric name
    pub name: String,

    /// Metric category
    pub category: TelemetryCategory,

    /// Metric numeric value
    pub value: f64,

    /// Unit representation
    ///
    /// Examples:
    /// - "%"
    /// - "C"
    /// - "MB"
    /// - "W"
    pub unit: String,
}

impl TelemetryMetric {
    /// Creates a new telemetry metric.
    pub fn new(name: String, category: TelemetryCategory, value: f64, unit: String) -> Self {
        Self {
            name,
            category,
            value,
            unit,
        }
    }
}

/// Represents a distributed telemetry packet.
///
/// Synchronizes:
/// - node observability
/// - orchestration analytics
/// - runtime metrics
/// - thermal state
/// - power telemetry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryPacket {
    /// Protocol version
    pub protocol_version: String,

    /// Source node ID
    pub node_id: String,

    /// Collected telemetry metrics
    pub metrics: Vec<TelemetryMetric>,

    /// Telemetry snapshot timestamp
    pub timestamp: u64,
}

impl TelemetryPacket {
    /// Creates an empty telemetry packet.
    pub fn new(node_id: String) -> Self {
        Self {
            protocol_version: String::from("0.1.0"),

            node_id,

            metrics: Vec::new(),

            timestamp: current_timestamp(),
        }
    }

    /// Adds telemetry metric into packet.
    pub fn add_metric(&mut self, metric: TelemetryMetric) {
        self.metrics.push(metric);
    }

    /// Serializes telemetry packet into JSON.
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    /// Deserializes telemetry packet from JSON.
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }

    /// Returns formatted telemetry summary.
    pub fn summary(&self) -> String {
        let mut output = String::new();

        output.push_str("========================================\n");

        output.push_str("TELEMETRY PACKET\n");

        output.push_str("========================================\n\n");

        output.push_str(&format!("Protocol Version : {}\n", self.protocol_version));

        output.push_str(&format!("Node ID           : {}\n", self.node_id));

        output.push_str(&format!("Timestamp         : {}\n\n", self.timestamp));

        output.push_str("Metrics\n");
        output.push_str("----------------------------------------\n");

        for metric in &self.metrics {
            output.push_str(&format!(
                "{} [{}] = {:.2} {}\n",
                metric.name,
                format!("{:?}", metric.category),
                metric.value,
                metric.unit,
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
