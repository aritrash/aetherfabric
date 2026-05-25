// runtime/telemetry/src/analyzer.rs

use crate::aggregator::TelemetryAggregator;
use crate::snapshot::TelemetrySnapshot;

use tracing::{info, warn};

/// Represents orchestration anomaly severity.
#[derive(Debug, Clone)]
pub enum AnomalySeverity {
    Low,
    Moderate,
    High,
    Critical,
}

/// Represents a telemetry-derived orchestration anomaly.
#[derive(Debug, Clone)]
pub struct TelemetryAnomaly {
    /// Node associated with anomaly
    pub node_id: String,

    /// Human-readable anomaly description
    pub description: String,

    /// Severity classification
    pub severity: AnomalySeverity,
}

impl TelemetryAnomaly {
    /// Returns formatted anomaly summary.
    pub fn summary(&self) -> String {
        format!(
            "\
Telemetry Anomaly
-------------------------
Node ID    : {}
Severity   : {:?}
Description: {}
",
            self.node_id, self.severity, self.description,
        )
    }
}

/// Telemetry intelligence and analysis engine.
///
/// Responsible for:
/// - overload detection
/// - thermal pressure analysis
/// - orchestration anomaly detection
/// - telemetry-derived orchestration intelligence
///
/// Future responsibilities:
/// - predictive migration
/// - trend analysis
/// - adaptive orchestration analytics
pub struct TelemetryAnalyzer;

impl TelemetryAnalyzer {
    /// Detects runtime anomalies across fabric.
    pub fn analyze_fabric(aggregator: &TelemetryAggregator) -> Vec<TelemetryAnomaly> {
        let mut anomalies = Vec::new();

        for snapshot in aggregator.all_snapshots() {
            anomalies.extend(Self::analyze_snapshot(snapshot));
        }

        anomalies
    }

    /// Analyzes single node snapshot.
    fn analyze_snapshot(snapshot: &TelemetrySnapshot) -> Vec<TelemetryAnomaly> {
        let mut anomalies = Vec::new();

        // -------------------------------------------------
        // CPU Usage Analysis
        // -------------------------------------------------

        if let Some(cpu) = snapshot.metric("cpu_usage") {
            if cpu.value >= 95.0 {
                anomalies.push(TelemetryAnomaly {
                    node_id: snapshot.node_id.clone(),

                    description: String::from("Critical CPU saturation detected"),

                    severity: AnomalySeverity::Critical,
                });
            } else if cpu.value >= 80.0 {
                anomalies.push(TelemetryAnomaly {
                    node_id: snapshot.node_id.clone(),

                    description: String::from("High compute pressure detected"),

                    severity: AnomalySeverity::High,
                });
            }
        }

        // -------------------------------------------------
        // Thermal Analysis
        // -------------------------------------------------

        if let Some(temp) = snapshot.metric("cpu_temperature") {
            if temp.value >= 85.0 {
                anomalies.push(TelemetryAnomaly {
                    node_id: snapshot.node_id.clone(),

                    description: String::from("Critical thermal threshold exceeded"),

                    severity: AnomalySeverity::Critical,
                });
            } else if temp.value >= 70.0 {
                anomalies.push(TelemetryAnomaly {
                    node_id: snapshot.node_id.clone(),

                    description: String::from("Thermal pressure increasing"),

                    severity: AnomalySeverity::High,
                });
            }
        }

        // -------------------------------------------------
        // Memory Pressure Analysis
        // -------------------------------------------------

        if let Some(memory) = snapshot.metric("available_memory") {
            if memory.value <= 256.0 {
                anomalies.push(TelemetryAnomaly {
                    node_id: snapshot.node_id.clone(),

                    description: String::from("Critical memory pressure detected"),

                    severity: AnomalySeverity::Critical,
                });
            } else if memory.value <= 512.0 {
                anomalies.push(TelemetryAnomaly {
                    node_id: snapshot.node_id.clone(),

                    description: String::from("Low available memory"),

                    severity: AnomalySeverity::Moderate,
                });
            }
        }

        anomalies
    }

    /// Emits orchestration telemetry analysis summary.
    pub fn report(anomalies: &[TelemetryAnomaly]) {
        info!("========================================");

        info!("AETHERFABRIC TELEMETRY ANALYSIS");

        info!("========================================");

        if anomalies.is_empty() {
            info!("No orchestration anomalies detected");

            return;
        }

        for anomaly in anomalies {
            warn!("\n{}", anomaly.summary());
        }
    }
}
