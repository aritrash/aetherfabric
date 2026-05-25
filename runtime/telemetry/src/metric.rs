// runtime/telemetry/src/metric.rs

use serde::{Deserialize, Serialize};

use std::fmt;

/// Represents telemetry metric classification.
///
/// Categories help:
/// - orchestration filtering
/// - analytics
/// - runtime observability
/// - future telemetry routing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MetricCategory {
    Compute,
    Thermal,
    Memory,
    Network,
    Scheduler,
    Accelerator,
    Power,
    Fabric,
}

/// Represents telemetry value type.
///
/// Allows future support for:
/// - integer metrics
/// - floating-point metrics
/// - boolean states
/// - textual runtime metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Text(String),
}

impl fmt::Display for MetricValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetricValue::Integer(v) => {
                write!(f, "{v}")
            }

            MetricValue::Float(v) => {
                write!(f, "{:.2}", v)
            }

            MetricValue::Boolean(v) => {
                write!(f, "{v}")
            }

            MetricValue::Text(v) => {
                write!(f, "{v}")
            }
        }
    }
}

/// Represents a runtime telemetry metric.
///
/// Metrics are the atomic observability units
/// inside the AetherFabric telemetry runtime.
///
/// Examples:
/// - CPU usage
/// - thermal pressure
/// - scheduler queue depth
/// - node power draw
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryMetric {
    /// Metric identifier
    pub name: String,

    /// Metric classification
    pub category: MetricCategory,

    /// Metric value
    pub value: MetricValue,

    /// Human-readable metric unit
    ///
    /// Examples:
    /// - "%"
    /// - "C"
    /// - "MB"
    /// - "W"
    pub unit: String,

    /// Metric collection timestamp
    pub timestamp: u64,
}

impl TelemetryMetric {
    /// Creates floating-point metric.
    pub fn float(name: String, category: MetricCategory, value: f64, unit: String) -> Self {
        Self {
            name,
            category,
            value: MetricValue::Float(value),
            unit,
            timestamp: current_timestamp(),
        }
    }

    /// Creates integer metric.
    pub fn integer(name: String, category: MetricCategory, value: i64, unit: String) -> Self {
        Self {
            name,
            category,
            value: MetricValue::Integer(value),
            unit,
            timestamp: current_timestamp(),
        }
    }

    /// Creates boolean metric.
    pub fn boolean(name: String, category: MetricCategory, value: bool, unit: String) -> Self {
        Self {
            name,
            category,
            value: MetricValue::Boolean(value),
            unit,
            timestamp: current_timestamp(),
        }
    }

    /// Creates textual metric.
    pub fn text(name: String, category: MetricCategory, value: String, unit: String) -> Self {
        Self {
            name,
            category,
            value: MetricValue::Text(value),
            unit,
            timestamp: current_timestamp(),
        }
    }

    /// Returns numeric value if metric
    /// contains integer/float data.
    pub fn numeric_value(&self) -> Option<f64> {
        match self.value {
            MetricValue::Integer(v) => Some(v as f64),

            MetricValue::Float(v) => Some(v),

            _ => None,
        }
    }

    /// Returns formatted metric summary.
    pub fn summary(&self) -> String {
        format!(
            "\
Telemetry Metric
-------------------------
Name      : {}
Category  : {:?}
Value     : {}
Unit      : {}
Timestamp : {}
",
            self.name, self.category, self.value, self.unit, self.timestamp,
        )
    }
}

/// Returns current UNIX timestamp.
fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
