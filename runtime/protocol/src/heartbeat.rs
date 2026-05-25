// runtime/protocol/src/heartbeat.rs

use serde::{Deserialize, Serialize};

use std::time::{SystemTime, UNIX_EPOCH};

/// Represents runtime node state.
///
/// This enum is intentionally protocol-level
/// and independent from orchestration logic.
///
/// It should remain lightweight and serializable.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeStatus {
    Initializing,
    Active,
    Busy,
    Overloaded,
    ThermalCritical,
    Offline,
}

/// Represents protocol-level telemetry snapshot.
///
/// This structure exists independently from
/// runtime telemetry collectors.
///
/// It is purely used for:
/// - synchronization
/// - serialization
/// - distributed transport
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryPacket {
    /// CPU usage percentage
    pub cpu_usage: f32,

    /// CPU temperature in Celsius
    pub cpu_temperature_c: f32,

    /// Available memory in MB
    pub available_memory_mb: u64,

    /// Memory usage in MB
    pub memory_usage_mb: u64,
}

/// Distributed runtime heartbeat packet.
///
/// Heartbeats synchronize:
/// - node telemetry
/// - runtime state
/// - orchestration liveness
/// - leadership awareness
///
/// This is one of the most critical
/// protocol packets in AetherFabric.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatPacket {
    /// Protocol version
    pub protocol_version: String,

    /// Source node identifier
    pub node_id: String,

    /// Current node runtime status
    pub status: NodeStatus,

    /// Current telemetry snapshot
    pub telemetry: TelemetryPacket,

    /// Whether node currently acts
    /// as orchestration leader
    pub is_leader: bool,

    /// Heartbeat timestamp
    pub timestamp: u64,
}

impl HeartbeatPacket {
    /// Creates a new heartbeat packet.
    pub fn new(
        node_id: String,
        status: NodeStatus,
        telemetry: TelemetryPacket,
        is_leader: bool,
    ) -> Self {
        Self {
            protocol_version: String::from("0.1.0"),

            node_id,
            status,
            telemetry,
            is_leader,

            timestamp: current_timestamp(),
        }
    }

    /// Serializes heartbeat into JSON.
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    /// Deserializes heartbeat from JSON.
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }

    /// Returns formatted heartbeat summary.
    pub fn summary(&self) -> String {
        format!(
            "\
========================================
HEARTBEAT PACKET
========================================

Protocol Version : {}
Node ID           : {}
Status            : {:?}
Leader            : {}

CPU Usage         : {:.2}%
CPU Temperature   : {:.2} °C

Available Memory  : {} MB
Memory Usage      : {} MB

Timestamp         : {}

",
            self.protocol_version,
            self.node_id,
            self.status,
            self.is_leader,
            self.telemetry.cpu_usage,
            self.telemetry.cpu_temperature_c,
            self.telemetry.available_memory_mb,
            self.telemetry.memory_usage_mb,
            self.timestamp,
        )
    }
}

/// Returns current UNIX timestamp.
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
